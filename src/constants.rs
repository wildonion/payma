

use crate::*;


pub static KEYPAIR: Lazy<Result<ring_signature::Ed25519KeyPair, ring::error::KeyRejected>> = Lazy::new(||{
    let rng = ring_rand::SystemRandom::new();
    let pkcs8_bytes = ring_signature::Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
    let keys = ring_signature::Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref());

    // let seed_phrase = "this is a private key";
    // let keys = ring_signature::Ed25519KeyPair::from_seed_unchecked(seed_phrase.as_bytes());

    keys

});