// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {ERC20} from "solady/tokens/ERC20.sol";
import {Ownable} from "solady/auth/Ownable.sol";

contract Verse is ERC20, Ownable {
    string private _name;
    string private _symbol;

    address public immutable ASSET;
    bytes32 public immutable MARKET_HASH;

    constructor(string memory name_, string memory symbol_, address asset, bytes32 marketHash) {
        _name = name_;
        _symbol = symbol_;
        ASSET = asset;
        MARKET_HASH = marketHash;

        _initializeOwner(msg.sender);
    }

    function name() public view virtual override returns (string memory) {
        return _name;
    }

    function symbol() public view virtual override returns (string memory) {
        return _symbol;
    }

    function mint(address to, uint256 amount) public onlyOwner {
        _mint(to, amount);
    }

    function burn(address from, uint256 amount) public onlyOwner {
        _burn(from, amount);
    }
}
