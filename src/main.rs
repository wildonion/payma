






use ring::*;



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    
    // https://github.com/skerkour/black-hat-rust/tree/main/ch_11

    // TODO 
    // encrypt the whole os using ring based on keypairs
    // we can decrypt it with private key only and the victim
    // must pay for the private key
    // ...

    // TODO
    // software update and data encryption using PGP cert and ed25519 public key digital signature
    // ed25519 digital signature key 
    //    --- has of public key is wallet
    //    --- private key can be used to sign tx
    //    --- seed phrase can be used to generate the keypairs 
    // ...



    Ok(())

}
