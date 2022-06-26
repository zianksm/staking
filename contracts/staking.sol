// SPDX-License-Identifier: MIT
pragma solidity >=0.5.0 <0.9.0;

contract Stakable {
    constructor() {
        // push to avoid index -1 big
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
    mapping(address => uint256) internal stakes;

    event Staked(
        address indexed user,
        uint256 amount,
        uint256 index,
        uint256 timestamp,
        Stakeholder
    );
}
