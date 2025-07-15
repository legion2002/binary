// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Test} from "forge-std/Test.sol";
import {Verse} from "../src/Verse.sol";
import {MockERC20} from "lib/solady/test/utils/mocks/MockERC20.sol";

contract VerseTest is Test {
    Verse public yesVerse;
    Verse public noVerse;
    MockERC20 public asset;
    
    address public owner = makeAddr("owner");
    address public alice = makeAddr("alice");
    address public bob = makeAddr("bob");
    
    bytes32 public constant MARKET_HASH = keccak256("test_market");
    
    function setUp() public {
        asset = new MockERC20("Test Asset", "TEST", 18);
        
        vm.startPrank(owner);
        yesVerse = new Verse("Yes", "YES", address(asset), MARKET_HASH);
        noVerse = new Verse("No", "NO", address(asset), MARKET_HASH);
        vm.stopPrank();
    }
    
    function testConstructor() public {
        assertEq(yesVerse.name(), "Yes");
        assertEq(yesVerse.symbol(), "YES");
        assertEq(yesVerse.ASSET(), address(asset));
        assertEq(yesVerse.MARKET_HASH(), MARKET_HASH);
        assertEq(yesVerse.owner(), owner);
        
        assertEq(noVerse.name(), "No");
        assertEq(noVerse.symbol(), "NO");
        assertEq(noVerse.ASSET(), address(asset));
        assertEq(noVerse.MARKET_HASH(), MARKET_HASH);
        assertEq(noVerse.owner(), owner);
    }
    
    function testMintAsOwner() public {
        uint256 mintAmount = 100e18;
        
        vm.prank(owner);
        yesVerse.mint(alice, mintAmount);
        
        assertEq(yesVerse.balanceOf(alice), mintAmount);
        assertEq(yesVerse.totalSupply(), mintAmount);
    }
    
    function testMintAsNonOwner() public {
        vm.prank(alice);
        vm.expectRevert(); // Ownable should revert
        yesVerse.mint(alice, 100e18);
    }
    
    function testBurnAsOwner() public {
        uint256 mintAmount = 100e18;
        uint256 burnAmount = 40e18;
        
        vm.prank(owner);
        yesVerse.mint(alice, mintAmount);
        
        vm.prank(owner);
        yesVerse.burn(alice, burnAmount);
        
        assertEq(yesVerse.balanceOf(alice), mintAmount - burnAmount);
        assertEq(yesVerse.totalSupply(), mintAmount - burnAmount);
    }
    
    function testBurnAsNonOwner() public {
        vm.prank(owner);
        yesVerse.mint(alice, 100e18);
        
        vm.prank(alice);
        vm.expectRevert(); // Ownable should revert
        yesVerse.burn(alice, 50e18);
    }
    
    function testERC20BasicFunctionality() public {
        uint256 mintAmount = 100e18;
        uint256 transferAmount = 30e18;
        
        // Mint to alice
        vm.prank(owner);
        yesVerse.mint(alice, mintAmount);
        
        // Alice transfers to bob
        vm.prank(alice);
        yesVerse.transfer(bob, transferAmount);
        
        assertEq(yesVerse.balanceOf(alice), mintAmount - transferAmount);
        assertEq(yesVerse.balanceOf(bob), transferAmount);
    }
    
    function testERC20Approve() public {
        uint256 mintAmount = 100e18;
        uint256 approveAmount = 50e18;
        uint256 transferAmount = 30e18;
        
        // Mint to alice
        vm.prank(owner);
        yesVerse.mint(alice, mintAmount);
        
        // Alice approves bob
        vm.prank(alice);
        yesVerse.approve(bob, approveAmount);
        
        assertEq(yesVerse.allowance(alice, bob), approveAmount);
        
        // Bob transfers from alice
        vm.prank(bob);
        yesVerse.transferFrom(alice, bob, transferAmount);
        
        assertEq(yesVerse.balanceOf(alice), mintAmount - transferAmount);
        assertEq(yesVerse.balanceOf(bob), transferAmount);
        assertEq(yesVerse.allowance(alice, bob), approveAmount - transferAmount);
    }
    
    function testOwnershipTransfer() public {
        address newOwner = makeAddr("newOwner");
        
        vm.prank(owner);
        yesVerse.transferOwnership(newOwner);
        
        assertEq(yesVerse.owner(), newOwner);
        
        // Old owner can't mint anymore
        vm.prank(owner);
        vm.expectRevert();
        yesVerse.mint(alice, 100e18);
        
        // New owner can mint
        vm.prank(newOwner);
        yesVerse.mint(alice, 100e18);
        assertEq(yesVerse.balanceOf(alice), 100e18);
    }
    
    function testRenounceOwnership() public {
        vm.prank(owner);
        yesVerse.renounceOwnership();
        
        assertEq(yesVerse.owner(), address(0));
        
        // No one can mint after renouncing
        vm.prank(owner);
        vm.expectRevert();
        yesVerse.mint(alice, 100e18);
    }
    
    function testMultipleMintsBurns() public {
        uint256[] memory amounts = new uint256[](3);
        amounts[0] = 50e18;
        amounts[1] = 30e18;
        amounts[2] = 20e18;
        
        address[] memory recipients = new address[](3);
        recipients[0] = alice;
        recipients[1] = bob;
        recipients[2] = makeAddr("charlie");
        
        // Multiple mints
        vm.startPrank(owner);
        for (uint256 i = 0; i < recipients.length; i++) {
            yesVerse.mint(recipients[i], amounts[i]);
        }
        
        // Check balances and total supply
        uint256 totalMinted;
        for (uint256 i = 0; i < recipients.length; i++) {
            assertEq(yesVerse.balanceOf(recipients[i]), amounts[i]);
            totalMinted += amounts[i];
        }
        assertEq(yesVerse.totalSupply(), totalMinted);
        
        // Burn from each
        uint256 burnAmount = 10e18;
        for (uint256 i = 0; i < recipients.length; i++) {
            yesVerse.burn(recipients[i], burnAmount);
        }
        vm.stopPrank();
        
        // Check final balances
        for (uint256 i = 0; i < recipients.length; i++) {
            assertEq(yesVerse.balanceOf(recipients[i]), amounts[i] - burnAmount);
        }
        assertEq(yesVerse.totalSupply(), totalMinted - (burnAmount * recipients.length));
    }
    
    // Fuzz tests
    function testFuzzMintBurn(address recipient, uint256 mintAmount, uint256 burnAmount) public {
        vm.assume(recipient != address(0));
        mintAmount = bound(mintAmount, 1, type(uint128).max);
        burnAmount = bound(burnAmount, 0, mintAmount);
        
        vm.prank(owner);
        yesVerse.mint(recipient, mintAmount);
        assertEq(yesVerse.balanceOf(recipient), mintAmount);
        
        vm.prank(owner);
        yesVerse.burn(recipient, burnAmount);
        assertEq(yesVerse.balanceOf(recipient), mintAmount - burnAmount);
        assertEq(yesVerse.totalSupply(), mintAmount - burnAmount);
    }
    
    function testFuzzTransfer(uint256 mintAmount, uint256 transferAmount) public {
        mintAmount = bound(mintAmount, 1, type(uint128).max);
        transferAmount = bound(transferAmount, 0, mintAmount);
        
        vm.prank(owner);
        yesVerse.mint(alice, mintAmount);
        
        vm.prank(alice);
        yesVerse.transfer(bob, transferAmount);
        
        assertEq(yesVerse.balanceOf(alice), mintAmount - transferAmount);
        assertEq(yesVerse.balanceOf(bob), transferAmount);
        assertEq(yesVerse.totalSupply(), mintAmount);
    }
    
    function testFuzzApproveTransferFrom(
        uint256 mintAmount,
        uint256 approveAmount,
        uint256 transferAmount
    ) public {
        mintAmount = bound(mintAmount, 1, type(uint128).max);
        approveAmount = bound(approveAmount, 0, mintAmount);
        transferAmount = bound(transferAmount, 0, approveAmount);
        
        vm.prank(owner);
        yesVerse.mint(alice, mintAmount);
        
        vm.prank(alice);
        yesVerse.approve(bob, approveAmount);
        
        vm.prank(bob);
        yesVerse.transferFrom(alice, bob, transferAmount);
        
        assertEq(yesVerse.balanceOf(alice), mintAmount - transferAmount);
        assertEq(yesVerse.balanceOf(bob), transferAmount);
        assertEq(yesVerse.allowance(alice, bob), approveAmount - transferAmount);
    }
}