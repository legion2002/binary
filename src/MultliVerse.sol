// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {ERC20} from "solady/tokens/ERC20.sol";
import {Verse} from "./Verse.sol";
import {IOracle} from "./IOracle.sol";

contract MultiVerse {
    error MarketAlreadyOpened();
    error MarketAlreadyResolved();
    error InvalidVerse();
    error InvalidResolution();

    struct Market {
        // After this time, the market resolves to even between YES and NO.
        uint32 resolutionDeadline;
        // The oracle that will resolve the market.
        address oracle;
        // The hash of the question that the market is about.
        bytes32 questionHash;
    }

    mapping(bytes32 marketHash => Market) public markets;
    mapping(bytes32 marketHash => IOracle.Resolution) public resolutions;

    function open(bytes32 questionHash, uint32 resolutionDeadline, address oracle) public {
        bytes32 marketHash = keccak256(abi.encode(questionHash, resolutionDeadline, oracle));
        if (resolutions[marketHash] != IOracle.Resolution.NULL) {
            revert MarketAlreadyOpened();
        }

        resolutions[marketHash] = IOracle.Resolution.UNRESOLVED;

        markets[marketHash] = Market({
            resolutionDeadline: resolutionDeadline,
            oracle: oracle,
            questionHash: questionHash
        });
    }

    function create(address asset, bytes32 marketHash)
        public
        returns (address yesVerse, address noVerse)
    {
        yesVerse = address(new Verse{salt: marketHash}("Yes", "YES", asset, marketHash));
        noVerse = address(new Verse{salt: marketHash}("No", "NO", asset, marketHash));
    }

    function getVerseAddress(address asset, bytes32 marketHash)
        public
        view
        returns (address yesVerse, address noVerse)
    {
        yesVerse = _getVerseAddress(true, asset, marketHash);
        noVerse = _getVerseAddress(false, asset, marketHash);
    }

    function _getVerseAddress(bool yes, address asset, bytes32 marketHash)
        internal
        view
        returns (address)
    {
        bytes memory initCode = abi.encodePacked(
            type(Verse).creationCode,
            abi.encode(yes ? "Yes" : "No", yes ? "YES" : "NO", asset, marketHash)
        );
        return address(
            uint160(
                uint256(
                    keccak256(
                        abi.encodePacked(
                            bytes1(0xff), address(this), marketHash, keccak256(initCode)
                        )
                    )
                )
            )
        );
    }

    function split(
        address asset,
        address depositor,
        address receiver,
        uint256 amount,
        bytes32 marketHash
    ) public {
        // Pull money
        ERC20(asset).transferFrom(depositor, address(this), amount);

        (address yesVerse, address noVerse) = getVerseAddress(asset, marketHash);

        // Mint split assets
        Verse(yesVerse).mint(receiver, amount);
        Verse(noVerse).mint(receiver, amount);
    }

    function resolve(bytes32 marketHash) public {
        if (resolutions[marketHash] != IOracle.Resolution.UNRESOLVED) {
            revert MarketAlreadyResolved();
        }

        IOracle.Resolution resolution =
            IOracle(markets[marketHash].oracle).getResolution(marketHash);
        resolutions[marketHash] = resolution;
    }

    function redeem(address depositor, address receiver, address verse, uint256 amount)
        public
        returns (uint256 redeemedAmount)
    {
        // TODO: Optimize this later
        bool isYes =
            keccak256(abi.encodePacked(ERC20(verse).symbol())) == keccak256(abi.encodePacked("YES"));
        bytes32 marketHash = Verse(verse).MARKET_HASH();
        address asset = Verse(verse).ASSET();
        if (verse != _getVerseAddress(isYes, asset, marketHash)) {
            revert InvalidVerse();
        }

        if (resolutions[marketHash] == IOracle.Resolution.YES && isYes) {
            redeemedAmount = amount;
        } else if (resolutions[marketHash] == IOracle.Resolution.NO && !isYes) {
            redeemedAmount = amount;
        } else if (resolutions[marketHash] == IOracle.Resolution.EVEN) {
            redeemedAmount = amount / 2;
        } else {
            revert InvalidResolution();
        }

        Verse(verse).burn(address(this), redeemedAmount);
        ERC20(verse).transferFrom(depositor, address(this), redeemedAmount);
        ERC20(asset).transfer(receiver, redeemedAmount);
    }

    function isVerse(address verse) public view returns (bool) {
        bytes32 marketHash = Verse(verse).MARKET_HASH();
        address asset = Verse(verse).ASSET();
        // TODO: Optimize this later
        bool isYes =
            keccak256(abi.encodePacked(ERC20(verse).symbol())) == keccak256(abi.encodePacked("YES"));

        return verse == _getVerseAddress(isYes, asset, marketHash);
    }
}
