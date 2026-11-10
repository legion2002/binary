// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import {CouncilOracle} from "../src/CouncilOracle.sol";
import {MultiVerse} from "../src/MultiVerse.sol";
import {Ownable} from "solady/auth/Ownable.sol";
import {LibString} from "solady/utils/LibString.sol";

/// @dev Records here follow Witan's decision record exactly (RFC 8785 JCS, keys sorted).
///      test/fixtures/council_record.json holds records built and signed by Witan's own
///      witan-core code; the other tests build the same shape and sign it with vm.sign.
contract CouncilOracleTest is Test {
    CouncilOracle oracle;

    uint256 constant SIGNER_PK = 0xA11CE;
    uint256 constant OTHER_PK = 0xB0B;
    address signer;
    address resolver = makeAddr("resolver");

    bytes32 constant MARKET = keccak256("market");
    bytes32 constant COUNCIL = 0x7777777777777777777777777777777777777777777777777777777777777777;
    string constant QUESTION = "Will it rain in Lisbon on 2026-12-01?";
    string constant QSALT = "0x0123456789abcdef0123456789abcdef";
    string constant COMMITMENTS =
        '[{"hash":"0x75219e5deba79b8a2f272797c9a91d4c072dff68cc0a00876fcf0df56397bfc9","party":"yes"},{"hash":"0xf605bef6a23293021436eafc2ea4aa67437cbf442100344b3ccf843dc2a4bf29","party":"no"}]';
    string constant YES = '{"type":"binary","value":"yes"}';
    string constant NO = '{"type":"binary","value":"no"}';
    string constant INVALID = '{"reason":"no_threshold","type":"invalid"}';

    bytes32 questionHash;

    function setUp() public {
        signer = vm.addr(SIGNER_PK);
        oracle = new CouncilOracle(signer);
        oracle.setResolver(resolver, true);
        // sha256(JCS({"question": q, "salt": s})): keys sorted, plain ASCII needs no escapes.
        questionHash =
            sha256(abi.encodePacked('{"question":"', QUESTION, '","salt":"', QSALT, '"}'));
        oracle.registerMarket(MARKET, COUNCIL, questionHash);
    }

    // ============================================
    // HELPERS
    // ============================================

    function hex32(bytes32 v) internal pure returns (string memory) {
        return LibString.toHexString(uint256(v), 32);
    }

    function rootFor(bytes32 qh, string memory commitments) internal pure returns (bytes32) {
        return sha256(
            abi.encodePacked('{"commitments":', commitments, ',"question_hash":"', hex32(qh), '"}')
        );
    }

    /// @dev JCS of a record without proof.signature, as Witan serializes it.
    function record(bytes32 council, bytes32 root, string memory outcome, address proofSigner)
        internal
        pure
        returns (bytes memory)
    {
        return recordAt(council, root, outcome, proofSigner, "2026-11-10T12:00:09Z");
    }

    function recordAt(
        bytes32 council,
        bytes32 root,
        string memory outcome,
        address proofSigner,
        string memory decidedAt
    ) internal pure returns (bytes memory) {
        return abi.encodePacked(
            '{"commitments":',
            COMMITMENTS,
            ',"council_hash":"',
            hex32(council),
            '","decided_at":"',
            decidedAt,
            '","evidence_root":"',
            hex32(root),
            '","opened_at":"2026-11-10T12:00:00Z","outcome":',
            outcome,
            ',"panel":["m1","m3","m4"],"proof":{"attestation_hash":"0x5555555555555555555555555555555555555555555555555555555555555555","measurement":"0x6666666666666666666666666666666666666666666666666666666666666666","signer":"',
            LibString.toHexString(proofSigner),
            '"},"query_id":"qry_01JCOUNCILTEST00000000000A","round":1,"screening":{"flagged":[],"ran":false},"seed":"0x1111111111111111111111111111111111111111111111111111111111111111","v":1,"votes":[{"member":"m1","model":"openai/gpt-6-luna","response_hash":"0x2222222222222222222222222222222222222222222222222222222222222222","vote":"yes"},{"member":"m3","model":"x/\"outcome\":{\"type\":\"binary\",\"value\":\"no\"}","response_hash":"0x3333333333333333333333333333333333333333333333333333333333333333","vote":"yes"},{"member":"m4","model":"google/gemini-2.5-flash","response_hash":"0x4444444444444444444444444444444444444444444444444444444444444444","vote":null}]}'
        );
    }

    function goodRecord(string memory outcome) internal view returns (bytes memory) {
        return record(COUNCIL, rootFor(questionHash, COMMITMENTS), outcome, signer);
    }

    /// @dev Witan's proof.signature: EIP-191 over the 32-byte sha256 digest, r||s||v, v = 27/28.
    function sign(uint256 pk, bytes memory rec) internal pure returns (bytes memory) {
        bytes32 digest = sha256(rec);
        bytes32 eth = keccak256(abi.encodePacked("\x19Ethereum Signed Message:\n32", digest));
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(pk, eth);
        return abi.encodePacked(r, s, v);
    }

    function resolveAs(address who, bytes32 market, bytes memory rec, bytes memory sig) internal {
        vm.prank(who);
        oracle.resolve(market, rec, bytes(COMMITMENTS), sig);
    }

    // ============================================
    // FIXTURE: RECORDS SIGNED BY WITAN'S OWN CODE
    // ============================================

    function loadFixture() internal view returns (string memory json) {
        json = vm.readFile(string.concat(vm.projectRoot(), "/test/fixtures/council_record.json"));
    }

    function fixtureOracle(string memory json) internal returns (CouncilOracle o, bytes32 market) {
        o = new CouncilOracle(vm.parseJsonAddress(json, ".signer"));
        market = keccak256("fixture market");
        o.registerMarket(
            market,
            vm.parseJsonBytes32(json, ".council_hash"),
            vm.parseJsonBytes32(json, ".question_hash")
        );
    }

    function fixtureResolve(
        CouncilOracle o,
        bytes32 market,
        string memory json,
        string memory which
    ) internal {
        o.resolve(
            market,
            vm.parseJsonBytes(json, string.concat(".", which, ".record_jcs_hex")),
            bytes(vm.parseJsonString(json, ".commitments_jcs")),
            vm.parseJsonBytes(json, string.concat(".", which, ".signature"))
        );
    }

    function test_fixture_yes() public {
        string memory json = loadFixture();
        (CouncilOracle o, bytes32 market) = fixtureOracle(json);
        vm.expectEmit(true, true, false, true, address(o));
        emit CouncilOracle.ResolutionSet(
            market,
            true,
            keccak256("qry_01JCOUNCILFIXTURE000000000"),
            vm.parseJsonBytes32(json, ".yes.digest"),
            vm.parseJsonAddress(json, ".signer"),
            "qry_01JCOUNCILFIXTURE000000000"
        );
        fixtureResolve(o, market, json, "yes");
        assertTrue(o.getResolution(market));
    }

    function test_fixture_no() public {
        string memory json = loadFixture();
        (CouncilOracle o, bytes32 market) = fixtureOracle(json);
        fixtureResolve(o, market, json, "no");
        assertFalse(o.getResolution(market));
    }

    function test_fixture_invalid_reverts_and_market_stays_open() public {
        string memory json = loadFixture();
        (CouncilOracle o, bytes32 market) = fixtureOracle(json);
        vm.expectRevert(CouncilOracle.OutcomeNotBinary.selector);
        this.fixtureResolveExternal(o, market, json, "invalid");
        vm.expectRevert(CouncilOracle.MarketNotSet.selector);
        o.getResolution(market);
        // A new query can still resolve it.
        fixtureResolve(o, market, json, "yes");
        assertTrue(o.getResolution(market));
    }

    function fixtureResolveExternal(
        CouncilOracle o,
        bytes32 market,
        string memory json,
        string memory which
    ) external {
        fixtureResolve(o, market, json, which);
    }

    function test_fixture_question_hash_matches_witan() public view {
        // The fixture question contains quotes, which JCS escapes as \".
        string memory json = loadFixture();
        bytes32 qh = sha256(
            abi.encodePacked(
                '{"question":"Did \\"Team A\\" win the final on 2026-11-09? Answer yes or no.","salt":"',
                vm.parseJsonString(json, ".question_salt"),
                '"}'
            )
        );
        assertEq(qh, vm.parseJsonBytes32(json, ".question_hash"));
        assertEq(
            rootFor(qh, vm.parseJsonString(json, ".commitments_jcs")),
            vm.parseJsonBytes32(json, ".evidence_root")
        );
    }

    // ============================================
    // RESOLUTION
    // ============================================

    function test_resolve_yes() public {
        bytes memory rec = goodRecord(YES);
        resolveAs(resolver, MARKET, rec, sign(SIGNER_PK, rec));
        assertTrue(oracle.getResolution(MARKET));
    }

    function test_resolve_no() public {
        bytes memory rec = goodRecord(NO);
        resolveAs(resolver, MARKET, rec, sign(SIGNER_PK, rec));
        assertFalse(oracle.getResolution(MARKET));
    }

    function test_invalid_outcome_reverts() public {
        bytes memory rec = goodRecord(INVALID);
        bytes memory sig = sign(SIGNER_PK, rec);
        vm.expectRevert(CouncilOracle.OutcomeNotBinary.selector);
        resolveAs(resolver, MARKET, rec, sig);
        vm.expectRevert(CouncilOracle.MarketNotSet.selector);
        oracle.getResolution(MARKET);
    }

    function test_other_outcome_types_revert() public {
        string[3] memory outcomes = [
            '{"type":"multiple_choice","value":"yes"}',
            '{"type":"split","value":70}',
            '{"type":"binary","value":"invalid"}'
        ];
        for (uint256 i; i < outcomes.length; i++) {
            bytes memory rec = goodRecord(outcomes[i]);
            bytes memory sig = sign(SIGNER_PK, rec);
            vm.expectRevert(CouncilOracle.OutcomeNotBinary.selector);
            resolveAs(resolver, MARKET, rec, sig);
        }
    }

    function test_wrong_signer_reverts() public {
        bytes memory rec =
            record(COUNCIL, rootFor(questionHash, COMMITMENTS), YES, vm.addr(OTHER_PK));
        bytes memory sig = sign(OTHER_PK, rec);
        vm.expectRevert(
            abi.encodeWithSelector(CouncilOracle.UntrustedSigner.selector, vm.addr(OTHER_PK))
        );
        resolveAs(resolver, MARKET, rec, sig);
    }

    function test_removed_signer_reverts() public {
        oracle.setSigner(signer, false);
        bytes memory rec = goodRecord(YES);
        bytes memory sig = sign(SIGNER_PK, rec);
        vm.expectRevert(abi.encodeWithSelector(CouncilOracle.UntrustedSigner.selector, signer));
        resolveAs(resolver, MARKET, rec, sig);
    }

    function test_proof_signer_must_match() public {
        bytes memory rec =
            record(COUNCIL, rootFor(questionHash, COMMITMENTS), YES, vm.addr(OTHER_PK));
        bytes memory sig = sign(SIGNER_PK, rec);
        vm.expectRevert(CouncilOracle.SignerMismatch.selector);
        resolveAs(resolver, MARKET, rec, sig);
    }

    function test_wrong_council_reverts() public {
        bytes32 other = keccak256("other council");
        bytes memory rec = record(other, rootFor(questionHash, COMMITMENTS), YES, signer);
        bytes memory sig = sign(SIGNER_PK, rec);
        vm.expectRevert(CouncilOracle.CouncilMismatch.selector);
        resolveAs(resolver, MARKET, rec, sig);
    }

    function test_wrong_question_reverts() public {
        bytes32 otherQ =
            sha256(abi.encodePacked('{"question":"Will it snow?","salt":"', QSALT, '"}'));
        bytes memory rec = record(COUNCIL, rootFor(otherQ, COMMITMENTS), YES, signer);
        bytes memory sig = sign(SIGNER_PK, rec);
        vm.expectRevert(CouncilOracle.EvidenceRootMismatch.selector);
        resolveAs(resolver, MARKET, rec, sig);
    }

    function test_wrong_commitments_reverts() public {
        bytes memory rec = goodRecord(YES);
        bytes memory sig = sign(SIGNER_PK, rec);
        vm.prank(resolver);
        vm.expectRevert(CouncilOracle.EvidenceRootMismatch.selector);
        oracle.resolve(MARKET, rec, bytes('[{"hash":"0x00","party":"yes"}]'), sig);
    }

    function test_tampered_record_reverts() public {
        bytes memory rec = goodRecord(NO);
        bytes memory sig = sign(SIGNER_PK, rec);
        bytes memory tampered = goodRecord(YES);
        vm.prank(resolver);
        vm.expectRevert(); // recovers to some other address
        oracle.resolve(MARKET, tampered, bytes(COMMITMENTS), sig);
        // Flip one byte of the signature too.
        sig[10] = bytes1(uint8(sig[10]) ^ 1);
        vm.prank(resolver);
        vm.expectRevert();
        oracle.resolve(MARKET, rec, bytes(COMMITMENTS), sig);
        vm.expectRevert(CouncilOracle.MarketNotSet.selector);
        oracle.getResolution(MARKET);
    }

    function test_double_resolve_reverts() public {
        bytes memory rec = goodRecord(YES);
        bytes memory sig = sign(SIGNER_PK, rec);
        resolveAs(resolver, MARKET, rec, sig);
        bytes memory rec2 = goodRecord(NO);
        bytes memory sig2 = sign(SIGNER_PK, rec2);
        vm.expectRevert(CouncilOracle.MarketAlreadyResolved.selector);
        resolveAs(resolver, MARKET, rec2, sig2);
        assertTrue(oracle.getResolution(MARKET));
    }

    function test_unregistered_market_reverts() public {
        bytes memory rec = goodRecord(YES);
        bytes memory sig = sign(SIGNER_PK, rec);
        vm.expectRevert(CouncilOracle.MarketNotRegistered.selector);
        resolveAs(resolver, keccak256("unknown"), rec, sig);
    }

    function test_only_resolver_resolves() public {
        bytes memory rec = goodRecord(YES);
        bytes memory sig = sign(SIGNER_PK, rec);
        vm.expectRevert(CouncilOracle.NotResolver.selector);
        resolveAs(makeAddr("anyone"), MARKET, rec, sig);
    }

    function test_get_resolution_reverts_when_unset() public {
        vm.expectRevert(CouncilOracle.MarketNotSet.selector);
        oracle.getResolution(MARKET);
    }

    function test_escaped_key_inside_a_string_is_not_matched() public {
        // A string value holding "outcome":{...yes} is escaped by JCS (\"outcome\":), so the
        // pattern "outcome": cannot match inside it, even before the real outcome field.
        bytes memory rec = recordAt(
            COUNCIL,
            rootFor(questionHash, COMMITMENTS),
            NO,
            signer,
            'x\\",\\"outcome\\":{\\"type\\":\\"binary\\",\\"value\\":\\"yes\\"}'
        );
        resolveAs(resolver, MARKET, rec, sign(SIGNER_PK, rec));
        assertFalse(oracle.getResolution(MARKET));
    }

    function test_unsupported_version_reverts() public {
        bytes memory rec = goodRecord(YES);
        // Replace "v":1 with "v":2 (the only "v": key) and re-sign.
        bytes memory needle = bytes('"v":1,');
        for (uint256 i; i + needle.length <= rec.length; i++) {
            if (keccak256(_sub(rec, i, needle.length)) == keccak256(needle)) {
                rec[i + 4] = "2";
                break;
            }
        }
        bytes memory sig = sign(SIGNER_PK, rec);
        vm.expectRevert(CouncilOracle.UnsupportedRecordVersion.selector);
        resolveAs(resolver, MARKET, rec, sig);
    }

    function _sub(bytes memory b, uint256 start, uint256 len)
        internal
        pure
        returns (bytes memory out)
    {
        out = new bytes(len);
        for (uint256 i; i < len; i++) {
            out[i] = b[start + i];
        }
    }

    // ============================================
    // ADMIN
    // ============================================

    function test_register_once_by_owner_or_resolver() public {
        vm.expectRevert(CouncilOracle.MarketAlreadyRegistered.selector);
        oracle.registerMarket(MARKET, COUNCIL, questionHash);

        vm.prank(resolver);
        oracle.registerMarket(keccak256("m2"), COUNCIL, questionHash);

        vm.prank(makeAddr("anyone"));
        vm.expectRevert(CouncilOracle.NotResolver.selector);
        oracle.registerMarket(keccak256("m3"), COUNCIL, questionHash);

        vm.expectRevert(CouncilOracle.ZeroHash.selector);
        oracle.registerMarket(keccak256("m4"), bytes32(0), questionHash);
    }

    function test_only_owner_manages_roles() public {
        vm.startPrank(makeAddr("anyone"));
        vm.expectRevert(Ownable.Unauthorized.selector);
        oracle.setSigner(address(1), true);
        vm.expectRevert(Ownable.Unauthorized.selector);
        oracle.setResolver(address(1), true);
        vm.stopPrank();
        assertTrue(oracle.trustedSigners(signer));
        assertTrue(oracle.resolvers(address(this)));
    }

    // ============================================
    // MULTIVERSE
    // ============================================

    function test_multiverse_resolves_from_council() public {
        MultiVerse mv = new MultiVerse();
        bytes32 q = keccak256(bytes(QUESTION));
        uint32 deadline = uint32(block.timestamp + 1 days);
        mv.open(q, deadline, address(oracle));
        bytes32 marketHash = keccak256(abi.encode(q, deadline, address(oracle)));
        oracle.registerMarket(marketHash, COUNCIL, questionHash);

        bytes memory rec = goodRecord(NO);
        resolveAs(resolver, marketHash, rec, sign(SIGNER_PK, rec));
        mv.resolve(marketHash);
        (,,, MultiVerse.Resolution resolution) = mv.markets(marketHash);
        assertEq(uint8(resolution), uint8(MultiVerse.Resolution.NO));
    }
}
