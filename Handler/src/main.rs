mod server_config;
use server_config::Config;
use std::env;
use std::str::FromStr;
use web3::contract::tokens::{Detokenize, Tokenizable};
use web3::contract::{Contract, Options};
use web3::types::{AccountDiff, Address, H160, U256};
use serde_json;

#[tokio::main]
async fn main() 
{
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
    let account: Address = Address::from_str("0xCCd987705C24aab4e2Fbe01A4BeD025A696DEA99").unwrap();
    let has_stakes:Box< dyn >= contract
        .query("hasStake", account, None, Options::default(), None)
        .await
        .unwrap();
        //let a = serde_json::(has_stakes).unwrap();


    //println!("Total Supply: {:?}", has_stakes);
}
