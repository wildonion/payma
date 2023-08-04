






use std::fmt::Write;
use ring::{signature as ring_signature, rand as ring_rand};
use ring::signature::Ed25519KeyPair;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};


mod crypto;
mod constants;


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
    let stringify_data = serde_json::to_string(&data).unwrap();

    let contract = crypto::Contract::new("wildonion");
    let signature_bytes = contract.sign(stringify_data.as_str());
    
    
    let signature_hex = hex::encode(&signature_bytes);
    let pubkey_hex = hex::encode(&contract.get_public_key());
    let prvkey_hex = hex::encode(&contract.get_private_key());


    let is_verified = contract.is_valid_transaction(signature_bytes, stringify_data.as_str());

    if is_verified{

        // start the project 
        // ...

    } else{

        // stop working
        // ...
    }




    Ok(())

}
