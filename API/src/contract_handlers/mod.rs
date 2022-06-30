use crate::server_config::Config;
use crate::total_supply;
use core::time;
use encoding_rs;
use serde::{Deserialize, Serialize};
use serde_json;
use std::{env, u8};
use std::str::FromStr;
use tracing::info;
use web3::contract::tokens::{Detokenize, Tokenizable};
use web3::contract::{Contract, Options};
use web3::ethabi::token::Token;
use web3::types::{Address, H160, U256};

mod contract_instance;

#[derive(Clone, Debug, Serialize)]
pub struct OurContract {
    owner: Address,
    address: Address,
    name: String,
    symbol: String,
    _total_supply_: U256,
    decimal: U256,
}

#[derive(Clone, Debug, Serialize)]
pub struct User {
    balances: U256,
    stakes: Vec<StakingSummary>,
}

#[derive(Clone, Debug, Serialize)]
pub struct Stake {
    user: Address,
    amount: U256,
    timestamp: U256,
    claimable: U256,
}

#[derive(Clone, Debug, Serialize)]
pub struct StakingSummary {
    total_stakes: U256,
    stakes: Vec<Stake>,
}

impl Detokenize for Stake {
    fn from_tokens(tokens: Vec<Token>) -> Result<Self, web3::contract::Error> {
        let user: Address = tokens[1].clone().into_address().unwrap();
        let amount: U256 = tokens[2].clone().into_uint().unwrap();
        let timestamp: U256 = tokens[3].clone().into_uint().unwrap();
        let claimable: U256 = tokens[4].clone().into_uint().unwrap();

        Ok(Self {
            user,
            amount,
            timestamp,
            claimable,
        })
    }
}

impl Detokenize for OurContract {
    fn from_tokens(tokens: Vec<Token>) -> Result<Self, web3::contract::Error> {
        let owner: Address = tokens[0].clone().into_address().unwrap();
        let address: Address = tokens[1].clone().into_address().unwrap();
        let name: String = tokens[2].clone().into_string().unwrap();
        let symbol: String = tokens[3].clone().into_string().unwrap();
        let _total_supply_: U256 = tokens[4].clone().into_uint().unwrap();
        let decimal: U256 = tokens[5].clone().into_uint().unwrap();

        Ok(Self {
            owner,
            address,
            name,
            symbol,
            _total_supply_,
            decimal,
        })
    }
}

impl Detokenize for StakingSummary {
    fn from_tokens(tokens: Vec<Token>) -> Result<Self, web3::contract::Error>
    where
        Self: Sized,
    {
        let total_stakes = tokens[0].clone().into_uint().unwrap();
        let stakes: Vec<Stake> = Vec::new();

        Ok(Self {
            total_stakes,
            stakes,
        })
    }
}

impl Detokenize for User {
    fn from_tokens(tokens: Vec<Token>) -> Result<Self, web3::contract::Error> {
        let balances: U256 = tokens[0].clone().into_uint().unwrap();
        let stakes: Vec<StakingSummary> = Vec::new();
        Ok(Self { balances, stakes })
    }
}

impl OurContract {
    pub async fn total_supply() -> U256 {
        let contract = contract_instance::get_contract().await;

        let total_supply_: U256 = contract
            .query("getTotalSupply", (), None, Options::default(), None)
            .await
            .unwrap();

        return total_supply_;
    }

    pub async fn balances(account: Address) -> U256 {
        let contract = contract_instance::get_contract().await;
        //let account = Address::from(account);

        let balances: U256 = contract
            .query("getBalance", account, None, Options::default(), None)
            .await
            .unwrap();

        return balances;
    }

    pub async fn locked_balance(account: Address) -> StakingSummary  {
        let contract = contract_instance::get_contract().await;
        let locked_balance: StakingSummary = contract
            .query("hasStake", account, account, Options::default(), None)
            .await
            .unwrap();

        return locked_balance
    }

    pub async fn get_contract_info()->OurContract{
        let contract = contract_instance::get_contract().await;

        let _owner: Address = contract.query("getOwner", (), None, Options::default(), None).await.unwrap();
        let _address: Address = contract.query("getContractAddress", (), None, Options::default(), None).await.unwrap();
        let _name:String = contract.query("getName", (), None, Options::default(), None).await.unwrap();
        let _symbol:String = contract.query("getSymbol", (), None, Options::default(), None).await.unwrap();
        let _total_supply:U256 =  contract
        .query("getTotalSupply", (), None, Options::default(), None)
        .await
        .unwrap();
        let _decimal:U256 = contract.query("getDecimal", (), None, Options::default(), None).await.unwrap();


        let info  = OurContract{
            owner: _owner,
            address: _address,
            name: _name,
            symbol: _symbol,
            _total_supply_: _total_supply,
            decimal: _decimal,
        };

        return info
        
    }
}
