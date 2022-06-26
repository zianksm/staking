// SPDX-License-Identifier: MIT
pragma solidity >=0.5.0 <0.9.0;

contract Stakable {
    constructor() {
        // push to avoid index -1 bug
        stakeholders.push();
    }

    struct Stake {
        address user;
        uint256 amount;
        uint256 timestamp;
    }

    struct Stakeholder {
        address user;
        Stake[] stakes_address;
    }

    Stakeholder[] internal stakeholders;
    // mapping used to track user index in stakeholder array so we don't reiterate the whole array
    mapping(address => uint256) internal stakes;

    event Staked(
        address indexed user,
        uint256 amount,
        uint256 index,
        uint256 timestamp
    );

    function addStakeHolder(address staker) internal returns (uint256) {
        stakeholders.push();

        uint256 userIndex = stakeholders.length - 1;

        stakeholders[userIndex].user = staker;
        stakes[staker] = userIndex;
        return userIndex;

    }

    function stakeLogic(uint256 amount) internal{
        require(amount >= 0, "can't stake 0 token");

        uint256 index = stakes[msg.sender];
        uint256 timestamp = block.timestamp;

        if (index == 0){
            index = addStakeHolder(msg.sender);
        }

        stakeholders[index].stakes_address.push(Stake(msg.sender, amount,timestamp));

        emit Staked(msg.sender, amount, index, timestamp);
    }
}
