


use crate::*;


/*


    `ed25519` keypair for server checksum and licensing, updating and verification using 
    its commit (like ssh keys), also time hash based (**`hash(user_id + time + ip + user agent)`**) 
    locking api with rate limit feature to avoid guarded api call spamming (like sleeping in thread 
    or using snowflake id based on client secret keys) using `argon2`, `rust-crypto`, `noise` and `ring` tools


    - Openssl and ed25519 and tokio rustls also secret chat based on a derived secret key for bot users per device 
    - cryptography algos(hex editor, bytes, seeds, xor, nor, &, |, include!, ed25519(pub/prv keys and base58 for ssh))
    - ed25519: public key is the wallet address in which we can verify the signature of the signed tx | seed will be used to generate keypair and private key is the pen to sign tx and generate signature | both public and private key are of type &[u8; 32] which are a 32 bytes slice of utf8

    >>>>>>>> u8 bytes -> &str using str::from_utf8()
    >>>>>>>> &str -> u8 bytes using as_bytes() or as_bytes_mut()
    >>>>>>>> u8 -> u16 using transmute or shift bits operations (shift 2 bytes) or u8 to hex ascii string then to u16 using self::from_hex_string_to_u16() function
    >>>>>>>> u8 -> hex ascii string using self::from_u8_to_hex_string() function
    >>>>>>>> hex ascii string to u8 or u16 using from_str_radix()
    >>>>>>>> u8 -> hex ascii vector using :x? in println! macro or dividing operations : u8 bytes % 16 


*/






pub fn from_u8_to_hex_string(bytes: &[u8]) -> Result<String, ()> { //// take a reference from u8 and will return a hex String
    
    use hex::*;
    
    /*
        let hex_ascii_string = "hello world".as_bytes().iter().map(|x| format!("{:02x}", x)).collect::<String>()
        >> let mut s = String::new();
        >> use std::fmt::Write as FmtWrite; // renaming import to avoid collision
        >> for b in "hello world".as_bytes() { write!(s, "{:02x}", b); }
        ()
        >> s
        "68656c6c6f20776f726c64"
        >> 
    */
    let hex_arr = &[0x23u8, 0xF2u8];
    let mut buffer = String::with_capacity(bytes.len() * 2); //// length of the String must be double of the size of the u8 cause we want to write u16 or hex into this buffer
    for &b in bytes {
        write!(&mut buffer, "{:02x}", b).expect("⚠️ writing to String buffer error for hex ascii"); //// writing formatted data into the buffer which is the String - panic on any error
    }
    Ok(buffer)
}

pub fn from_hex_string_to_u8(hex_string: &str) -> Result<Vec<u8>, ()>{
    let mut hex_bytes = hex_string.as_bytes().iter().filter_map(|b| {
        match b {
            b'0'..=b'9' => Some(b - b'0'),
            b'a'..=b'f' => Some(b - b'a' + 10),
            b'A'..=b'F' => Some(b - b'A' + 10),
            _ => None,
        }
    }).fuse();

    let mut bytes = Vec::new();
    while let (Some(h), Some(l)) = (hex_bytes.next(), hex_bytes.next()) {
        bytes.push(h << 4 | l)
    }
    Ok(bytes)
}

pub fn from_hex_string_to_u16(s: &str) -> Result<Vec<u16>, std::num::ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u16::from_str_radix(&s[i..i + 2], 16))
        .collect()
}


struct Keypair;
struct Pubkey;
struct Prvkey;
impl Keypair{
    fn new() -> (Pubkey, Prvkey){
        (Pubkey, Prvkey)
    }
}
// let keypairs: (Pubkey, Prvkey) = Keypair::new();
// use private key to sign the data 
// use public key to verify the signature
// use seed phrase to generate both keys
