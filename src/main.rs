

use sha2::{Digest, Sha256};
use std::fmt::Write;
use ring::{signature as ring_signature, rand as ring_rand};
use ring::signature::Ed25519KeyPair;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use ring::{signature::KeyPair, pkcs8::Document};
use secp256k1::Secp256k1;
use secp256k1::ecdsa::Signature;
use secp256k1::{rand::SeedableRng, rand::rngs::StdRng, PublicKey, SecretKey, Message, hashes::sha256};
use std::io::BufWriter;
use tiny_keccak::keccak256;
use std::str::FromStr;
use std::{fs::OpenOptions, io::BufReader};
use web3::{
    transports,
    types::{Address, TransactionParameters, H256, U256},
    Web3,
};
use secp256k1::hashes::Hash;
use rand::random;
use wallexerr::*;


pub mod misc;
pub mod constants;



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

    let buffer_size = std::env::var("BUFFER_SIZE").unwrap_or("1024".to_string()).parse::<usize>().unwrap();
    let repo = "".to_string();
    let cmd = "deposit";

    let value = misc::Metadata{
        repo, 
        budget: 10, 
        commits: 50
    };

    /* --------------- */
    /* wallexerr tests */
    /* --------------- */
    let mut data = DataBucket{
        value: serde_json::to_string_pretty(&value).unwrap(), /* json stringify of config had instance */ 
        signature: "".to_string(),
        signed_at: 0,
    };
    let stringify_data = serde_json::to_string_pretty(&data.value).unwrap();

    /* wallet operations */

    let mut contract = Contract::new_with_secp256k1("0xDE6D7045Df57346Ec6A70DfE1518Ae7Fe61113f4", "");
    Wallet::save_to_json(&contract.wallet, "secp256k1").unwrap();
    
    let signature_hex = Wallet::secp256k1_sign(stringify_data.clone().as_str(), contract.wallet.secp256k1_secret_key.clone().unwrap().as_str());
    let verify_res = Wallet::verify_secp256k1_signature_from_pubkey_str(data.value.as_str(), signature_hex.clone().to_string().as_str(), contract.wallet.secp256k1_public_key.clone().unwrap().as_str());

    let keypair = Wallet::retrieve_secp256k1_keypair(
        /* 
            unwrap() takes the ownership of the type hence we must borrow 
            the type before calling it using as_ref() 
        */
        contract.wallet.secp256k1_secret_key.as_ref().unwrap().as_str()
    );

    /* fill the signature and signed_at fields if the signature was valid */
    if verify_res.is_ok(){
        data.signature = signature_hex.to_string();
        data.signed_at = chrono::Local::now().timestamp_nanos();

        contract.data = Some(data)
    }


    let tx_hash = if cmd == "deposit"{

        misc::deposit(&contract)
            .await
            .unwrap()

    } else{

        misc::withdraw(&contract)
            .await
            .unwrap()

    };
    

    Ok(())     


}
