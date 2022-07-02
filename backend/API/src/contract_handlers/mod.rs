use crate::server_config::Config;
use crate::total_supply;
use core::time;
use serde::{Deserialize, Serialize};
use serde_json;
use std::str::FromStr;
use std::{env, u8};
use tracing::info;
use web3::contract::tokens::{Detokenize, Tokenizable, Tokenize};
use web3::contract::{Contract, Options};
use web3::ethabi::token::Token;
use web3::types::{Address, H160, U256};
use web3::{signing, Web3};

mod contract_instance;
mod key_instance;

#[derive(Clone, Debug, Serialize)]
pub struct OurContract {
    owner: Address,
    address: Address,
    name: String,
    symbol: String,
    _total_supply_: U256,
    decimal: U256,
}

#[derive(Clone, Debug)]
pub struct User {
    balances: U256,
    stakes: Vec<StakingSummary>,
}

#[derive(Clone, Debug)]
pub struct AcceptStakes {
    total: U256,
    user: Vec<Token>,
    amount: Vec<Token>,
    timestamp: Vec<Token>,
    claimable: Vec<Token>,
}

#[derive(Clone, Debug, Serialize)]
pub struct Stakes {
    user: Address,
    amount: u64,
    timestamp: u64,
    claimable: u64,
}

#[derive(Clone, Debug, Serialize)]
pub struct StakingSummary {
    total_stakes: u64,
    stakes: Vec<Stakes>,
}

#[derive(Serialize, Deserialize)]
pub struct StakeRequest {
    pub private_key: String,
    pub amount_to_stake: u32,
}

pub struct PrvKey<'a> {
    pub prv_key: &'a web3::signing::SecretKeyRef<'a>,
}

impl Detokenize for AcceptStakes {
    fn from_tokens(tokens: Vec<Token>) -> Result<Self, web3::contract::Error> {
        let total: U256 = tokens[0].clone().into_uint().unwrap();
        let user: Vec<Token> = tokens[1].clone().into_array().unwrap();
        let amount: Vec<Token> = tokens[2].clone().into_array().unwrap();
        let timestamp: Vec<Token> = tokens[3].clone().into_array().unwrap();
        let claimable: Vec<Token> = tokens[4].clone().into_array().unwrap();

        Ok(Self {
            total,
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

    pub async fn balances(account: Address) -> u64 {
        let contract = contract_instance::get_contract().await;

        let balances: U256 = contract
            .query("getBalance", account, None, Options::default(), None)
            .await
            .unwrap();

        return U256::as_u64(&balances);
    }

    async fn map_stakes(stakes: AcceptStakes) -> StakingSummary {
        let mut i = 0;

        let mut staking_summary = StakingSummary {
            total_stakes: 0,
            stakes: vec![],
        };

        staking_summary.total_stakes = U256::as_u64(&stakes.total);

        while i < stakes.user.len() {
            let mut user_ = stakes.user[i]
                .clone()
                .into_address()
                .expect("converting user address");
            let mut amount_ = stakes.amount[i]
                .clone()
                .into_uint()
                .expect("converting staking amount");
            let mut timestamp_ = stakes.timestamp[i]
                .clone()
                .into_uint()
                .expect("converting staking amount");
            let mut claimable_ = stakes.claimable[i]
                .clone()
                .into_uint()
                .expect("converting staking amount");

            let mut stake_to_push = Stakes {
                user: user_,
                amount: U256::as_u64(&amount_),
                timestamp: U256::as_u64(&timestamp_),
                claimable: U256::as_u64(&claimable_),
            };

            staking_summary.stakes.push(stake_to_push);

            i = i + 1;
        }

        return staking_summary;
    }

    pub async fn get_stakes(account: Address) -> StakingSummary {
        let contract = contract_instance::get_contract().await;
        let stakes: AcceptStakes = contract
            .query("hasStakePublic", account, account, Options::default(), None)
            .await
            .unwrap();

        let result = OurContract::map_stakes(stakes).await;

        return result;
    }

    pub async fn get_contract_info() -> OurContract {
        let contract = contract_instance::get_contract().await;

        let _owner: Address = contract
            .query("getOwner", (), None, Options::default(), None)
            .await
            .unwrap();
        let _address: Address = contract
            .query("getContractAddress", (), None, Options::default(), None)
            .await
            .unwrap();
        let _name: String = contract
            .query("getName", (), None, Options::default(), None)
            .await
            .unwrap();
        let _symbol: String = contract
            .query("getSymbol", (), None, Options::default(), None)
            .await
            .unwrap();
        let _total_supply: U256 = contract
            .query("getTotalSupply", (), None, Options::default(), None)
            .await
            .unwrap();
        let _decimal: U256 = contract
            .query("getDecimal", (), None, Options::default(), None)
            .await
            .unwrap();

        let info = OurContract {
            owner: _owner,
            address: _address,
            name: _name,
            symbol: _symbol,
            _total_supply_: _total_supply,
            decimal: _decimal,
        };

        return info;
    }

    /*TODO : figure out how to handle transactions in api(talk to kak krisna)
    pub async fn stake(
        private_key: String,
        amount_to_stake: u32,
    ) -> web3::types::TransactionReceipt {
        let contract = contract_instance::get_contract().await;
        let prv_key_ = key_instance::get_prv_key(private_key);

        let sign_key = PrvKey { prv_key: &prv_key_ };

        let staking_amount = U256::from(amount_to_stake);


        let res = contract
            .signed_call_with_confirmations(
                "stake",
                staking_amount,
                Options::default(),
                0,
                sign_key,
            )
            .await
            .unwrap();

        return res;
    }*/
}
