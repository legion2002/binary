//! Market resolution through an AI council (Witan).
//!
//! A council of AI models decides the market question and signs a decision record
//! (RFC 8785 canonical JSON). The backend opens the query with the council CLI, verifies the
//! record against pinned signers, and submits it to the CouncilOracle contract, which checks
//! the signature, the council config hash and the salted question hash on chain.
//!
//! Names stay generic ("council") so the council service can be swapped or renamed; the CLI
//! is the Witan CLI (`witan`).

use alloy::primitives::{keccak256, Address, Signature, B256};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::time::Duration;
use tokio::process::Command;

use crate::db::Market;

alloy::sol! {
    /// Minimal binding for contracts/src/CouncilOracle.sol.
    #[sol(rpc, all_derives)]
    interface CouncilOracle {
        error MarketNotSet();
        error NotResolver();
        error ZeroHash();
        error MarketAlreadyRegistered();
        error MarketNotRegistered();
        error MarketAlreadyResolved();
        error UntrustedSigner(address recovered);
        error MalformedRecord(string field);
        error UnsupportedRecordVersion();
        error SignerMismatch();
        error CouncilMismatch();
        error EvidenceRootMismatch();
        error OutcomeNotBinary();

        function registerMarket(bytes32 marketHash, bytes32 councilHash, bytes32 questionHash) external;
        function resolve(bytes32 marketHash, bytes recordJcs, bytes commitmentsJcs, bytes signature) external;
        function markets(bytes32 marketHash) external view returns (bytes32 councilHash, bytes32 questionHash, bool registered, bool resolved, bool isYes);
        function getResolution(bytes32 marketHash) external view returns (bool);
        function trustedSigners(address signer) external view returns (bool);
        function resolvers(address resolver) external view returns (bool);
    }
}

/// Default time a council query may take (the council itself times out model calls).
const DEFAULT_TIMEOUT_SECS: u64 = 300;

// ============================================================================
// Settings
// ============================================================================

/// How the backend reaches the council. Built from COUNCIL_* environment variables.
/// Holds the payment key: deliberately not Debug.
#[derive(Clone)]
pub struct CouncilSettings {
    /// CouncilOracle contract address.
    pub oracle_address: Address,
    /// Council config hash every market is pinned to (COUNCIL_HASH).
    pub council_hash: B256,
    /// Council CLI binary (COUNCIL_CLI, default "witan").
    pub cli: String,
    /// Council API base URL (COUNCIL_URL, else WITAN_URL); passed to the CLI as --url.
    pub url: Option<String>,
    /// Trusted record signers (COUNCIL_SIGNERS, comma-separated). Never empty.
    pub signers: Vec<Address>,
    /// Tempo private key that pays for queries over MPP (COUNCIL_PRIVATE_KEY).
    pub private_key: Option<String>,
    /// Tempo network the CLI pays on (COUNCIL_NETWORK: moderato or tempo).
    pub network: Option<String>,
    /// Most to pay per query in USD (COUNCIL_MAX_PRICE); the council's quote otherwise.
    pub max_price: Option<String>,
    /// Where query files are kept (COUNCIL_QUERY_DIR, default ./council-queries).
    /// A query file is the only copy of the question and evidence the council saw.
    pub query_dir: PathBuf,
    /// Accept "test"/"dev" council attestations (COUNCIL_ALLOW_DEV=1): local councils only.
    pub allow_dev: bool,
    /// Longest a CLI call may run (COUNCIL_TIMEOUT_SECS).
    pub timeout: Duration,
}

