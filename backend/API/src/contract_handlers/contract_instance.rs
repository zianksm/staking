use crate::server_config::Config;
use serde_json;
use std::env;
use std::str::FromStr;
use web3::contract::{Contract, Options};
use web3::transports::Http;
use web3::types::{Address, H160, U256};
use std::convert::TryFrom;

pub async fn get_contract_call() -> Contract<Http> {
    let config = Config::from_env_without_tracing().expect("network and contract address");

    let http = web3::transports::Http::new(&config.network).unwrap();
    let web3s = web3::Web3::new(http);

    let contract_address = Address::from_str(&config.contract).unwrap();
    let _contract = Contract::from_json(
        web3s.eth(),
        contract_address,
        include_bytes!("../token.json"),
    )
    .unwrap();

    return _contract;
}

