mod server_config;

use server_config::Config;
use std::env;
use std::str::FromStr;

use web3::contract::{Contract, Options};
use web3::types::{Address, H160, U256};

#[tokio::main]
async fn main() -> web3::Result<()> {
    dotenv::dotenv().ok();

    let config = Config::from_env().expect("loading server configurations");

    let http = web3::transports::Http::new(&config.network).unwrap();
    let web3s = web3::Web3::new(http);

    let contract_address = Address::from_str(&config.contract).unwrap();
    let contract = Contract::from_json(
        web3s.eth(),
        contract_address,
        include_bytes!("./token.json"),
    )
    .unwrap();

    let total_supply: U256 = contract
        .query("getTotalSupply", (), None, Options::default(), None)
        .await
        .unwrap();

    println!("Total Supply: {}", total_supply);

    Ok(())
}