impl CouncilSettings {
    /// Reads COUNCIL_* settings for the CouncilOracle at oracle_address.
    pub fn from_env(oracle_address: Address) -> anyhow::Result<Self> {
        let var = |k: &str| {
            std::env::var(k)
                .ok()
                .map(|v| v.trim().to_string())
                .filter(|v| !v.is_empty())
        };
        let council_hash = var("COUNCIL_HASH").ok_or_else(|| {
            anyhow::anyhow!("COUNCIL_HASH must be set when MARKET_ORACLE=council")
        })?;
        let council_hash = parse_hash(&council_hash)
            .ok_or_else(|| anyhow::anyhow!("COUNCIL_HASH must be 0x + 64 hex characters"))?;
        let signers = parse_signers(&var("COUNCIL_SIGNERS").unwrap_or_default())?;
        if signers.is_empty() {
            anyhow::bail!("COUNCIL_SIGNERS must list the council's trusted signer address(es)");
        }
        let timeout = var("COUNCIL_TIMEOUT_SECS")
            .map(|s| s.parse::<u64>())
            .transpose()?
            .unwrap_or(DEFAULT_TIMEOUT_SECS);
        Ok(Self {
            oracle_address,
            council_hash,
            cli: var("COUNCIL_CLI").unwrap_or_else(|| "witan".to_string()),
            url: var("COUNCIL_URL").or_else(|| var("WITAN_URL")),
            signers,
            private_key: var("COUNCIL_PRIVATE_KEY"),
            network: var("COUNCIL_NETWORK"),
            max_price: var("COUNCIL_MAX_PRICE"),
            query_dir: PathBuf::from(
                var("COUNCIL_QUERY_DIR").unwrap_or_else(|| "council-queries".into()),
            ),
            allow_dev: matches!(var("COUNCIL_ALLOW_DEV").as_deref(), Some("1" | "true")),
            timeout: Duration::from_secs(timeout),
        })
    }
}

/// Parses a comma-separated list of addresses.
pub fn parse_signers(list: &str) -> anyhow::Result<Vec<Address>> {
    list.split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.parse::<Address>()
                .map_err(|_| anyhow::anyhow!("invalid signer address {s:?}"))
        })
        .collect()
}

/// Parses "0x" + 64 hex characters.
pub fn parse_hash(s: &str) -> Option<B256> {
    let body = s.strip_prefix("0x")?;
    if body.len() != 64 {
        return None;
    }
    hex::decode(body).ok().map(|b| B256::from_slice(&b))
}

// ============================================================================
// Hashes, exactly as the council computes them
// ============================================================================

/// "0x" + 64 lowercase hex: how records write hashes.
pub fn hex32(h: B256) -> String {
    format!("0x{}", hex::encode(h))
}

fn sha256(bytes: &[u8]) -> B256 {
    B256::from_slice(&Sha256::digest(bytes))
}

/// RFC 8785 (JCS) bytes of a JSON value.
pub fn jcs(value: &Value) -> Vec<u8> {
    serde_json_canonicalizer::to_vec(value).expect("JSON values always canonicalize")
}

/// A fresh question salt: 16 random bytes as "0x" + 32 lowercase hex.
pub fn new_question_salt() -> String {
    let mut salt = [0u8; 16];
    rand::rngs::OsRng.fill_bytes(&mut salt);
    format!("0x{}", hex::encode(salt))
}

/// Whether s is a question salt: "0x" + 32 lowercase hex.
pub fn is_salt(s: &str) -> bool {
    s.strip_prefix("0x").is_some_and(|h| {
        h.len() == 32
            && h.bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    })
}

/// question_hash = sha256(JCS({"question": question, "salt": salt})).
/// Registered on chain at market creation; the council puts it in the evidence root.
pub fn question_hash(question: &str, salt: &str) -> B256 {
    sha256(&jcs(&json!({"question": question, "salt": salt})))
}

/// evidence_root = sha256(JCS({"question_hash": .., "commitments": [{"party","hash"}..]})).
pub fn evidence_root(question_hash: B256, commitments: &Value) -> B256 {
    sha256(&jcs(
        &json!({"question_hash": hex32(question_hash), "commitments": commitments}),
    ))
}

/// Party names the council accepts: [a-z0-9_-]{1,32}.
pub fn is_party(name: &str) -> bool {
    (1..=32).contains(&name.len())
        && name
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_' || b == b'-')
}

