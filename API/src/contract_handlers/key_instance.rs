use std::str::FromStr;
use web3::signing::SecretKey;

pub fn get_prv_key(key: String)->SecretKey {
    let key = SecretKey::from_str(&key).expect("error converting into secret key");

    return key
}
