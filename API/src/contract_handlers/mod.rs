use crate::server_config::Config;
use std::env;
use std::str::FromStr;
use web3::contract::tokens::Tokenizable;
use web3::contract::{Contract, Options};
use web3::types::{Address, H160, U256};
use encoding_rs;

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

    pub async fn balances(account: Address) -> U256 {
        let contract = contract_instance::get_contract().await;
        //let account = Address::from(account);

        let balances:U256 = contract
        .query("getBalance", account, None, Options::default(), None)
        .await
        .unwrap();

        return balances
    }

    pub async fn locked_balance(account: Address) -> U256{
        let contract = contract_instance::get_contract().await;
        let mut locked_balance: U256 = contract
        .query("hasStake", account, None, Options::default(), None)
        .await
        .unwrap();
       
        return locked_balance
    }
}