// ============================================================================
// Decision records
// ============================================================================

/// A decided outcome as far as a binary market cares.
#[derive(Debug, Clone, PartialEq)]
pub enum Outcome {
    Yes,
    No,
    /// Anything else (an invalid decision, another outcome type): the market stays open.
    Other(Value),
}

impl Outcome {
    pub fn from_record(record: &Value) -> Self {
        let o = &record["outcome"];
        match (o["type"].as_str(), o["value"].as_str()) {
            (Some("binary"), Some("yes")) => Outcome::Yes,
            (Some("binary"), Some("no")) => Outcome::No,
            _ => Outcome::Other(o.clone()),
        }
    }
}

/// What CouncilOracle.resolve takes, derived from a signed record.
#[derive(Debug, Clone)]
pub struct CouncilProof {
    /// JCS of the record with proof.signature removed: the signed bytes.
    pub record_jcs: Vec<u8>,
    /// JCS of the record's commitments array.
    pub commitments_jcs: Vec<u8>,
    /// 65 bytes r||s||v.
    pub signature: Vec<u8>,
    /// sha256(record_jcs).
    pub digest: B256,
    /// Address the signature recovers to.
    pub signer: Address,
    pub query_id: String,
    pub outcome: Outcome,
}

impl CouncilProof {
    /// Builds the proof from a full record (with proof.signature).
    pub fn from_record(record: &Value) -> anyhow::Result<Self> {
        let sig_hex = record["proof"]["signature"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("record has no proof.signature"))?;
        let signature = hex::decode(sig_hex.strip_prefix("0x").unwrap_or(sig_hex))?;
        if signature.len() != 65 {
            anyhow::bail!("proof.signature must be 65 bytes");
        }
        let mut unsigned = record.clone();
        unsigned["proof"]
            .as_object_mut()
            .ok_or_else(|| anyhow::anyhow!("record has no proof"))?
            .remove("signature");
        let record_jcs = jcs(&unsigned);
        let commitments = &record["commitments"];
        if !commitments.is_array() {
            anyhow::bail!("record has no commitments array");
        }
        let digest = sha256(&record_jcs);
        // EIP-191 over the 32 digest bytes, v = 27/28.
        let signer =
            Signature::from_raw(&signature)?.recover_address_from_msg(digest.as_slice())?;
        Ok(Self {
            commitments_jcs: jcs(commitments),
            record_jcs,
            signature,
            digest,
            signer,
            query_id: record["query_id"]
                .as_str()
                .ok_or_else(|| anyhow::anyhow!("record has no query_id"))?
                .to_string(),
            outcome: Outcome::from_record(record),
        })
    }

    /// keccak256 of the query id, as the ResolutionSet event indexes it.
    pub fn query_id_hash(&self) -> B256 {
        keccak256(self.query_id.as_bytes())
    }
}

/// The checks CouncilOracle.resolve makes, run before paying gas for them.
pub fn check_record(
    record: &Value,
    proof: &CouncilProof,
    council_hash: B256,
    question_hash: B256,
    signers: &[Address],
) -> Result<(), String> {
    if record["v"] != json!(1) {
        return Err("unsupported record version".into());
    }
    if !signers.contains(&proof.signer) {
        return Err(format!(
            "record signed by untrusted signer {:?}",
            proof.signer
        ));
    }
    if record["proof"]["signer"].as_str()
        != Some(format!("{:?}", proof.signer).to_lowercase().as_str())
    {
        return Err("proof.signer does not match the signature".into());
    }
    if record["council_hash"].as_str() != Some(hex32(council_hash).as_str()) {
        return Err("record was decided by another council".into());
    }
    let root = evidence_root(question_hash, &record["commitments"]);
    if record["evidence_root"].as_str() != Some(hex32(root).as_str()) {
        return Err("record's evidence root does not commit to the market's question".into());
    }
    Ok(())
}

// ============================================================================
// Council CLI
// ============================================================================

