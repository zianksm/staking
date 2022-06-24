const { assert } = require("chai");

const STK = artifacts.require("StakingToken");
let address;

function deploy() {
  let token = STK.deployed();

  return token;
}

contract("StakingToken", async (accounts) => {
  it("total supply", async () => {

    let token = await deploy()
    let supply = await token.getTotalSupply();

    assert.equal(
      supply.toNumber(),
      5_000_000_000_000,
      "supply isn't the same as in migrations"
    );
  });

  it("name", async () => {

    token = await deploy()
    let name = await token.getName();

    assert.equal(name, "StakingToken", "the name is wrong");
  });

  it("symbols", async () => {

    token = await deploy();
    let symbol = await token.getSymbol();

    assert.equal(symbol, "STK","symbol is wrong");
  })

  it("decimals", async () => {

    token = await deploy();
    let decimal = await token.getDecimal();

    assert.equal(decimal, 18,"decimal is wrong, decimal is");
  })
});
