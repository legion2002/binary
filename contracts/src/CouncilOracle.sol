// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {IOracle} from "./IOracle.sol";
import {Ownable} from "solady/auth/Ownable.sol";
import {ECDSA} from "solady/utils/ECDSA.sol";
import {LibBytes} from "solady/utils/LibBytes.sol";
import {LibString} from "solady/utils/LibString.sol";

/**
 * @title CouncilOracle
 * @author Binary Protocol
 * @notice Resolves binary markets from signed decision records of an AI council (Gavel).
 * @dev A council service decides a question with a panel of AI models and signs a decision
 *      record: RFC 8785 (JCS) canonical JSON. This contract accepts a record only when
 *
 *      - its signature recovers to a trusted signer: secp256k1 over EIP-191 of
 *        digest = sha256(JCS(record without proof.signature)),
 *      - its "council_hash" is the council config registered for the market,
 *      - its "evidence_root" commits to the question registered for the market:
 *        evidence_root = sha256('{"commitments":' || commitmentsJcs || ',"question_hash":"0x..."}')
 *        where question_hash = sha256(JCS({"question": q, "salt": question_salt})) is fixed at
 *        registration, so the council provably decided this exact question,
 *      - its "outcome" is exactly {"type":"binary","value":"yes"} or {"type":"binary","value":"no"}.
 *        Any other outcome (an invalid decision) reverts and the market stays unresolved; the
 *        resolver may open a new query and try again.
 *
 *      Parsing. The record is canonical JSON produced by the trusted signer, so this contract
 *      reads fields by searching for the byte pattern "key": and takes the first match. That
 *      match is always the real field:
 *      1. A pattern "key": can only match a JSON object key. Inside a string value every
 *         double quote is escaped with a backslash (JCS escapes quote and backslash), so the
 *         quote that closes "key" in the pattern would have to follow a backslash, not the
 *         key's last letter; and a quote that closes a string value is followed by , ] } or :,
 *         never by a key letter. So no string content (model ids, member ids, votes; the
 *         question and evidence texts never appear in a record) can forge a match.
 *      2. The keys read here (council_hash, evidence_root, outcome, query_id, signer, v) occur
 *         exactly once in a record, each at one fixed place: the signer serializes a fixed
 *         struct with unknown fields denied, and no nested object reuses these names
 *         ("signer" only in proof, the rest only at the top level).
 *      3. On top of that, JCS sorts keys, so every field before "outcome" (commitments,
 *         council_hash, decided_at, evidence_root, opened_at) holds only hex, timestamps and
 *         party names limited to [a-z0-9_-]: nothing caller-written precedes the outcome.
 *      A record that is not well formed fails one of the exact byte comparisons.
 *
 *      Trust model: the trusted signer key is the trust root (the council runs without a
 *      hardware TEE in v1); the council config is pinned by hash and the question by salted
 *      hash at registration; resolvers choose which evidence the council sees, so only
 *      resolvers may submit records.
 */