/// One party's evidence for the council.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Evidence {
    pub party: String,
    pub text: String,
}

/// The parts of `query open --json` output the backend uses.
#[derive(Debug, Clone, Deserialize)]
pub struct OpenedQuery {
    pub query_id: String,
    pub record: Value,
    pub verification: Verification,
}

/// A verification report (`query verify --json`, or verification in open's output).
#[derive(Debug, Clone, Deserialize)]
pub struct Verification {
    pub ok: bool,
    #[serde(default)]
    pub unpinned: bool,
}

impl Verification {
    fn pinned_ok(&self) -> bool {
        self.ok && !self.unpinned
    }
}

fn global_args(s: &CouncilSettings) -> Vec<String> {
    let mut args = vec!["--json".to_string()];
    if let Some(url) = &s.url {
        args.push(format!("--url={url}"));
    }
    for signer in &s.signers {
        args.push(format!("--signer={}", format!("{signer:?}").to_lowercase()));
    }
    if s.allow_dev {
        args.push("--allow-dev".into());
    }
    args
}

/// Arguments for `query open`. Evidence goes in files (name=@path), never inline, so
/// evidence text is never parsed as a CLI option or file reference.
pub fn open_args(
    s: &CouncilSettings,
    question: &str,
    salt: &str,
    party_files: &[(String, PathBuf)],
    out: &Path,
) -> Vec<String> {
    let mut args = global_args(s);
    args.extend([
        "query".to_string(),
        "open".to_string(),
        format!("--council={}", hex32(s.council_hash)),
        format!("--question={question}"),
        format!("--question-salt={salt}"),
    ]);
    for (party, path) in party_files {
        args.push(format!("--party={party}=@{}", path.display()));
    }
    args.push(format!("--out={}", out.display()));
    if let Some(max) = &s.max_price {
        args.push(format!("--max-price={max}"));
    }
    args
}

/// Arguments for `query verify`.
pub fn verify_args(s: &CouncilSettings, query_id: &str) -> Vec<String> {
    let mut args = global_args(s);
    args.extend([
        "query".to_string(),
        "verify".to_string(),
        query_id.to_string(),
    ]);
    args
}

/// Runs the CLI; returns its exit code and stdout as JSON. Exit 2 means it ran but a
/// verification failed, and still prints JSON.
async fn run_cli(s: &CouncilSettings, args: &[String]) -> anyhow::Result<(i32, Value)> {
    let mut cmd = Command::new(&s.cli);
    cmd.args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true)
        // Only the signers given as flags count.
        .env_remove("WITAN_SIGNERS");
    if let Some(key) = &s.private_key {
        cmd.env("WITAN_PRIVATE_KEY", key);
    }
    if let Some(network) = &s.network {
        cmd.env("WITAN_NETWORK", network);
    }
    let out = tokio::time::timeout(s.timeout, cmd.output())
        .await
        .map_err(|_| anyhow::anyhow!("council CLI timed out after {:?}", s.timeout))?
        .map_err(|e| anyhow::anyhow!("could not run council CLI {:?}: {e}", s.cli))?;
    let code = out.status.code().unwrap_or(-1);
    if code != 0 && code != 2 {
        let stderr = String::from_utf8_lossy(&out.stderr);
        let msg: String = stderr.trim().chars().take(500).collect();
        anyhow::bail!("council CLI failed (exit {code}): {msg}");
    }
    let json = serde_json::from_slice(&out.stdout)
        .map_err(|e| anyhow::anyhow!("council CLI printed no JSON (exit {code}): {e}"))?;
    Ok((code, json))
}

