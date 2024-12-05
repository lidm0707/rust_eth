// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract SwapPool {
    mapping(address => uint256) public balances;

    // Deposit ETH into the pool
    function deposit() external payable {
        balances[msg.sender] += msg.value;
    }

    // Mock Swap Function
    function swap(address recipient, uint256 amount) external {
        require(balances[msg.sender] >= amount, "Insufficient balance");
        balances[msg.sender] -= amount;
        balances[recipient] += amount;
    }

    // Withdraw ETH from the pool
    function withdraw(uint256 amount) external {
        require(balances[msg.sender] >= amount, "Insufficient balance");
        balances[msg.sender] -= amount;
        payable(msg.sender).transfer(amount);
    }
}
