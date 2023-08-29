

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
use themis::keys as themis_keys;
use themis::secure_message::{SecureSign, SecureVerify};
use themis::keygen::gen_ec_key_pair;
use themis::keys::{EcdsaKeyPair, EcdsaPrivateKey, EcdsaPublicKey};
use themis::keys::KeyPair as ThemisKeyPair;
use secp256k1::hashes::Hash;
use rand::random;

use crypter;
pub mod constants;
pub mod wallet;
use wallet::{Wallet, Contract};


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

    /* wallet operations */

    let contract = Contract::new_with_secp256k1("wildonion");

    let signature = Wallet::secp256k1_sign(contract.wallet.secp256k1_secret_key.as_ref().unwrap().to_string(), stringify_data.clone());

    let pubkey = Wallet::generate_secp256k1_pubkey_from(contract.wallet.secp256k1_public_key.as_ref().unwrap().to_string()).unwrap();

    let keypair = Wallet::retrieve_secp256k1_keypair(
        /* 
            unwrap() takes the ownership of the type hence we must borrow 
            the type before calling it using as_ref() 
        */
        contract.wallet.secp256k1_secret_key.as_ref().unwrap().as_str()
    );


    let verification_result = Wallet::verify_secp256k1_signature(stringify_data, signature, pubkey).unwrap();


    Ok(())     


}