/// Opens a query (paid) and returns the decided record, verified against the pinned signers.
/// Keeps the query file (question, salts, evidence) in the query dir under file_stem.
pub async fn open_query(
    s: &CouncilSettings,
    file_stem: &str,
    question: &str,
    salt: &str,
    evidence: &[Evidence],
) -> anyhow::Result<OpenedQuery> {
    if !is_salt(salt) {
        anyhow::bail!("question salt must be 0x + 32 lowercase hex");
    }
    if evidence.is_empty() {
        anyhow::bail!("evidence needs at least one party");
    }
    for (i, e) in evidence.iter().enumerate() {
        if !is_party(&e.party) {
            anyhow::bail!("party {:?} must match [a-z0-9_-]{{1,32}}", e.party);
        }
        if evidence[..i].iter().any(|o| o.party == e.party) {
            anyhow::bail!("party {:?} is listed twice", e.party);
        }
    }
    tokio::fs::create_dir_all(&s.query_dir).await?;
    let mut files = Vec::with_capacity(evidence.len());
    for e in evidence {
        let path = s.query_dir.join(format!("{file_stem}.{}.txt", e.party));
        tokio::fs::write(&path, &e.text).await?;
        files.push((e.party.clone(), path));
    }
    let out = s.query_dir.join(format!("{file_stem}.json"));
    let result = run_cli(s, &open_args(s, question, salt, &files, &out)).await;
    // The query file now holds the evidence; the inputs are not needed any more.
    for (_, path) in &files {
        let _ = tokio::fs::remove_file(path).await;
    }
    let (_, json) = result?;
    let opened: OpenedQuery = serde_json::from_value(json)
        .map_err(|e| anyhow::anyhow!("unexpected council CLI output: {e}"))?;
    if !opened.verification.pinned_ok() {
        anyhow::bail!(
            "query {} was decided but its record did not verify against the pinned signers",
            opened.query_id
        );
    }
    Ok(opened)
}

/// Verifies a stored record again through the council API, against the pinned signers.
pub async fn verify_query(s: &CouncilSettings, query_id: &str) -> anyhow::Result<()> {
    let (_, json) = run_cli(s, &verify_args(s, query_id)).await?;
    let report: Verification = serde_json::from_value(json)
        .map_err(|e| anyhow::anyhow!("unexpected council CLI output: {e}"))?;
    if !report.pinned_ok() {
        anyhow::bail!("query {query_id} did not verify against the pinned signers");
    }
    Ok(())
}

// ============================================================================
// API
// ============================================================================

/// What anyone needs to check a council market: the council config hash, the question salt
/// and the salted question hash registered on chain, and the query that resolved it.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CouncilInfo {
    pub council_hash: String,
    pub question_salt: String,
    pub question_hash: String,
    pub query_id: Option<String>,
}

