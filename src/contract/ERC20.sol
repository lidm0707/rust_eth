// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

abstract contract IERC20 {
    // Events ตามมาตรฐาน ERC-20
    event Transfer(address indexed _from, address indexed _to, uint256 _value);
    event Approval(
        address indexed _owner,
        address indexed _spender,
        uint256 _value
    );

    // ฟังก์ชัน ERC-20 มาตรฐาน
    function name() external view virtual returns (string memory);
    function symbol() external view virtual returns (string memory);
    function decimals() external view virtual returns (uint8);
    function totalSupply() external view virtual returns (uint256);
    function balanceOf(address account) external view virtual returns (uint256);

    function transfer(
        address recipient,
        uint256 amount
    ) public virtual returns (bool);
    function approve(
        address spender,
        uint256 amount
    ) public virtual returns (bool);
    function transferFrom(
        address sender,
        address recipient,
        uint256 amount
    ) public virtual returns (bool);
    function allowance(
        address owner,
        address spender
    ) public view virtual returns (uint256);
}

abstract contract ERC20 is IERC20 {
    string _name;
    string _symbol;
    uint256 _totalSupply;
    mapping(address => uint256) _balance;
    mapping(address => mapping(address => uint256)) _allowances;

    constructor(string memory name_, string memory symbol_) {
        _name = name_;
        _symbol = symbol_;
    }

    function name() public view override returns (string memory) {
        return _name;
    }

    function symbol() public view override returns (string memory) {
        return _symbol;
    }
    function decimals() public pure override returns (uint8) {
        return 18;
    }
    function totalSupply() public view override returns (uint256) {
        return _totalSupply;
    }
    function balanceOf(address owner) public view override returns (uint256) {
        return _balance[owner];
    }

    function transfer(
        address to,
        uint256 amount
    ) public override returns (bool) {
        _transfer(msg.sender, to, amount);
        return true;
    }

    // == private function == //
    function _transfer(
        address from,
        address to,
        uint256 amount
    ) internal returns (bool) {
        require(from != address(0), "tranfer from zero address");
        require(to != address(0), "tranfer to zero address");
        require(amount <= _balance[from], "tranfer amount exceed balance");
        _balance[from] -= amount;
        _balance[to] -= amount;

        emit Transfer(from, to, amount);
        return true;
    }
    function approve(
        address spender,
        uint256 amount
    ) public override returns (bool) {
        _approve(msg.sender, spender, amount);
        return true;
    }

    function _approve(
        address owner,
        address spender,
        uint256 amount
    ) internal returns (bool) {
        require(owner != address(0), "approve from zero address");
        require(spender != address(0), "approve spender zero address");
        _allowances[owner][spender] = amount;
        emit Approval(owner, spender, amount);
        return true;
    }

    function allowance(
        address owner,
        address spender
    ) public view override returns (uint256) {
        return _allowances[owner][spender];
    }

    function transferFrom(
        address from,
        address to,
        uint256 amount
    ) public override returns (bool) {
        if (from != msg.sender) {
            uint256 allowanceAmount = _allowances[from][msg.sender];
            require(
                amount <= allowanceAmount,
                "transfer amount exceeds allowance"
            );
            _approve(from, msg.sender, allowanceAmount - amount);
        }
        _transfer(from, to, amount);
        return true;
    }

    function _mint(address to, uint256 amount) internal {
        require(to != address(0), "mint to zero address");
        _balance[to] += amount;
        _totalSupply += amount;

        emit Transfer(address(0), to, amount);
    }

    function _burn(address from, uint256 amount) internal {
        require(from != address(0), "burn zero address");
        require(amount <= _balance[from],"burn amount exceeds balance");

        _balance[from] -= amount;
        _totalSupply -= amount;
        emit Transfer(from, address(0), amount);
    }
}


// use contract nameCOIN is ERC20{}

contract MooCoin is ERC20 {
    constructor() ERC20("MooCoin","moo") {

    }

    function deposit() public payable{
        require(msg.value > 0 , "amount is zero");
        _mint(msg.sender,msg.value);
    }

    function  withdraw(uint256 amount) public{
        require(amount > 0 && amount <= _balance[msg.sender],"withdraw amount is zero");
        payable(msg.sender).transfer(amount);
        _burn(msg.sender, amount);
        
    }
}