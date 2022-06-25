// SPDX-License-Identifier: MIT
pragma solidity >=0.5.0 <0.9.0;

contract StakingToken {
    // required variable for the token operation

    uint256 private totalSupply;
    uint8 private decimals;
    string private name;
    string private symbol;

    mapping(address => uint256) private balances;

    event Transfer(address indexed from, address indexed to, uint256 value);

    constructor(
        string memory tokenName,
        string memory tokenSymbol,
        uint256 tokenTotalSupply,
        uint8 tokenDecimals
    ) {
        name = tokenName;
        symbol = tokenSymbol;
        totalSupply = tokenTotalSupply;
        decimals = tokenDecimals;

        balances[msg.sender] = totalSupply;

        emit Transfer(address(0), msg.sender, totalSupply);
    }

    function getName() external view returns (string memory) {
        return name;
    }

    function getSymbol() external view returns (string memory) {
        return symbol;
    }

    function getTotalSupply() external view returns (uint256) {
        return totalSupply;
    }

    function getDecimal() external view returns (uint8) {
        return decimals;
    }

    function getBalance(address account) external view returns (uint256){
        return balances[account];
    }

    function getContractAddress() external view returns (address){
        address contractAddress = address(this);

        return contractAddress;
    }

    function mint(address account, uint256 amount)  public{
        require(account != address(0),"cannot mint into zero address");

        totalSupply= totalSupply + amount;
        balances[account] = balances[account] + amount;

        emit Transfer(address(0), account, amount);
    }

    function burn(address account, uint256 amount) public {
        require(account != address(0),"cannot mint into zero address");
        require(balances[account] >= amount, "balance is not enough to burn");

        totalSupply = totalSupply - amount;
        balances[account] = balances[account] - amount;

        emit Transfer(account, address(0), amount);

    }
}
