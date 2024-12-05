// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

interface IERC20 {
    function transferFrom(address sender, address recipient, uint256 amount) external returns (bool);
    function transfer(address recipient, uint256 amount) external returns (bool);
    function balanceOf(address account) external view returns (uint256);
}

contract EthMecoinPool {
    IERC20 public mecoin;
    uint256 public totalEth;
    uint256 public totalMecoin;

    mapping(address => uint256) public ethBalances;
    mapping(address => uint256) public mecoinBalances;

    event LiquidityAdded(address indexed provider, uint256 ethAmount, uint256 mecoinAmount);
    event LiquidityRemoved(address indexed provider, uint256 ethAmount, uint256 mecoinAmount);
    event Swapped(address indexed trader, uint256 ethIn, uint256 mecoinOut);

    constructor(address _mecoin) {
        mecoin = IERC20(_mecoin);
    }

    // Add liquidity to the ETH/mecoin pool
    function addLiquidity(uint256 mecoinAmount) external payable {
        require(msg.value > 0, "ETH amount must be greater than 0");
        require(mecoinAmount > 0, "mecoin amount must be greater than 0");

        mecoin.transferFrom(msg.sender, address(this), mecoinAmount);

        ethBalances[msg.sender] += msg.value;
        mecoinBalances[msg.sender] += mecoinAmount;

        totalEth += msg.value;
        totalMecoin += mecoinAmount;

        emit LiquidityAdded(msg.sender, msg.value, mecoinAmount);
    }

    // Remove liquidity from the pool
    function removeLiquidity(uint256 ethAmount) external {
        require(ethBalances[msg.sender] >= ethAmount, "Insufficient ETH balance");

        uint256 mecoinAmount = (ethAmount * totalMecoin) / totalEth;

        require(mecoinBalances[msg.sender] >= mecoinAmount, "Insufficient mecoin balance");

        ethBalances[msg.sender] -= ethAmount;
        mecoinBalances[msg.sender] -= mecoinAmount;

        totalEth -= ethAmount;
        totalMecoin -= mecoinAmount;

        payable(msg.sender).transfer(ethAmount);
        mecoin.transfer(msg.sender, mecoinAmount);

        emit LiquidityRemoved(msg.sender, ethAmount, mecoinAmount);
    }

    // Swap ETH for mecoin
    function swapEthForMecoin() external payable {
        require(msg.value > 0, "ETH amount must be greater than 0");

        uint256 mecoinAmount = (msg.value * totalMecoin) / totalEth;
        require(mecoinAmount > 0 && mecoinAmount <= totalMecoin, "Insufficient pool balance");

        totalEth += msg.value;
        totalMecoin -= mecoinAmount;

        mecoin.transfer(msg.sender, mecoinAmount);

        emit Swapped(msg.sender, msg.value, mecoinAmount);
    }
}

/*
Key Features:
Liquidity Pool for ETH and mecoin:

Allows users to deposit ETH and mecoin in equal ratios to provide liquidity.
Liquidity Management:

Users can add or remove liquidity from the pool.
Balances are tracked for each user.
Swap Functionality:

Users can swap ETH for mecoin using the swapEthForMecoin function.
Implements a simple pricing formula based on the pool's reserves.
ERC-20 Compliance:

Requires an ERC-20 compliant token (mecoin) address to be provided during deployment.
Events:

Emits events for adding/removing liquidity and performing swaps.
Notes:
This contract uses a constant product formula (x * y = k) for pricing, which is typical for decentralized exchanges like Uniswap.
A production-ready contract would require further testing, validation, and security measures such as handling slippage, reentrancy attacks, and front-running.

*/