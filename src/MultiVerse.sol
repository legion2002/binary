// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {ERC20} from "solady/tokens/ERC20.sol";
import {Verse} from "./Verse.sol";
import {IOracle} from "./IOracle.sol";

/**
 * @title MultiVerse
 * @author Binary Protocol
 * @notice Core contract for Binary's prediction market framework that creates separate token economies for binary outcomes
 * @dev This contract manages the lifecycle of prediction markets:
 *      1. Opening markets with binary questions
 *      2. Creating verse tokens (YES/NO variants) for any ERC20 asset
 *      3. Splitting assets into verse tokens (1 token → 1 YES + 1 NO)
 *      4. Resolving markets based on oracle outcomes
 *      5. Redeeming winning tokens for underlying assets
 *      6. Combining YES + NO tokens back into the original asset
 *
 *      Key concepts:
 *      - Markets: Binary questions with oracle resolution and deadlines
 *      - Verses: Separate token economies for YES and NO outcomes
 *      - Splitting: Core mechanism to enter prediction markets
 *      - Resolution: Oracle determines outcome or defaults to EVEN after deadline
 */
contract MultiVerse {
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
     * @notice Market configuration data
     * @param resolutionDeadline Unix timestamp after which market resolves to EVEN
     * @param oracle Address that will provide the binary resolution
     * @param questionHash Keccak256 hash of the binary question
     */
    struct Market {
        uint32 resolutionDeadline;
        address oracle;
        bytes32 questionHash;
    }

    // ============================================
    // STATE VARIABLES
    // ============================================

    /// @notice Stores market configuration for each market hash
    mapping(bytes32 marketHash => Market) public markets;

    /// @notice Stores resolution state for each market hash
    mapping(bytes32 marketHash => Resolution) public resolutions;

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
        if (resolutions[marketHash] != Resolution.NULL) {
            revert MarketAlreadyOpened();
        }

        // Initialize market state
        resolutions[marketHash] = Resolution.UNRESOLVED;
        markets[marketHash] = Market({
            resolutionDeadline: resolutionDeadline, oracle: oracle, questionHash: questionHash
        });

        emit MarketOpened(marketHash, questionHash, resolutionDeadline, oracle);
    }

    // ============================================
    // VERSE TOKEN CREATION
    // ============================================

    /**
     * @notice Creates YES and NO verse tokens for a specific asset in a market
     * @dev Uses CREATE2 for deterministic addresses, allowing pre-computation of verse addresses
     * @param asset Address of the ERC20 token to create verses for (e.g., WETH, USDC)
     * @param marketHash Unique identifier of the market
     * @return yesVerse Address of the deployed YES verse token
     * @return noVerse Address of the deployed NO verse token
     */
    function create(address asset, bytes32 marketHash)
        public
        returns (address yesVerse, address noVerse)
    {
        // Deploy with deterministic addresses using market hash as salt
        yesVerse = address(new Verse{salt: marketHash}("Yes", "YES", asset, marketHash));
        noVerse = address(new Verse{salt: marketHash}("No", "NO", asset, marketHash));

        emit VerseTokensCreated(marketHash, asset, yesVerse, noVerse);
    }

    // ============================================
    // CORE OPERATIONS
    // ============================================

    /**
     * @notice Splits ERC20 tokens into equal amounts of YES and NO verse tokens
     * @dev This is the primary mechanism for entering prediction markets
     *      1 token → 1 YES token + 1 NO token
     * @param asset Address of the ERC20 token to split
     * @param amount Amount of tokens to split
     * @param marketHash Unique identifier of the market
     */
    function split(address asset, uint256 amount, bytes32 marketHash) public {
        // Ensure market is still open for trading
        if (resolutions[marketHash] != Resolution.UNRESOLVED) {
            revert InvalidMarketState();
        }

        // Transfer underlying asset from user
        ERC20(asset).transferFrom(msg.sender, address(this), amount);

        // Get or compute verse token addresses
        (address yesVerse, address noVerse) = getVerseAddress(asset, marketHash);

        // Mint equal amounts of YES and NO tokens to user
        Verse(yesVerse).mint(msg.sender, amount);
        Verse(noVerse).mint(msg.sender, amount);

        emit Split(msg.sender, asset, marketHash, amount, yesVerse, noVerse);
    }

    /**
     * @notice Resolves a market based on oracle input or deadline expiry
     * @dev Can be called by anyone. Resolution is permanent and affects redemption values:
     *      - Before deadline: Uses oracle's YES/NO resolution
     *      - After deadline: Automatically resolves to EVEN (50/50 split)
     * @param marketHash Unique identifier of the market to resolve
     */
    function resolve(bytes32 marketHash) public {
        // Ensure market hasn't already been resolved
        if (resolutions[marketHash] != Resolution.UNRESOLVED) {
            revert InvalidMarketState();
        }

        Resolution resolution;
        // Check if deadline has passed
        if (block.timestamp >= markets[marketHash].resolutionDeadline) {
            // Past deadline: split value equally between YES and NO
            resolution = Resolution.EVEN;
        } else {
            // Before deadline: query oracle for resolution
            bool isYes = IOracle(markets[marketHash].oracle).getResolution(marketHash);
            resolution = isYes ? Resolution.YES : Resolution.NO;
        }

        resolutions[marketHash] = resolution;
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
        // Determine if this is a YES or NO token
        // TODO: Optimize by storing this in verse contract
        bool isYes =
            keccak256(abi.encodePacked(ERC20(verse).symbol())) == keccak256(abi.encodePacked("YES"));

        // Get market and asset information
        bytes32 marketHash = Verse(verse).MARKET_HASH();
        address asset = Verse(verse).ASSET();

        // Verify this is a valid verse token
        if (verse != _getVerseAddress(isYes, asset, marketHash)) {
            revert InvalidVerse();
        }

        // Calculate redemption amount based on resolution
        if (resolutions[marketHash] == Resolution.YES && isYes) {
            redeemedAmount = amount; // YES tokens worth 100%
        } else if (resolutions[marketHash] == Resolution.NO && !isYes) {
            redeemedAmount = amount; // NO tokens worth 100%
        } else if (resolutions[marketHash] == Resolution.EVEN) {
            redeemedAmount = amount / 2; // Both worth 50%
        } else {
            revert InvalidResolution();
        }

        // Execute redemption
        ERC20(verse).transferFrom(msg.sender, address(this), amount);
        Verse(verse).burn(address(this), amount);
        ERC20(asset).transfer(msg.sender, redeemedAmount);

        emit Redeemed(msg.sender, verse, marketHash, amount, redeemedAmount);
    }

    /**
     * @notice Combines equal amounts of YES and NO tokens back into the underlying asset
     * @dev Reverse operation of split: 1 YES + 1 NO → 1 underlying token
     *      Only available while market is unresolved
     * @param asset Address of the underlying ERC20 token
     * @param amount Amount of YES and NO tokens to combine
     * @param marketHash Unique identifier of the market
     */
    function combine(address asset, uint256 amount, bytes32 marketHash) public {
        // Ensure market is still open
        if (resolutions[marketHash] != Resolution.UNRESOLVED) {
            revert InvalidMarketState();
        }

        // Get verse token addresses
        (address yesVerse, address noVerse) = getVerseAddress(asset, marketHash);

        // Transfer YES and NO tokens from user
        ERC20(yesVerse).transferFrom(msg.sender, address(this), amount);
        ERC20(noVerse).transferFrom(msg.sender, address(this), amount);

        // Burn the verse tokens
        Verse(yesVerse).burn(address(this), amount);
        Verse(noVerse).burn(address(this), amount);

        // Return the underlying asset to user
        ERC20(asset).transfer(msg.sender, amount);

        emit Combined(msg.sender, asset, marketHash, amount, yesVerse, noVerse);
    }

    // ============================================
    // VIEW FUNCTIONS
    // ============================================
    /**
     * @notice Internal function to compute verse token address using CREATE2
     * @dev Uses EIP-1014 CREATE2 address computation
     * @param yes True for YES verse, false for NO verse
     * @param asset Address of the underlying ERC20 token
     * @param marketHash Unique identifier of the market
     * @return Computed address of the verse token
     */
    function _getVerseAddress(bool yes, address asset, bytes32 marketHash)
        internal
        view
        returns (address)
    {
        // Construct initialization code with constructor parameters
        bytes memory initCode = abi.encodePacked(
            type(Verse).creationCode,
            abi.encode(yes ? "Yes" : "No", yes ? "YES" : "NO", asset, marketHash)
        );

        // Compute CREATE2 address
        return address(
            uint160(
                uint256(
                    keccak256(
                        abi.encodePacked(
                            bytes1(0xff), // CREATE2 prefix
                            address(this), // Deployer address
                            marketHash, // Salt
                            keccak256(initCode) // Init code hash
                        )
                    )
                )
            )
        );
    }

    /**
     * @notice Computes the deterministic addresses for YES and NO verse tokens
     * @dev Can be called before tokens are deployed to know their future addresses
     * @param asset Address of the underlying ERC20 token
     * @param marketHash Unique identifier of the market
     * @return yesVerse Computed address of the YES verse token
     * @return noVerse Computed address of the NO verse token
     */
    function getVerseAddress(address asset, bytes32 marketHash)
        public
        view
        returns (address yesVerse, address noVerse)
    {
        yesVerse = _getVerseAddress(true, asset, marketHash);
        noVerse = _getVerseAddress(false, asset, marketHash);
    }

    /**
     * @notice Verifies if an address is a valid verse token created by this contract
     * @dev Uses CREATE2 address computation to verify authenticity
     * @param verse Address to verify
     * @return True if the address is a valid verse token
     */
    function isVerse(address verse) public view returns (bool) {
        // Get verse token properties
        bytes32 marketHash = Verse(verse).MARKET_HASH();
        address asset = Verse(verse).ASSET();

        // Determine if YES or NO token
        // TODO: Optimize by adding a method to Verse contract
        bool isYes =
            keccak256(abi.encodePacked(ERC20(verse).symbol())) == keccak256(abi.encodePacked("YES"));

        // Verify address matches expected CREATE2 address
        return verse == _getVerseAddress(isYes, asset, marketHash);
    }
}
