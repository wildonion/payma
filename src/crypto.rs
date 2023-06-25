


use crate::*;
use constants::KEYPAIR;


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
    
    let msg = "get";
    let msg_bytes = msg.as_bytes();
    /* 

        `get` payload in hex will be 0x676574 since `g` is 67 in hex
        which is 103 in decimal which is in form of utf8 bytes means that 
        every char in form of utf8 bytes must be in range 0 up to 255
        also every char in hex is 4 bits in binary which means every two
        chars in hex is 1 byte in utf8 bytes thus 0x676574 is 3 bytes in 
        form of utf8 bytes also chars can be represented in form of utf16 
        or 2 bytes long, like `get` is 0xfeff0067feff0065feff0074 in hex which
        is 12 bytes long or 24 hex chars because `get` has 3 chars which in 
        utf16 form every char has size of 2 bytes which is 4 chars in hex 
        thus 3 * 4 = 12 bytes in total for 3 chars in utf16 form.

    */
    let playload_hex_ascii = msg_bytes.iter().map(|b| format!("{:x}", b)).collect::<String>();
    println!("{playload_hex_ascii:}");


    ///// -------------- union, enum and struct -------------- /////
    /*
        offset is the size in bytes and in order to get
        the starting offset of a type or struct instance 
        we can get a raw pointer (since smart pointer in rust 
        will be coerced to raw pointers at compile time) 
        to the instance then cast that pointer to usize 
        which is the size in bytes of the instance pointer itself
    */
    
    /*
        a pointer contains the memory address of an obejct
        and it has either 32 or 64 bits (depends on the os arc)
        size hence we can get it's size by casting it into the 
        usize type that contains the size of that pointer in bytes
        inside the stack
    */
 
    struct Object{
        a: u8, //// we can fill this in a hex form
        b: u16, //// we can fill this in a hex form
        c: u32, //// we can fill this in a hex form
    }

    // utf8 hex representation
    let obj = Object{
        /*
            since `a` field is of type u8 thus we have to fill 
            it with only two chars in hex since every 4 bits 
            in base 2 is a hex char; the 0xaa is 170 in decimal
            0xaa is one byte or 8 bits
        */
        a: 0xaa, 
        /*
            since `b` field is of type u16 thus we have to fill 
            it with only four chars in hex since every 4 bits 
            in base 2 is a hex char; the 0xaa is 48059 in decimal
            0xbbbb is two bytes or 16 bits
        */
        b: 0xbbbb, 
        /*
            since `c` field is of type u32 thus we have to fill 
            it with only eight chars in hex since every 4 bits 
            in base 2 is a hex char; the 0xcccccccc is 3435973836 in decimal
            0xcccccccc is two bytes or 32 bits
        */
        c: 0xcccccccc
    };

    /*
        usize is an unsigned size which is big enough
        to store any pointer and in 32 bits arch is 4 bytes
        and in 64 bits is 8 bytes also each usize contains 
        the size in bytes in either 32 or 64 bits format
    //
        base is the usize pointer of the object itself 
        which contains the size of the starting offset 
        in bytes in memory, we've just cast the pointer 
        to the location of the obj instance into the usize
        to get the size of its pointer in bytes which is the 
        starting offset of all its fields
    */
    let base = &obj as *const _ as usize; //// we're considering the pointer of the obj instance as the starting point of the offset by converting its pointer into usize 
    let a_off = &obj.a as *const _ as usize - base; //// this is the `a` field offset by subtracting its usize pointer (cast its *const pointer to usize) from the base offset
    let b_off = &obj.b as *const _ as usize - base; //// this is the `b` field offset by subtracting its usize pointer (cast its *const pointer to usize) from the base offset
    let c_off = &obj.c as *const _ as usize - base; //// this is the `c` field offset by subtracting its usize pointer (cast its *const pointer to usize) from the base offset
    println!("base: {}", base); 
    println!("a: {}", a_off as usize - base);
    println!("b: {}", b_off as usize - base);
    println!("c: {}", c_off as usize - base);


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



/* ED25519 implementation using ring */
struct Payma{
    pub keypair: &'static Ed25519KeyPair,
}

impl Payma{
    fn new() -> Self{
        
        let keypair = KEYPAIR.as_ref();
        let Ok(keys) = keypair else{
            panic!("can't generate keypair due to: {:?}", keypair.unwrap_err());
        };

        Self { keypair: keys }
        
    }

    fn sign(&self, data: &str) -> Vec<u8>{


        let signature = self.keypair.sign(data.as_bytes());
        signature.as_ref().to_vec()


    }
}
