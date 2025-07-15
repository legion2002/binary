// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {ERC20} from "solady/tokens/ERC20.sol";
import {Ownable} from "solady/auth/Ownable.sol";

/**
 * @title Verse
 * @author Binary Protocol
 * @notice ERC20 token representing a specific outcome (YES or NO) for an asset in a prediction market
 * @dev Each Verse token is uniquely tied to:
 *      - A specific market (identified by MARKET_HASH)
 *      - A specific asset (e.g., WETH, USDC)
 *      - A specific outcome (YES or NO)
 *      
 *      Key properties:
 *      - Minting: Only by MultiVerse contract during split operations
 *      - Burning: Only by MultiVerse contract during combine/redeem operations
 *      - Value: Determined by market resolution (1:1 for winning side, 0 for losing side)
 *      - Trading: Fully transferable ERC20, can be traded on DEXs
 *      
 *      Example: YES_WETH for "Will FOCIL be included?" is worth:
 *      - 1 WETH if FOCIL is included (YES resolution)
 *      - 0 WETH if FOCIL is not included (NO resolution)
 *      - 0.5 WETH if market expires (EVEN resolution)
 */
contract Verse is ERC20, Ownable {
    // ============================================
    // STATE VARIABLES
    // ============================================
    
    /// @notice Token name (e.g., "Yes" or "No")
    string private _name;
    
    /// @notice Token symbol (e.g., "YES" or "NO")
    string private _symbol;

    /// @notice The underlying ERC20 asset this verse token represents
    /// @dev Immutable after deployment, links to assets like WETH, USDC, etc.
    address public immutable ASSET;
    
    /// @notice The market this verse token belongs to
    /// @dev Immutable after deployment, computed as keccak256(questionHash, deadline, oracle)
    bytes32 public immutable MARKET_HASH;

    // ============================================
    // CONSTRUCTOR
    // ============================================
    
    /**
     * @notice Deploys a new verse token for a specific market outcome
     * @dev Only deployed by MultiVerse contract using CREATE2 for deterministic addresses
     * @param name_ Token name ("Yes" or "No")
     * @param symbol_ Token symbol ("YES" or "NO")
     * @param asset Address of the underlying ERC20 asset
     * @param marketHash Unique identifier of the prediction market
     */
    constructor(string memory name_, string memory symbol_, address asset, bytes32 marketHash) {
        _name = name_;
        _symbol = symbol_;
        ASSET = asset;
        MARKET_HASH = marketHash;

        // Set MultiVerse as owner for minting/burning privileges
        _initializeOwner(msg.sender);
    }

    // ============================================
    // ERC20 METADATA
    // ============================================
    
    /**
     * @notice Returns the name of the token
     * @return Token name (e.g., "Yes" or "No")
     */
    function name() public view virtual override returns (string memory) {
        return _name;
    }

    /**
     * @notice Returns the symbol of the token
     * @return Token symbol (e.g., "YES" or "NO")
     */
    function symbol() public view virtual override returns (string memory) {
        return _symbol;
    }

    // ============================================
    // PRIVILEGED FUNCTIONS
    // ============================================
    
    /**
     * @notice Mints new verse tokens
     * @dev Only callable by MultiVerse contract during split operations
     * @param to Address to receive the minted tokens
     * @param amount Amount of tokens to mint
     */
    function mint(address to, uint256 amount) public onlyOwner {
        _mint(to, amount);
    }

    /**
     * @notice Burns verse tokens
     * @dev Only callable by MultiVerse contract during combine/redeem operations
     * @param from Address to burn tokens from
     * @param amount Amount of tokens to burn
     */
    function burn(address from, uint256 amount) public onlyOwner {
        _burn(from, amount);
    }
}
