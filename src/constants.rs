

use ring::pkcs8::Document;
use crate::*;


pub static KEYPAIR: Lazy<(Result<ring_signature::Ed25519KeyPair, ring::error::KeyRejected>, Document)> = Lazy::new(||{
    let rng = ring_rand::SystemRandom::new();
    let pkcs8_bytes = ring_signature::Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
    let keys = ring_signature::Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref());
    
    (keys, pkcs8_bytes)

});