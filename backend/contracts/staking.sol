// SPDX-License-Identifier: MIT
pragma solidity >=0.5.0 <0.9.0;

contract Stakable {
    // 10% reward rate per hour(make it and the post-fix larger to represent decimals)
    uint256 rewardRate = 8;
    // post-fix to divide as so it will be the same as multiplying with %(make it larger to represent decimals)
    uint256 divideRate = 100;

    address private owner;

    constructor() {
        // push to avoid index -1 bug
        stakeholders.push();
        owner = msg.sender;
    }

    modifier onlyOwner() {
        require(msg.sender == owner);
        _;
    }

    struct Stake {
        address user;
        uint256 amount;
        uint256 timestamp;
        uint256 claimable;
    }

    struct StakingSummary {
        uint256 totalAmount;
        Stake[] stakes;
    }

    struct Stakeholder {
        address user;
        Stake[] StakesAddress;
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

        stakeholders[index].StakesAddress.push(
            Stake(msg.sender, amount, timestamp, 0)
        );

        emit Staked(msg.sender, amount, index, timestamp);
    }

    function withdrawLogic(uint256 amount, uint256 index)
        internal
        returns (uint256)
    {
        // get the uesr index in the array of stakeholders
        uint256 user_index = stakes[msg.sender];

        // get the desired stake for withdrawal
        Stake memory currentStake = stakeholders[user_index].StakesAddress[
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
            delete stakeholders[user_index].StakesAddress[index];
            stakeholders[user_index].StakesAddress[index] = stakeholders[
                user_index
            ].StakesAddress[stakeholders[user_index].StakesAddress.length - 1];

            stakeholders[user_index].StakesAddress.pop();

        } else {
            // replace the value of the current staked value with the new staked value after withdrawal
            stakeholders[user_index].StakesAddress[index].amount = currentStake
                .amount;

            // reset the stakes timestamp
            stakeholders[user_index].StakesAddress[index].timestamp = block
                .timestamp;
        }

        return reward;
    }

    function calculateReward(Stake memory currentStake)
        internal
        view
        returns (uint256)
    {
        // find the durations
        uint256 duration = block.timestamp - currentStake.timestamp;
        duration = duration / 1 hours;

        // multiply by the staking amount(this return 1000% yield rate /token staked)
        uint256 reward = duration * currentStake.amount;

        // find the real reward by dividing with the reward rate to get  the real reward precentage(1000 / your reward rate);
        reward = (reward * rewardRate) / divideRate;

        return reward;
    }

    //function overloading to accept only 1 parameter
    function changeReward(uint256 newRewardRate, uint256 newDivideRate)
        internal
        onlyOwner
    {
        rewardRate = newRewardRate;
        divideRate = newDivideRate;
    }

    function changeRewardRate(uint256 newRewardRate)
        external
        onlyOwner
        returns (bool)
    {
        changeReward(newRewardRate, divideRate);

        return true;
    }

    function changeDivideRate(uint256 newDivideRate)
        external
        onlyOwner
        returns (bool)
    {
        changeReward(rewardRate, newDivideRate);

        return true;
    }

    function hasStake(address staker)
        public
        view
        returns (StakingSummary memory)
    {
        uint256 totalStakeAmount;

        uint256 user_index = stakes[staker];

        // get the stakes array
        StakingSummary memory summary = StakingSummary(
            0,
            stakeholders[user_index].StakesAddress
        );

        // loop through the stakes array calculating reward and the total stake amount
        for (uint256 i = 0; i < summary.stakes.length; i += 1) {
            uint256 reward = calculateReward(summary.stakes[i]);
            summary.stakes[i].claimable = reward;
            totalStakeAmount = totalStakeAmount + summary.stakes[i].amount;
        }

        summary.totalAmount = totalStakeAmount;
        return summary;
    }

    function hasStakePublic(address staker)
        external
        view
        returns (uint256, address[] memory, uint256[]memory, uint256[] memory, uint256[]memory)
    {
        uint256 totalStakeAmount;

        uint256 user_index = stakes[staker];

        // get the stakes array
        StakingSummary memory summary = StakingSummary(
            0,
            stakeholders[user_index].StakesAddress
        );
        address[] memory user_returned = new address[](summary.stakes.length);
        uint256[] memory amount_returned = new uint256[](summary.stakes.length);
        uint256[] memory timestamp_returned = new uint256[](summary.stakes.length);
        uint256[] memory claimable_returned = new uint256[](summary.stakes.length);

        // loop through the stakes array calculating reward and the total stake amount
        for (uint256 i = 0; i < summary.stakes.length; i += 1) {
            uint256 reward = calculateReward(summary.stakes[i]);
            summary.stakes[i].claimable = reward;
            claimable_returned[i] = reward;

            totalStakeAmount = totalStakeAmount + summary.stakes[i].amount;

            user_returned[i] = summary.stakes[i].user;
            amount_returned[i] = summary.stakes[i].amount;
            timestamp_returned[i] = summary.stakes[i].timestamp;


        }

        summary.totalAmount = totalStakeAmount;
        return (summary.totalAmount, user_returned, amount_returned, timestamp_returned, claimable_returned);
    }

}