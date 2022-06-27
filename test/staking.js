const { assert } = require("chai");
const truffleAssert = require("truffle-assertions");
const helpers = require("./helpers/truffleTestHelpers.js");

const STK = artifacts.require("StakingToken");
function deploy() {
  let token = STK.deployed();

  return token;
}

contract("StakingToken", async (accounts) => {
  it("staking", async () => {
    let token = await deploy();

    let owner = accounts[0];

    let stake_amount = 100;

    await token.mint(accounts[0], stake_amount);

    let accountBalance = await token.getBalance(accounts[0]);
    let stakeID = await token.stake(stake_amount, { from: owner });

    truffleAssert.eventEmitted(
      stakeID,
      "Staked",
      (ev) => {
        assert.equal(ev.amount, stake_amount, "staking amount didn't match up");
        assert.equal(ev.index, 1, "stake ID didn't match up");

        return true;
      },
      "stake event should've been emitted"
    );
  });

  it("stake more than balance available", async () => {
    let token = await deploy();

    let owner = accounts[6];
    let stake_amount = 100;

    try {
      await token.stake(stake_amount, { from: owner });
    } catch (error) {
      assert.equal(error.reason, "can't stake more than you own");
    }
  });

  it("index increments", async () => {
    let token = await deploy();

    let stake_amount = 100;

    let owner = accounts[6];
    await token.mint(owner, stake_amount);

    let stakeID = await token.stake(stake_amount, { from: owner });

    truffleAssert.eventEmitted(
      stakeID,
      "Staked",
      (ev) => {
        assert.equal(ev.amount, stake_amount);
        assert.equal(ev.index, 2);

        return true;
      },
      "stake event should've been emitted"
    );
  });

  it("can't withdraw more than staked", async () => {
    let token = await deploy();

    let owner = accounts[0];
    let amount = 250;

    try {
      await token.widthdrawStake(amount, 0, { from: owner });
    } catch (error) {
      assert.equal(
        error.reason,
        "can't withdraw nore than available balance",
        "fail to notice withdrawal"
      );
    }
  });

  it("withdraw 50 from stake", async () => {
    let token = await deploy();

    let owner = accounts[0];
    let index = 0;
    let amount = 50;

    await token.stake(100, { from: owner });

    let summaryBeforeWidthdraw = await token.hasStake(owner);
    let stakeAmountBeforeWidthdraw =
      summaryBeforeWidthdraw.stakes[index].amount;
    let totalAmountBeforeWidthdraw = summaryBeforeWidthdraw.totalAmount;

    await token.widthdrawStake(amount, index, { from: owner });

    let summaryAfterWidthdraw = await token.hasStake(owner);
    let stakeAmountAfterWidthdraw = summaryAfterWidthdraw.stakes[index].amount;
    let totalAmountAfterWidthdraw = summaryAfterWidthdraw.totalAmount;

    assert.equal(
      stakeAmountAfterWidthdraw,
      stakeAmountBeforeWidthdraw - amount,
      "expected staking amount didn't match up"
    );
    assert.equal(
      totalAmountAfterWidthdraw,
      totalAmountBeforeWidthdraw - amount,
      "expected total amount didn't match up"
    );
  });
});
