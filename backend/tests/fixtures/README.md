# Council record fixture

`council_record.json` holds Witan decision records (yes, no and invalid outcomes) built with Witan's own `witan-core` crate and signed with its signing code (`Keys::derive(TEST_SECRET)`, the council's fixed test key), so the contract and backend tests check against the council's real hashing and signing:

- `question_hash`: `witan_core::commit::question_hash(question, question_salt)`
- commitments: `witan_core::commit::submission_commitment` per submission; `evidence_root`: `witan_core::commit::evidence_root`
- `record_jcs`: JCS of the record without `proof.signature`; `digest` = sha256 of it; `signature` = `Keys::sign_digest(digest)` (EIP-191, v = 27/28)

Records use `query_id` (`qry_` prefix). The same file lives in `contracts/test/fixtures/` and `backend/tests/fixtures/`: update both together.
