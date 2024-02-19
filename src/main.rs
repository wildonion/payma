

use sha2::{Digest, Sha256};
use std::fmt::Write;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
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
    if we want to use Result<(), impl std::error::Error + Send + Sync + 'static>
    as the return type of the error part, the exact error type instance must be 
    sepecified also the Error trait must be implemented for the error type (impl 
    Error for ErrorType{}) since we're implementing the Error trait for the error 
    type in return type which insists that the instance of the type implements the 
    Error trait. by returning a boxed error trait we're returning the Error trait 
    as a heap object behind a valid pointer which handles all error type at runtime, 
    this is the solution to return traits as an object cause we don't know what type 
    causes the error at runtiem and is the implementor of the Error trait which 
    forces us to return the trait as the error itself and since traits are dynamically
    sized we can't treat them as a typed object directly we must put them behind 
    pointer like &'valid dyn Trait or box them to send them on the heap, also by 
    bounding the Error trait to Send + Sync + 'static we'll make it sefable, sendable 
    and shareable to move it between different scopes and threads.
*/
#[tokio::main(flavor="multi_thread", worker_threads=10)]
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

    /* generaring keypair from existing mnemonic */
    let existing_mnemonic_sample = Some("obot glare amazing hip saddle habit soft barrel sell fine document february");
    let mut contract = Contract::new_with_secp256k1("0xDE6D7045Df57346Ec6A70DfE1518Ae7Fe61113f4", "wildonion123", existing_mnemonic_sample);
    
    /* generaring a new keypair */
    // let mut contract = Contract::new_with_secp256k1("0xDE6D7045Df57346Ec6A70DfE1518Ae7Fe61113f4", "wildonion123", None);
    
    Wallet::save_to_json(&contract.wallet, "secp256k1").unwrap();
    
    let signature_hex = Wallet::secp256k1_sign(stringify_data.clone().as_str(), contract.wallet.secp256k1_secret_key.clone().unwrap().as_str());
    
    let hash_of_data = Wallet::generate_keccak256_hash_from(&data.value);
    let verify_res = Wallet::verify_secp256k1_signature_from_pubkey_str(hash_of_data.as_slice(), signature_hex.clone().to_string().as_str(), contract.wallet.secp256k1_public_key.clone().unwrap().as_str());
    
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