impl CouncilInfo {
    pub fn from_market(m: &Market) -> Option<Self> {
        Some(Self {
            council_hash: m.council_hash.clone()?,
            question_salt: m.council_question_salt.clone()?,
            question_hash: m.council_question_hash.clone()?,
            query_id: m.council_query_id.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Records built and signed by Witan's own witan-core code (see tests/fixtures/README.md).
    fn fixture() -> Value {
        serde_json::from_str(include_str!("../tests/fixtures/council_record.json")).unwrap()
    }

    fn b256(v: &Value) -> B256 {
        parse_hash(v.as_str().unwrap()).unwrap()
    }

    fn settings(cli: &str, dir: &Path) -> CouncilSettings {
        let f = fixture();
        CouncilSettings {
            oracle_address: Address::ZERO,
            council_hash: b256(&f["council_hash"]),
            cli: cli.into(),
            url: Some("http://council.test".into()),
            signers: vec![f["signer"].as_str().unwrap().parse().unwrap()],
            private_key: None,
            network: None,
            max_price: Some("0.05".into()),
            query_dir: dir.to_path_buf(),
            allow_dev: false,
            timeout: Duration::from_secs(20),
        }
    }

    #[test]
    fn question_hash_matches_witan() {
        let f = fixture();
        let qh = question_hash(
            f["question"].as_str().unwrap(),
            f["question_salt"].as_str().unwrap(),
        );
        assert_eq!(hex32(qh), f["question_hash"].as_str().unwrap());
        // The question has quotes: JCS escapes them.
        let manual = format!(
            r#"{{"question":"Did \"Team A\" win the final on 2026-11-09? Answer yes or no.","salt":"{}"}}"#,
            f["question_salt"].as_str().unwrap()
        );
        assert_eq!(qh, sha256(manual.as_bytes()));
    }

    #[test]
    fn evidence_root_and_commitments_match_witan() {
        let f = fixture();
        let rec = &f["yes"]["record"];
        let root = evidence_root(b256(&f["question_hash"]), &rec["commitments"]);
        assert_eq!(hex32(root), f["evidence_root"].as_str().unwrap());
        // Each commitment is sha256(JCS({"party","text","salt"})) of a submission.
        for (i, sub) in f["submissions"].as_array().unwrap().iter().enumerate() {
            assert_eq!(
                hex32(sha256(&jcs(sub))),
                rec["commitments"][i]["hash"].as_str().unwrap()
            );
        }
    }

    #[test]
    fn proof_matches_witan_record_bytes() {
        let f = fixture();
        for (name, outcome) in [("yes", Outcome::Yes), ("no", Outcome::No)] {
            let case = &f[name];
            let proof = CouncilProof::from_record(&case["record"]).unwrap();
            assert_eq!(
                String::from_utf8(proof.record_jcs.clone()).unwrap(),
                case["record_jcs"].as_str().unwrap()
            );
            assert_eq!(
                String::from_utf8(proof.commitments_jcs.clone()).unwrap(),
                f["commitments_jcs"].as_str().unwrap()
            );
            assert_eq!(hex32(proof.digest), case["digest"].as_str().unwrap());
            assert_eq!(
                format!("0x{}", hex::encode(&proof.signature)),
                case["signature"].as_str().unwrap()
            );
            assert_eq!(
                proof.signer,
                f["signer"].as_str().unwrap().parse::<Address>().unwrap()
            );
            assert_eq!(proof.query_id, "qry_01JCOUNCILFIXTURE000000000");
            assert_eq!(proof.outcome, outcome);
        }
        let invalid = CouncilProof::from_record(&f["invalid"]["record"]).unwrap();
        assert!(matches!(invalid.outcome, Outcome::Other(_)));
    }

    #[test]
    fn check_record_mirrors_the_contract() {
        let f = fixture();
        let rec = &f["yes"]["record"];
        let proof = CouncilProof::from_record(rec).unwrap();
        let council = b256(&f["council_hash"]);
        let qh = b256(&f["question_hash"]);
        let signers = vec![proof.signer];
        assert_eq!(check_record(rec, &proof, council, qh, &signers), Ok(()));
        assert!(check_record(rec, &proof, B256::repeat_byte(1), qh, &signers).is_err());
        let other_q = question_hash("Another question?", f["question_salt"].as_str().unwrap());
        assert!(check_record(rec, &proof, council, other_q, &signers).is_err());
        assert!(check_record(rec, &proof, council, qh, &[Address::repeat_byte(9)]).is_err());

        // A tampered record recovers to another signer.
        let mut tampered = rec.clone();
        tampered["outcome"]["value"] = json!("no");
        let p = CouncilProof::from_record(&tampered).unwrap();
        assert_ne!(p.signer, proof.signer);
        assert!(check_record(&tampered, &p, council, qh, &signers).is_err());
    }

    #[test]
    fn salts_and_parties() {
        let s = new_question_salt();
        assert!(is_salt(&s));
        assert_ne!(s, new_question_salt());
        assert!(!is_salt("0xABCDEF0123456789abcdef0123456789"));
        assert!(!is_salt("0x00"));
        assert!(is_party("yes") && is_party("team_a-1"));
        assert!(
            !is_party("") && !is_party("Yes") && !is_party("a b") && !is_party(&"a".repeat(33))
        );
        assert_eq!(
            parse_signers(" 0x4fa279c0e172be184228f2802b50858947d4eb74 ,")
                .unwrap()
                .len(),
            1
        );
        assert!(parse_signers("nope").is_err());
    }

    #[test]
    fn cli_args() {
        let s = settings("witan", Path::new("/q"));
        let args = open_args(
            &s,
            "--help?",
            "0x0123456789abcdef0123456789abcdef",
            &[("yes".into(), PathBuf::from("/q/m.yes.txt"))],
            Path::new("/q/m.json"),
        );
        assert_eq!(
            args,
            vec![
                "--json",
                "--url=http://council.test",
                "--signer=0xa32c17ca1b4eda0e70a434bb7c8b4fb216274ff2",
                "query",
                "open",
                "--council=0x7777777777777777777777777777777777777777777777777777777777777777",
                "--question=--help?",
                "--question-salt=0x0123456789abcdef0123456789abcdef",
                "--party=yes=@/q/m.yes.txt",
                "--out=/q/m.json",
                "--max-price=0.05",
            ]
        );
        assert_eq!(&verify_args(&s, "qry_x")[3..], ["query", "verify", "qry_x"]);
    }

    /// Runs open_query and verify_query against a fake CLI that answers like the real one.
    #[tokio::test]
    async fn open_and_verify_through_a_fake_cli() {
        let f = fixture();
        let dir = std::env::temp_dir().join(format!("council-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let opened = json!({
            "query_id": "qry_01JCOUNCILFIXTURE000000000",
            "council_hash": f["council_hash"],
            "record": f["yes"]["record"],
            "reasons": [],
            "receipt": null,
            "payload": {},
            "verification": {"ok": true, "unpinned": false, "checks": [], "model_call_proofs": "not_available"}
        });
        std::fs::write(dir.join("opened.json"), opened.to_string()).unwrap();
        std::fs::write(
            dir.join("verified.json"),
            r#"{"ok":true,"unpinned":false,"checks":[]}"#,
        )
        .unwrap();
        let cli = dir.join("fake-cli.sh");
        std::fs::write(
            &cli,
            format!(
                "#!/bin/sh\necho \"$@\" > {d}/args.txt\nfor a in \"$@\"; do\n  case \"$a\" in\n    open) cat {d}/opened.json; exit 0;;\n    verify) cat {d}/verified.json; exit 0;;\n  esac\ndone\nexit 1\n",
                d = dir.display()
            ),
        )
        .unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&cli, std::fs::Permissions::from_mode(0o755)).unwrap();
        }
        let s = settings(cli.to_str().unwrap(), &dir.join("queries"));
        let evidence = vec![
            Evidence {
                party: "yes".into(),
                text: "@/etc/passwd is just text".into(),
            },
            Evidence {
                party: "no".into(),
                text: "none".into(),
            },
        ];
        let q = open_query(
            &s,
            "m1",
            f["question"].as_str().unwrap(),
            f["question_salt"].as_str().unwrap(),
            &evidence,
        )
        .await
        .unwrap();
        assert_eq!(q.query_id, "qry_01JCOUNCILFIXTURE000000000");
        let args = std::fs::read_to_string(dir.join("args.txt")).unwrap();
        assert!(args.contains("--party=yes=@"), "{args}");
        // Evidence files are removed once the query file holds them.
        assert!(!dir.join("queries/m1.yes.txt").exists());
        verify_query(&s, &q.query_id).await.unwrap();

        // Bad input never reaches the CLI.
        let bad = vec![Evidence {
            party: "Bad Party".into(),
            text: "x".into(),
        }];
        assert!(
            open_query(&s, "m2", "q?", f["question_salt"].as_str().unwrap(), &bad)
                .await
                .is_err()
        );
        assert!(open_query(&s, "m2", "q?", "0x00", &evidence).await.is_err());

        // An unpinned or failed verification is refused.
        std::fs::write(
            dir.join("verified.json"),
            r#"{"ok":true,"unpinned":true,"checks":[]}"#,
        )
        .unwrap();
        assert!(verify_query(&s, &q.query_id).await.is_err());
        std::fs::remove_dir_all(dir).ok();
    }
}
