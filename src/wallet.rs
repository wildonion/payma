

use crate::*;


/* secp256k1 crypto wallet (compatible with all evm based chains) */


#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct NewSeedRequest{
    pub mail: String,
    pub username: String,
    pub phone_number: String,
    pub paypal_id: String,
    pub account_number: String,
    pub device_id: String,
    pub social_id: String,
}


// https://thalesdocs.com/gphsm/luna/7/docs/network/Content/sdk/using/ecc_curve_cross-reference.htm
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Wallet {
    pub secp256k1_secret_key: Option<String>,
    pub secp256k1_public_key: Option<String>,
    pub secp256k1_public_address: Option<String>,
    pub secp256r1_secret_key: Option<String>,
    pub secp256r1_public_key: Option<String>,
    pub ed25519_secret_key: Option<String>,
    pub ed25519_public_key: Option<String>
}

impl Wallet{

    pub fn generate_keccak256_from(pubk: String) -> String{

        let pubk = PublicKey::from_str(&pubk).unwrap();
        let public_key = pubk.serialize_uncompressed();
        let hash = keccak256(&public_key[1..]);
        let addr: Address = Address::from_slice(&hash[12..]);
        let addr_bytes = addr.as_bytes();
        let addr_string = format!("0x{}", hex::encode(&addr_bytes));
        addr_string

    }

    pub fn new_secp256k1(input_seed: NewSeedRequest) -> Self{

        /* generating seed from the input id to create the rng for secp256k1 keypair */
        let input_seed_string = serde_json::to_string(&input_seed).unwrap();
        let input_seed_bytes = input_seed_string.as_bytes();
        let hashed_input_seed = ring::digest::digest(&ring::digest::SHA256, input_seed_bytes);
        let hashed_input_seed_bytes = hashed_input_seed.as_ref();
        
        /* to create the rng we need a 32 bytes seed and we're sure that the hash is 32 bytes cause it's sha256 bits */
        let seed = <[u8; 32]>::try_from(&hashed_input_seed_bytes[0..32]).unwrap(); /* creating a 32 bytes from the first 32 bytes of hashed_input_seed_bytes */
        let rng = &mut StdRng::from_seed(seed);
        
        /* since the secp is going to be built from an specific seed thus the generated keypair will be the same everytime we request a new one */
        let secp = secp256k1::Secp256k1::new();
        let (prvk, pubk) = secp.generate_keypair(rng);
        let prv_str = prvk.display_secret().to_string();

        Wallet{
            secp256k1_secret_key: Some(prv_str),
            secp256k1_public_key: Some(pubk.to_string()),
            secp256k1_public_address: Some(Self::generate_keccak256_from(pubk.to_string())),
            secp256r1_public_key: None,
            secp256r1_secret_key: None,
            ed25519_public_key: None,
            ed25519_secret_key: None
        }
    }

    pub fn generate_secp256k1_pubkey_from(pk: String) -> Result<PublicKey, secp256k1::Error>{
        let secp256k1_pubkey = PublicKey::from_str(&pk);
        secp256k1_pubkey
    }

    pub fn verify_secp256k1_signature(data: String, sig: Signature, pk: PublicKey) -> Result<(), secp256k1::Error>{

        /* 
            data is required to be passed to the method since we'll compare
            the hash of it with the one inside the signature 
        */
        let data_bytes = data.as_bytes();
        let hashed_data = Message::from_hashed_data::<sha256::Hash>(data_bytes);
            
        /* message is an sha256 bits hashed data */
        let secp = Secp256k1::verification_only();
        secp.verify_ecdsa(&hashed_data, &sig, &pk)

    }

    pub fn retrieve_secp256k1_keypair(secret_key: &str) -> (PublicKey, SecretKey){

        /* 
            since secret_key is a hex string we have to get its bytes using 
            hex::decode() cause calling .as_bytes() on the hex string converts
            the hex string itself into bytes and it doesn't return the acutal bytes
        */
        let prv_bytes = hex::decode(secret_key).unwrap();
        let secp = Secp256k1::new();
        let secret_key = SecretKey::from_slice(&prv_bytes).unwrap();
        let public_key = PublicKey::from_secret_key(&secp, &secret_key);

        (public_key, secret_key)
    }

    pub fn secp256k1_sign(signer: String, data: String) -> Signature{

        let secret_key = SecretKey::from_str(&signer).unwrap();
        let data_bytes = data.as_bytes();
        let hashed_data = Message::from_hashed_data::<sha256::Hash>(data_bytes);
        
        /* message is an sha256 bits hashed data */
        let secp = Secp256k1::new();

        /* signing the hashed data */
        secp.sign_ecdsa(&hashed_data, &secret_key)

    }

    pub fn generate_sha256_from(data: String) -> [u8; 32]{

        /* generating sha25 bits hash of data */
        let data_bytes = data.as_bytes();
        let hash_data = sha256::Hash::hash(data_bytes);
        let hash_data_bytes = hash_data.as_byte_array();
        hash_data_bytes.to_owned()

    }
    
}

pub struct Contract{
    pub wallet: Wallet,
    pub iat: i64,
    pub owner: &'static str,
    pub data: Option<Data>,
}

impl Contract{

    fn new_with_secp256k1(owner: &str) -> Self{
        
        let static_owner = constants::string_to_static_str(owner.to_string());
        let wallet = Wallet::new_secp256k1(NewSeedRequest::default());

        Self { 
            wallet,
            iat: chrono::Local::now().timestamp_nanos(), 
            owner: static_owner,
            data: None
        }
        
    }

    pub fn init(owner: &str, repo: String, commits: u16, budget: u16) -> Self{

        let mut data = Data{
            repo, 
            commits,
            budget,
            signature: "".to_string(),
            signed_at: 0,
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

        data.signature = signature.to_string();
        data.signed_at = chrono::Local::now().timestamp_nanos();

        Self { wallet: contract.wallet.clone(), iat: contract.iat, owner: contract.owner, data: Some(data) }


    }

}

#[derive(Serialize, Deserialize)]
pub struct Data{
    pub repo: String,
    pub commits: u16,
    pub budget: u16,
    pub signed_at: i64,
    pub signature: String
}
    