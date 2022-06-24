const Token = artifacts.require("StakingToken");

module.exports = async function (deployer, network, account) {
  await deployer.deploy(Token,"StakingToken","STK",5_000_000_000_000, 18);

  const token = await Token.deployed()

};
