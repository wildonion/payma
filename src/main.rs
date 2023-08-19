






use std::fmt::Write;
use ring::{signature as ring_signature, rand as ring_rand};
use ring::signature::Ed25519KeyPair;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use crypter::{self, Wallet};
use web3::contract;


mod constants;
mod lle;


/*
    the error part of the result type is a pointer to a std Error trait 
    bounded to Send, Sync and 'static lifetime to be shareable between
    threads, the reason is actually we don't know the type that will cause
    the error at runtime and in order to handle the error for that type
    we must bound the type to this trait at runtime which can handle all
    the possible errors of the type, also since traits are not fix sized 
    and they're on the heap we must put them behind a pointer like &dyn Trait
    with a valid lifetime or inside the Box like Box<dyn Trait> which has 
    its own valid lifetime.
*/
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    
    #[derive(Serialize, Deserialize)]
    struct Data{
        pub repo: String,
        pub commits: u16,
        pub budget: u16 
    }
    let data = Data{
        repo: "github repo containing the code".to_string(), 
        commits: 0u16,
        budget: 50
    };
    let stringify_data = serde_json::to_string_pretty(&data).unwrap();

    let contract = crypter::Contract::new("wildonion");
    
    let signature_hex = Wallet::ed25519_sign(stringify_data.clone(), contract.wallet.ed25519_secret_key.unwrap());
    
    let is_verified = Wallet::verify_ed25519_signature(signature_hex.unwrap(), stringify_data, contract.wallet.ed25519_public_key.unwrap());

    if is_verified{

        // start the project 
        // ...

    } else{

        // stop working
        // ...
    }




    Ok(())

}
