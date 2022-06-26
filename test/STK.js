const { assert } = require("chai");

const STK = artifacts.require("StakingToken");

function deploy() {
  let token = STK.deployed();

  return token;
}

contract("StakingToken", async (accounts) => {
  it("total supply", async () => {
    let token = await deploy();
    let supply = await token.getTotalSupply();

    assert.equal(
      supply.toNumber(),
      5_000_000_000_000,
      "supply isn't the same as in migrations"
    );
  });

  it("name", async () => {
    let token = await deploy();

    let name = await token.getName();

    assert.equal(name, "StakingToken", "the name is wrong");
  });

  it("symbols", async () => {
    let token = await deploy();

    let symbol = await token.getSymbol();

    assert.equal(symbol, "STK", "symbol is wrong");
  });

  it("decimals", async () => {
    let token = await deploy();

    let decimal = await token.getDecimal();

    assert.equal(decimal, 18, "decimal is wrong, decimal is");
  });

  it("address", async () => {
    let token = await deploy();

    let contractAddress = await token.getContractAddress();

    assert.equal(
      contractAddress.toString(),
      "0x8CdaF0CD259887258Bc13a92C0a6dA92698644C0",
      "wrong contract address"
    );
  });

  it("minting", async () => {
    let token = await deploy();

    let amount = 100;

    let beforeMintTotalSupply = await token.getTotalSupply();
    let mint = await token.mint(accounts[1], amount);
    let accountBalance = await token.getBalance(accounts[1]);
    let afterMintTotalSupply = await token.getTotalSupply();

    assert.equal(
      amount,
      accountBalance,
      "after minting to balance should be 100"
    );
    assert.equal(
      afterMintTotalSupply.toNumber(),
      beforeMintTotalSupply.toNumber() + accountBalance.toNumber(),
      "supply doesnt match minting amount!"
    );

    try {
      await token.mint("0x0000000000000000000000000000000000000000", amount);
    } catch (error) {
      assert.equal(
        error.reason,
        "cannot mint into zero address",
        "fail to mint to zero address"
      );
    }
  });

  it("burning", async () => {
    let token = await deploy();

    let amount = 100;

    let beforeBurnTotalSupply = await token.getTotalSupply();
    let beforeBurnBalance = await token.getBalance(accounts[1]);

    let burn = await token.burn(accounts[1], amount);
    let afterBurnTotalSupply = await token.getTotalSupply();
    let afterBurnBalance = await token.getBalance(accounts[1]);

    assert.equal(
      afterBurnTotalSupply,
      beforeBurnTotalSupply - amount,
      "expected supply doesn't match up"
    );
    assert.equal(
      afterBurnBalance,
      beforeBurnBalance - amount,
      "expected balance doesn't match up"
    );

    try {
      await token.burn("0x0000000000000000000000000000000000000000", amount);
    } catch (error) {
      assert.equal(
        error.reason,
        "cannot burn into zero address",
        "fail to burn into zero adddress"
      );
    }

    try {
      await token.burn(accounts[1], 200);
    } catch (error) {
      assert.equal(
        error.reason,
        "not enough token to burn",
        "fail to burn token"
      );
    }
  });

  it("transfer", async () => {
    let token = await deploy();

    let amount = 100;

    try {
      await token.transfer(
        "0x0000000000000000000000000000000000000000",
        amount,
        { from: accounts[3] }
      );
    } catch (error) {
      assert.equal(
        error.reason,
        "transfer to zero address is prohibited",
        "fail to to transfer to zero address"
      );
    }

    try {
      await token.transfer(accounts[4], amount, { from: accounts[3] });
    } catch (error) {
      assert.equal(error.reason, "not enough token to transfer");
    }

    let balancesBeforeTransferSender = await token.getBalance(accounts[3]);
    let balancesBeforeTransferRecipient = await token.getBalance(accounts[4]);

    await token.mint(accounts[3], amount);
    let tf = await token.transfer(accounts[4], amount, { from: accounts[3] });

    let balancesAfterTransferSender = await token.getBalance(accounts[3]);
    let balancesAfterTransferRecipient = await token.getBalance(accounts[4]);

    assert.equal(tf.receipt.status, true, "transfer is not successful");
    assert.equal(
      balancesAfterTransferSender.words[0],
      balancesBeforeTransferSender.words[0],
      "expected balance did not match up"
    );
    assert.equal(
      balancesAfterTransferRecipient.words[0],
      balancesBeforeTransferRecipient.words[0] + amount,
      "expected balance did not match up"
    );

    token.burn(accounts[4], amount);
  });
  //todo : add burn function test
});
