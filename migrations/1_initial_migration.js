const Token = artifacts.require("StakingToken");

module.exports = async function (deployer, network, account) {
  await deployer.deploy(Token,0xf98181b4b3bc77cDC84a0F71ec789eA4F5f3150Bn,"StakingToken","STK",5_000_000_000_000, 18);

  const token = await Token.deployed()

};
