// SPDX-License-Identifier: MIT
pragma solidity >=0.5.0 <0.9.0;

contract Stakable {
    uint256 rewardRate = 10;

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

    function stakeLogic(uint256 amount) internal {
        require(amount >= 0, "can't stake 0 token");

        uint256 index = stakes[msg.sender];
        uint256 timestamp = block.timestamp;

        if (index == 0) {
            index = addStakeHolder(msg.sender);
        }

        stakeholders[index].stakes_address.push(
            Stake(msg.sender, amount, timestamp)
        );

        emit Staked(msg.sender, amount, index, timestamp);
    }

    function withdrawLogic(uint256 amount, uint256 index) internal returns(uint256) {
        // get the uesr index in the array of stakeholders
        uint256 user_index = stakes[msg.sender];

        // get the desired stake for withdrawal
        Stake memory currentStake = stakeholders[user_index].stakes_address[
            index
        ];
        require(
            currentStake.amount >= amount,
            "can't withdraw nore than available balance"
        );

        uint256 reward = calculateReward(currentStake);

        // remove the withdrawed token from the stakes
        currentStake.amount = currentStake.amount - amount;

        if (currentStake.amount == 0) {
            // delete the current stake if the staked amount is 0
            delete stakeholders[user_index].stakes_address[index];
        } else {
            // replace the value of the current staked value with the new staked value after withdrawal
            stakeholders[user_index].stakes_address[index].amount = currentStake
                .amount;

            // reset the stakes timestamp
            stakeholders[user_index].stakes_address[index].timestamp = block
                .timestamp;
        }

        return amount + reward;
    }

    function calculateReward(Stake memory currentStake)
        internal
        view
        returns (uint256)
    {
        // find the durations
        uint256 durationSeconds = block.timestamp - currentStake.timestamp;
        durationSeconds = durationSeconds / 1 hours;

        // multiply by the staking amount(this return 100% yield rate)
        uint256 reward = durationSeconds * currentStake.amount;

        // find the real reward by dividing with the % reward rate
        reward = reward / (rewardRate / 100);

        return reward;
    }
}
