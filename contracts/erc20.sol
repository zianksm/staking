// SPDX-License-Identifier: MIT
pragma solidity >=0.5.0 <0.9.0;

import "./staking.sol";

contract StakingToken is Stakable {
    // required variable for the token operation

    uint256 private totalSupply;
    uint8 private decimals;
    string private name;
    string private symbol;
    address private owner;

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
        owner = msg.sender;

        emit Transfer(address(0), msg.sender, totalSupply);
    }

    function getBlockTime() external view returns (uint256) {
        return block.timestamp;
    }

    function getOwner() external view returns (address) {
        return owner;
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

    function getBalance(address account) external view returns (uint256) {
        return balances[account];
    }

    function getContractAddress() external view returns (address) {
        address contractAddress = address(this);

        return contractAddress;
    }

    function _mint(address account, uint256 amount) internal {
        require(account != address(0), "cannot mint into zero address");

        totalSupply = totalSupply + amount;
        balances[account] = balances[account] + amount;

        emit Transfer(address(0), account, amount);
    }

    function mint(address account, uint256 amount)
        external
        onlyOwner
        returns (bool)
    {
        _mint(account, amount);

        return true;
    }

    function _burn(address account, uint256 amount) internal {
        require(account != address(0), "cannot burn into zero address");
        require(balances[account] >= amount, "not enough token to burn");

        totalSupply = totalSupply - amount;
        balances[account] = balances[account] - amount;

        emit Transfer(account, address(0), amount);
    }

    function burn(address account, uint256 amount)
        external
        onlyOwner
        returns (bool)
    {
        _burn(account, amount);

        return true;
    }

    function transferLogic(
        address sender,
        address receipient,
        uint256 amount
    ) internal {
        require(
            receipient != address(0),
            "transfer to zero address is prohibited"
        );
        require(balances[sender] >= amount, "not enough token to transfer");

        StakingSummary memory userStakes = hasStake(sender);

        require(
            userStakes.totalAmount + amount <= balances[sender],
            "not enough balance due to remaining token is being staked"
        );

        balances[sender] = balances[sender] - amount;
        balances[receipient] = balances[receipient] + amount;

        emit Transfer(sender, receipient, amount);
    }

    function transfer(address receipient, uint256 amount)
        external
        returns (bool)
    {
        transferLogic(msg.sender, receipient, amount);

        return true;
    }

    function stake(uint256 amount) external returns (bool) {
        require(
            amount <= balances[msg.sender],
            "can't stake more than you own"
        );
        StakingSummary memory userStakes = hasStake(msg.sender);
        require(
            userStakes.totalAmount + amount <= balances[msg.sender],
            "not enough balance due to remaining token is being staked"
        );

        stakeLogic(amount);

        return true;
    }

    function widthdrawStake(uint256 amount, uint256 stake_index)
        external
        returns (bool)
    {
        uint256 amountToMint = withdrawLogic(amount, stake_index);

        //change to transfer from reserve account later
        _mint(msg.sender, amountToMint);

        return true;
    }

    //TODO: -make staking logic flow and add documentation
}
