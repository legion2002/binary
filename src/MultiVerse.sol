// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {IOracle} from "./IOracle.sol";
import {ITIP20} from "./interfaces/ITIP20.sol";
import {ITIP20Factory} from "./interfaces/ITIP20Factory.sol";
import {ITIP20RolesAuth} from "./interfaces/ITIP20RolesAuth.sol";

/**
 * @title MultiVerse
 * @author Binary Protocol
 * @notice Core contract for Binary's prediction market framework that creates separate token economies for binary outcomes
 * @dev This contract manages the lifecycle of prediction markets:
 *      1. Opening markets with binary questions
 *      2. Creating verse tokens (YES/NO variants) via TIP20 Factory
 *      3. Splitting assets into verse tokens (1 token → 1 YES + 1 NO)
 *      4. Resolving markets based on oracle outcomes
 *      5. Redeeming winning tokens for underlying assets
 *      6. Combining YES + NO tokens back into the original asset
 *
 *      Key concepts:
 *      - Markets: Binary questions with oracle resolution and deadlines
 *      - Verses: TIP20 tokens representing YES and NO outcomes
 *      - Splitting: Core mechanism to enter prediction markets
 *      - Resolution: Oracle determines outcome or defaults to EVEN after deadline
 */
contract MultiVerse {
    // ============================================
    // CONSTANTS
    // ============================================

    /// @notice TIP20 Factory precompile address
    address constant TIP20_FACTORY = 0x20Fc000000000000000000000000000000000000;

    /// @notice PATH_USD token (first TIP20 token, used as quote token)
    ITIP20 constant PATH_USD = ITIP20(0x20C0000000000000000000000000000000000000);

    /// @notice ISSUER_ROLE for TIP20 tokens
    bytes32 constant ISSUER_ROLE = keccak256("ISSUER_ROLE");

    // ============================================
    // ERRORS
    // ============================================

    /// @notice Thrown when attempting to open a market that already exists
    error MarketAlreadyOpened();

    /// @notice Thrown when performing operations on markets in invalid states
    error InvalidMarketState();

    /// @notice Thrown when interacting with invalid verse token addresses
    error InvalidVerse();

    /// @notice Thrown when attempting to redeem tokens in invalid resolution states
    error InvalidResolution();

    /// @notice Thrown when attempting to create verses that already exist
    error VersesAlreadyCreated();

    // ============================================
    // EVENTS
    // ============================================

    /// @notice Emitted when a new prediction market is opened
    event MarketOpened(
        bytes32 indexed marketHash,
        bytes32 indexed questionHash,
        uint32 resolutionDeadline,
        address indexed oracle
    );

    /// @notice Emitted when YES and NO verse tokens are created for a market
    event VerseTokensCreated(
        bytes32 indexed marketHash, address indexed asset, address yesVerse, address noVerse
    );

    /// @notice Emitted when a user splits tokens into YES and NO verse tokens
    event Split(
        address indexed user,
        address indexed asset,
        bytes32 indexed marketHash,
        uint256 amount,
        address yesVerse,
        address noVerse
    );

    /// @notice Emitted when a market is resolved
    event MarketResolved(bytes32 indexed marketHash, Resolution resolution);

    /// @notice Emitted when a user redeems verse tokens for underlying assets
    event Redeemed(
        address indexed user,
        address indexed verse,
        bytes32 indexed marketHash,
        uint256 amount,
        uint256 redeemedAmount
    );

    /// @notice Emitted when a user combines YES and NO tokens back into underlying asset
    event Combined(
        address indexed user,
        address indexed asset,
        bytes32 indexed marketHash,
        uint256 amount,
        address yesVerse,
        address noVerse
    );

    // ============================================
    // TYPES
    // ============================================

    /**
     * @notice Possible resolution states for a market
     * @dev NULL: Market doesn't exist
     *      UNRESOLVED: Market is open and awaiting resolution
     *      YES: Oracle resolved to YES
     *      NO: Oracle resolved to NO
     *      EVEN: Market expired, 50/50 split between YES and NO
     */
    enum Resolution {
        NULL,
        UNRESOLVED,
        YES,
        NO,
        EVEN
    }

    /**
     * @notice Stores YES and NO verse token addresses for an asset in a market
     * @param yesVerse Address of the YES verse TIP20 token
     * @param noVerse Address of the NO verse TIP20 token
     */
    struct VerseTokens {
        address yesVerse;
        address noVerse;
    }

    /**
     * @notice Market configuration and state data
     * @param resolutionDeadline Unix timestamp after which market resolves to EVEN
     * @param oracle Address that will provide the binary resolution
     * @param questionHash Keccak256 hash of the binary question
     * @param resolution Current resolution state of the market
     * @param verses Mapping from asset address to verse token pair
     */
    struct MarketInfo {
        uint32 resolutionDeadline;
        address oracle;
        bytes32 questionHash;
        Resolution resolution;
        mapping(address asset => VerseTokens) verses;
    }

    /**
     * @notice Verse token metadata for reverse lookups
     * @param marketHash The market this verse token belongs to
     * @param asset The underlying TIP20 asset
     * @param isYes True if this is a YES verse, false if NO
     */
    struct VerseInfo {
        bytes32 marketHash;
        address asset;
        bool isYes;
    }

    // ============================================
    // STATE VARIABLES
    // ============================================

    /// @notice Stores market configuration and state for each market hash
    mapping(bytes32 marketHash => MarketInfo) internal _markets;

    /// @notice Stores verse token metadata for reverse lookups
    mapping(address verse => VerseInfo) public verseInfo;

    // ============================================
    // MARKET MANAGEMENT
    // ============================================

    /**
     * @notice Opens a new prediction market for a binary question
     * @dev Market hash is deterministically computed from parameters to ensure uniqueness
     * @param questionHash Keccak256 hash of the binary question (e.g., "Will FOCIL be included?")
     * @param resolutionDeadline Unix timestamp after which market defaults to EVEN resolution
     * @param oracle Address that will provide the YES/NO resolution before deadline
     */
    function open(bytes32 questionHash, uint32 resolutionDeadline, address oracle) public {
        // Compute unique market identifier
        bytes32 marketHash = keccak256(abi.encode(questionHash, resolutionDeadline, oracle));

        // Ensure market doesn't already exist
        if (_markets[marketHash].resolution != Resolution.NULL) {
            revert MarketAlreadyOpened();
        }

        // Initialize market state
        MarketInfo storage market = _markets[marketHash];
        market.resolutionDeadline = resolutionDeadline;
        market.oracle = oracle;
        market.questionHash = questionHash;
        market.resolution = Resolution.UNRESOLVED;

        emit MarketOpened(marketHash, questionHash, resolutionDeadline, oracle);
    }

    // ============================================
    // VERSE TOKEN CREATION
    // ============================================

    /**
     * @notice Creates YES and NO verse tokens for a specific asset in a market
     * @dev Uses TIP20 Factory to deploy tokens with PATH_USD as quote token
     * @param asset Address of the TIP20 token to create verses for
     * @param marketHash Unique identifier of the market
     * @return yesVerse Address of the deployed YES verse token
     * @return noVerse Address of the deployed NO verse token
     */
    function create(address asset, bytes32 marketHash)
        public
        returns (address yesVerse, address noVerse)
    {
        MarketInfo storage market = _markets[marketHash];

        // Ensure market exists
        if (market.resolution == Resolution.NULL) revert InvalidMarketState();

        // Ensure verses don't already exist for this asset
        if (market.verses[asset].yesVerse != address(0)) revert VersesAlreadyCreated();

        ITIP20Factory factory = ITIP20Factory(TIP20_FACTORY);

        // Create YES verse with PATH_USD as quote token
        yesVerse = factory.createToken("Yes", "YES", "VERSE", PATH_USD, address(this));

        // Create NO verse with PATH_USD as quote token
        noVerse = factory.createToken("No", "NO", "VERSE", PATH_USD, address(this));

        // Store in market mapping
        market.verses[asset] = VerseTokens(yesVerse, noVerse);

        // Store in verse lookup mapping
        verseInfo[yesVerse] = VerseInfo(marketHash, asset, true);
        verseInfo[noVerse] = VerseInfo(marketHash, asset, false);

        // Grant ISSUER_ROLE to this contract for minting/burning
        ITIP20RolesAuth(yesVerse).grantRole(ISSUER_ROLE, address(this));
        ITIP20RolesAuth(noVerse).grantRole(ISSUER_ROLE, address(this));

        emit VerseTokensCreated(marketHash, asset, yesVerse, noVerse);
    }

    // ============================================
    // CORE OPERATIONS
    // ============================================

    /**
     * @notice Splits TIP20 tokens into equal amounts of YES and NO verse tokens
     * @dev This is the primary mechanism for entering prediction markets
     *      1 token → 1 YES token + 1 NO token
     * @param asset Address of the TIP20 token to split
     * @param amount Amount of tokens to split
     * @param marketHash Unique identifier of the market
     */
    function split(address asset, uint256 amount, bytes32 marketHash) public {
        MarketInfo storage market = _markets[marketHash];

        // Ensure market is still open for trading
        if (market.resolution != Resolution.UNRESOLVED) {
            revert InvalidMarketState();
        }

        // Transfer underlying asset from user
        ITIP20(asset).transferFrom(msg.sender, address(this), amount);

        // Get verse token addresses
        VerseTokens storage verses = market.verses[asset];
        if (verses.yesVerse == address(0)) revert InvalidVerse();

        // Mint equal amounts of YES and NO tokens to user
        ITIP20(verses.yesVerse).mint(msg.sender, amount);
        ITIP20(verses.noVerse).mint(msg.sender, amount);

        emit Split(msg.sender, asset, marketHash, amount, verses.yesVerse, verses.noVerse);
    }

    /**
     * @notice Resolves a market based on oracle input or deadline expiry
     * @dev Can be called by anyone. Resolution is permanent and affects redemption values:
     *      - Before deadline: Uses oracle's YES/NO resolution
     *      - After deadline: Automatically resolves to EVEN (50/50 split)
     * @param marketHash Unique identifier of the market to resolve
     */
    function resolve(bytes32 marketHash) public {
        MarketInfo storage market = _markets[marketHash];

        // Ensure market hasn't already been resolved
        if (market.resolution != Resolution.UNRESOLVED) {
            revert InvalidMarketState();
        }

        Resolution resolution;
        // Check if deadline has passed
        if (block.timestamp >= market.resolutionDeadline) {
            // Past deadline: split value equally between YES and NO
            resolution = Resolution.EVEN;
        } else {
            // Before deadline: query oracle for resolution
            bool isYes = IOracle(market.oracle).getResolution(marketHash);
            resolution = isYes ? Resolution.YES : Resolution.NO;
        }

        market.resolution = resolution;
        emit MarketResolved(marketHash, resolution);
    }

    /**
     * @notice Redeems verse tokens for underlying assets based on market resolution
     * @dev Redemption values depend on resolution:
     *      - Winning tokens (YES on YES resolution, NO on NO resolution): 1:1 redemption
     *      - Losing tokens: 0 redemption value
     *      - EVEN resolution: 0.5:1 redemption for both YES and NO
     * @param verse Address of the verse token to redeem
     * @param amount Amount of verse tokens to redeem
     * @return redeemedAmount Amount of underlying asset received
     */
    function redeem(address verse, uint256 amount) public returns (uint256 redeemedAmount) {
        VerseInfo storage info = verseInfo[verse];

        // Verify this is a valid verse token
        if (info.asset == address(0)) revert InvalidVerse();

        MarketInfo storage market = _markets[info.marketHash];
        Resolution resolution = market.resolution;

        // Calculate redemption amount based on resolution
        if (resolution == Resolution.YES && info.isYes) {
            redeemedAmount = amount; // YES tokens worth 100%
        } else if (resolution == Resolution.NO && !info.isYes) {
            redeemedAmount = amount; // NO tokens worth 100%
        } else if (resolution == Resolution.EVEN) {
            redeemedAmount = amount / 2; // Both worth 50%
        } else {
            revert InvalidResolution();
        }

        // Transfer verse tokens from user and burn
        ITIP20(verse).transferFrom(msg.sender, address(this), amount);
        ITIP20(verse).burn(amount);

        // Transfer underlying asset to user
        ITIP20(info.asset).transfer(msg.sender, redeemedAmount);

        emit Redeemed(msg.sender, verse, info.marketHash, amount, redeemedAmount);
    }

    /**
     * @notice Combines equal amounts of YES and NO tokens back into the underlying asset
     * @dev Reverse operation of split: 1 YES + 1 NO → 1 underlying token
     *      Only available while market is unresolved
     * @param asset Address of the underlying TIP20 token
     * @param amount Amount of YES and NO tokens to combine
     * @param marketHash Unique identifier of the market
     */
    function combine(address asset, uint256 amount, bytes32 marketHash) public {
        MarketInfo storage market = _markets[marketHash];

        // Ensure market is still open
        if (market.resolution != Resolution.UNRESOLVED) {
            revert InvalidMarketState();
        }

        // Get verse token addresses
        VerseTokens storage verses = market.verses[asset];
        if (verses.yesVerse == address(0)) revert InvalidVerse();

        // Transfer verse tokens from user
        ITIP20(verses.yesVerse).transferFrom(msg.sender, address(this), amount);
        ITIP20(verses.noVerse).transferFrom(msg.sender, address(this), amount);

        // Burn the verse tokens
        ITIP20(verses.yesVerse).burn(amount);
        ITIP20(verses.noVerse).burn(amount);

        // Return underlying asset to user
        ITIP20(asset).transfer(msg.sender, amount);

        emit Combined(msg.sender, asset, marketHash, amount, verses.yesVerse, verses.noVerse);
    }

    // ============================================
    // VIEW FUNCTIONS
    // ============================================

    /**
     * @notice Returns market configuration data
     * @param marketHash Unique identifier of the market
     * @return resolutionDeadline Unix timestamp after which market resolves to EVEN
     * @return oracle Address that provides the binary resolution
     * @return questionHash Keccak256 hash of the binary question
     * @return resolution Current resolution state
     */
    function markets(bytes32 marketHash)
        public
        view
        returns (uint32 resolutionDeadline, address oracle, bytes32 questionHash, Resolution resolution)
    {
        MarketInfo storage market = _markets[marketHash];
        return (market.resolutionDeadline, market.oracle, market.questionHash, market.resolution);
    }

    /**
     * @notice Returns the verse token addresses for a specific asset in a market
     * @param asset Address of the underlying TIP20 token
     * @param marketHash Unique identifier of the market
     * @return yesVerse Address of the YES verse token
     * @return noVerse Address of the NO verse token
     */
    function getVerseAddress(address asset, bytes32 marketHash)
        public
        view
        returns (address yesVerse, address noVerse)
    {
        VerseTokens storage verses = _markets[marketHash].verses[asset];
        return (verses.yesVerse, verses.noVerse);
    }

    /**
     * @notice Verifies if an address is a valid verse token created by this contract
     * @param verse Address to verify
     * @return True if the address is a valid verse token
     */
    function isVerse(address verse) public view returns (bool) {
        return verseInfo[verse].asset != address(0);
    }
}