contract CouncilOracle is IOracle, Ownable {
    using LibBytes for bytes;

    // ============================================
    // TYPES
    // ============================================

    /// @notice What a market is pinned to, and its resolution once set.
    struct MarketConfig {
        /// @dev sha256 of the council config JSON the market must be decided by.
        bytes32 councilHash;
        /// @dev sha256(JCS({"question","salt"})): the question the council must decide.
        bytes32 questionHash;
        bool registered;
        bool resolved;
        bool isYes;
    }

    // ============================================
    // EVENTS
    // ============================================

    event SignerSet(address indexed signer, bool trusted);
    event ResolverSet(address indexed resolver, bool allowed);
    event MarketRegistered(bytes32 indexed marketHash, bytes32 councilHash, bytes32 questionHash);
    /// @param queryIdHash keccak256 of the record's query_id string
    /// @param digest sha256 of the signed record bytes (the record's digest)
    event ResolutionSet(
        bytes32 indexed marketHash,
        bool isYes,
        bytes32 indexed queryIdHash,
        bytes32 digest,
        address signer,
        string queryId
    );

    // ============================================
    // ERRORS
    // ============================================

    /// @notice Thrown when querying resolution for a market that hasn't been set
    error MarketNotSet();
    error NotResolver();
    error ZeroHash();
    error MarketAlreadyRegistered();
    error MarketNotRegistered();
    error MarketAlreadyResolved();
    error UntrustedSigner(address recovered);
    /// @notice A field the contract reads is missing or malformed.
    error MalformedRecord(string field);
    error UnsupportedRecordVersion();
    error SignerMismatch();
    error CouncilMismatch();
    error EvidenceRootMismatch();
    /// @notice The council decided "invalid" (or anything but yes/no): the market stays open.
    error OutcomeNotBinary();

    // ============================================
    // STATE VARIABLES
    // ============================================

    /// @notice Council signer addresses whose records are accepted.
    mapping(address => bool) public trustedSigners;

    /// @notice Addresses allowed to register markets and submit records.
    mapping(address => bool) public resolvers;

    /// @notice Per market: pinned council and question, and the resolution once set.
    mapping(bytes32 => MarketConfig) public markets;

    bytes private constant OUTCOME_YES = '{"type":"binary","value":"yes"}';
    bytes private constant OUTCOME_NO = '{"type":"binary","value":"no"}';

    // ============================================
    // CONSTRUCTOR
    // ============================================

    /**
     * @param initialSigner Trusted council signer (address(0) to add later)
     * @dev The deployer becomes owner and resolver.
     */
    constructor(address initialSigner) {
        _initializeOwner(msg.sender);
        _setResolver(msg.sender, true);
        if (initialSigner != address(0)) _setSigner(initialSigner, true);
    }

    // ============================================
    // OWNER FUNCTIONS
    // ============================================

    /// @notice Trusts or distrusts a council signer.
    function setSigner(address signer, bool trusted) external onlyOwner {
        _setSigner(signer, trusted);
    }

    /// @notice Grants or revokes the resolver role.
    function setResolver(address resolver, bool allowed) external onlyOwner {
        _setResolver(resolver, allowed);
    }

    // ============================================
    // RESOLVER FUNCTIONS
    // ============================================

    /**
     * @notice Pins a market to a council config and a salted question hash. Once only.
     * @dev Callable by the owner or a resolver.
     * @param marketHash MultiVerse market hash
     * @param councilHash sha256 of the council config (council_hash in records)
     * @param questionHash sha256(JCS({"question": q, "salt": question_salt}))
     */
    function registerMarket(bytes32 marketHash, bytes32 councilHash, bytes32 questionHash)
        external
    {
        if (!resolvers[msg.sender] && msg.sender != owner()) revert NotResolver();
        if (councilHash == bytes32(0) || questionHash == bytes32(0)) revert ZeroHash();
        MarketConfig storage m = markets[marketHash];
        if (m.registered) revert MarketAlreadyRegistered();
        m.councilHash = councilHash;
        m.questionHash = questionHash;
        m.registered = true;
        emit MarketRegistered(marketHash, councilHash, questionHash);
    }

    /**
     * @notice Resolves a market from a signed council decision record.
     * @dev Only resolvers: they choose the evidence the council saw.
     * @param marketHash MultiVerse market hash, registered before
     * @param recordJcs JCS bytes of the record with proof.signature removed
     * @param commitmentsJcs JCS bytes of the record's "commitments" array
     * @param signature 65 bytes r||s||v (v = 27/28): the record's proof.signature
     */
    function resolve(
        bytes32 marketHash,
        bytes calldata recordJcs,
        bytes calldata commitmentsJcs,
        bytes calldata signature
    ) external {
        if (!resolvers[msg.sender]) revert NotResolver();
        MarketConfig storage m = markets[marketHash];
        if (!m.registered) revert MarketNotRegistered();
        if (m.resolved) revert MarketAlreadyResolved();

        bytes memory record = recordJcs;

        // 1. Signature over digest = sha256(record without proof.signature), EIP-191.
        bytes32 digest = sha256(record);
        address signer = ECDSA.tryRecover(ECDSA.toEthSignedMessageHash(digest), signature);
        if (signer == address(0) || !trustedSigners[signer]) revert UntrustedSigner(signer);

        // 2. Fields. See the contract doc for why the first match is the real field.
        if (!_slice(record, _after(record, '"v":', "v"), 2).eq(bytes("1,"))) {
            revert UnsupportedRecordVersion();
        }
        if (!_stringField(record, "signer", 42).eq(bytes(LibString.toHexString(signer)))) {
            revert SignerMismatch();
        }
        if (!_stringField(record, "council_hash", 66).eq(_hex32(m.councilHash))) {
            revert CouncilMismatch();
        }
        // Exact bytes of Gavel's evidence root preimage. The fixed-length suffix pins
        // question_hash: a different commitmentsJcs cannot shift it into a match.
        bytes32 expectedRoot = sha256(
            // forge-lint: disable-next-line(encode-packed-collision)
            abi.encodePacked(
                '{"commitments":',
                commitmentsJcs,
                ',"question_hash":"',
                _hex32(m.questionHash),
                '"}'
            )
        );
        if (!_stringField(record, "evidence_root", 66).eq(_hex32(expectedRoot))) {
            revert EvidenceRootMismatch();
        }

        uint256 start = _after(record, '"outcome":', "outcome");
        bool isYes = _matchesAt(record, start, OUTCOME_YES);
        if (!isYes && !_matchesAt(record, start, OUTCOME_NO)) revert OutcomeNotBinary();

        string memory queryId = _queryId(record);

        m.resolved = true;
        m.isYes = isYes;
        emit ResolutionSet(marketHash, isYes, keccak256(bytes(queryId)), digest, signer, queryId);
    }

    // ============================================
    // ORACLE INTERFACE
    // ============================================

    /**
     * @notice Returns the resolution for a specific market
     * @dev Implements IOracle interface. Reverts if no record resolved it yet.
     * @param marketHash Unique identifier of the market
     * @return True for YES resolution, false for NO resolution
     */
    function getResolution(bytes32 marketHash) external view override returns (bool) {
        MarketConfig storage m = markets[marketHash];
        if (!m.resolved) revert MarketNotSet();
        return m.isYes;
    }

    // ============================================
    // INTERNAL
    // ============================================

    function _setSigner(address signer, bool trusted) internal {
        trustedSigners[signer] = trusted;
        emit SignerSet(signer, trusted);
    }

    function _setResolver(address resolver, bool allowed) internal {
        resolvers[resolver] = allowed;
        emit ResolverSet(resolver, allowed);
    }

    /// @dev Offset just past the first occurrence of pattern; reverts when absent.
    function _after(bytes memory record, bytes memory pattern, string memory field)
        private
        pure
        returns (uint256)
    {
        uint256 i = record.indexOf(pattern);
        if (i == LibBytes.NOT_FOUND) revert MalformedRecord(field);
        return i + pattern.length;
    }

    /// @dev record[start:start+len], or empty when out of range.
    function _slice(bytes memory record, uint256 start, uint256 len)
        private
        pure
        returns (bytes memory)
    {
        if (start + len > record.length) return "";
        return record.slice(start, start + len);
    }

    /// @dev The value of string field "key", which must be exactly len bytes long.
    function _stringField(bytes memory record, string memory key, uint256 len)
        private
        pure
        returns (bytes memory)
    {
        uint256 start = _after(record, abi.encodePacked('"', key, '":"'), key);
        if (start + len >= record.length || record[start + len] != '"') {
            revert MalformedRecord(key);
        }
        return record.slice(start, start + len);
    }

    /// @dev The record's query_id: up to the next quote (ids are ASCII letters, digits, '_').
    function _queryId(bytes memory record) private pure returns (string memory) {
        uint256 start = _after(record, '"query_id":"', "query_id");
        uint256 end = record.indexOfByte('"', start);
        if (end == LibBytes.NOT_FOUND || end == start || end - start > 64) {
            revert MalformedRecord("query_id");
        }
        return string(record.slice(start, end));
    }

    function _matchesAt(bytes memory record, uint256 start, bytes memory expected)
        private
        pure
        returns (bool)
    {
        return _slice(record, start, expected.length).eq(expected);
    }

    /// @dev "0x" + 64 lowercase hex, as records write hashes.
    function _hex32(bytes32 value) private pure returns (bytes memory) {
        return bytes(LibString.toHexString(uint256(value), 32));
    }
}
