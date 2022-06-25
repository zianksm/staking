const { assert } = require("chai");

const STK = artifacts.require("StakingToken");
let address;

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
    token = await deploy();
    let name = await token.getName();

    assert.equal(name, "StakingToken", "the name is wrong");
  });

  it("symbols", async () => {
    token = await deploy();
    let symbol = await token.getSymbol();

    assert.equal(symbol, "STK", "symbol is wrong");
  });

  it("decimals", async () => {
    token = await deploy();
    let decimal = await token.getDecimal();

    assert.equal(decimal, 18, "decimal is wrong, decimal is");
  });

  it("address", async () => {
    token = await deploy();
    let contractAddress = await token.getContractAddress();

    assert.equal(
      contractAddress.toString(),
      "0x8CdaF0CD259887258Bc13a92C0a6dA92698644C0",
      "wrong contract address"
    );
  });

  it("minting", async () => {
    token = await deploy();


    let beforeMintTotalSupply = await token.getTotalSupply();
    let mint = await token.mint(accounts[1], 100)
    let accountBalance = await token.getBalance(accounts[1]);
    let afterMintTotalSupply = await token.getTotalSupply();

    assert.equal(100, accountBalance, "after minting to balance should be 100");
    assert.equal(afterMintTotalSupply.toNumber(), beforeMintTotalSupply.toNumber() + accountBalance.toNumber(),"supply doesnt match minting amount!");

    
    try{
      let zeroAddressMint = token.mint("0x0000000000000000000000000000000000000000", 100)
   }catch(error){
     assert.equal(error.reason,"cannot mint into zero address", "fail to mint to zero address");
   }
  });
  //todo : add burn function test
});
