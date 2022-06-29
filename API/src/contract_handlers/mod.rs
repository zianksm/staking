use crate::server_config::Config;
use std::env;
use std::str::FromStr;
use web3::contract::{Contract, Options};
use web3::types::{Address, H160, U256};

mod contract_instance;

pub struct OurContract {
    owner: String,
    address: String,
    name: String,
    symbol: String,
    total_supply: U256,
    decimal: U256,
}

pub struct User {
    balances: U256,
    stakes: Vec<StakingSummary>,
}

pub struct Stake {
    user: String,
    amount: U256,
    timestamp: U256,
    claimable: U256,
}

pub struct StakingSummary {
    total_stakes: U256,
    stakes: Vec<Stake>,
}

impl OurContract {
    pub async fn total_supply() -> U256 {
        let contract = contract_instance::get_contract().await;

        let total_supply: U256 = contract
            .query("getTotalSupply", (), None, Options::default(), None)
            .await
            .unwrap();

        return total_supply;
    }

    /*async fn address() -> String {
        let config = Config::from_env_without_tracing().expect("network and contract address");
    }*/
}
