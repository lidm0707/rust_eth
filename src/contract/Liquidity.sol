// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "./IERC20.sol";

contract LiquidityPool {
    IERC20 public tokenA;
    IERC20 public tokenB;
    mapping(address => uint256) public liquidity;

    uint256 public reserveA;
    uint256 public reserveB;

    constructor(address _tokenA, address _tokenB) {
        tokenA = IERC20(_tokenA);
        tokenB = IERC20(_tokenB);
    }

    function addLiquidity(uint256 amountA, uint256 amountB) public {
        require(
            tokenA.transferFrom(msg.sender, address(this), amountA),
            "Token A transfer failed"
        );
        require(
            tokenB.transferFrom(msg.sender, address(this), amountB),
            "Token B transfer failed"
        );

        reserveA += amountA;
        reserveB += amountB;
        liquidity[msg.sender] += (amountA + amountB);
    }

    function removeLiquidity(uint256 shares) public {
        uint256 amountA = (shares * reserveA) / totalLiquidity();
        uint256 amountB = (shares * reserveB) / totalLiquidity();

        require(
            tokenA.transfer(msg.sender, amountA),
            "Token A transfer failed"
        );
        require(
            tokenB.transfer(msg.sender, amountB),
            "Token B transfer failed"
        );

        reserveA -= amountA;
        reserveB -= amountB;
        liquidity[msg.sender] -= shares;
    }

    function swap(
        address tokenIn,
        uint256 amountIn
    ) public returns (uint256 amountOut) {
        require(amountIn > 0, "Input amount must be greater than zero");
        require(
            tokenIn == address(tokenA) || tokenIn == address(tokenB),
            "Invalid input token"
        );

        bool isTokenA = tokenIn == address(tokenA);

        // Reserves of the input and output tokens
        uint256 reserveIn = isTokenA ? reserveA : reserveB;
        uint256 reserveOut = isTokenA ? reserveB : reserveA;

        // Apply fee of 0.3% (997/1000)
        uint256 amountInWithFee = (amountIn * 997) / 1000;
        amountOut =
            (reserveOut * amountInWithFee) /
            (reserveIn + amountInWithFee);

        require(amountOut > 0, "Insufficient output amount");

        // Transfer the input token from the user to the pool
        IERC20(tokenIn).transferFrom(msg.sender, address(this), amountIn);

        // Transfer the output token from the pool to the user
        IERC20 outputToken = isTokenA ? tokenB : tokenA;
        outputToken.transfer(msg.sender, amountOut);

        // Update reserves
        if (isTokenA) {
            reserveA += amountIn;
            reserveB -= amountOut;
        } else {
            reserveB += amountIn;
            reserveA -= amountOut;
        }
    }

    function totalLiquidity() public view returns (uint256) {
        return reserveA + reserveB;
    }
}
