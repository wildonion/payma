
/*

    https://thirdweb.com/thirdweb.eth/Split
    https://crates.io/crates/wasmtime
    https://wasmer.io/

*/

use serde::{Serialize, Deserialize};
use std::fmt::Write;
use crate::*;

/* 
    don't include wallet module in here cause it main.rs modules 
    can't be accessible inside the wallet.rs since by using wallet 
    in here the root would be the lib.rs intead of main.rs
*/


pub async fn deposit(contract: Contract) -> Result<String, Box<dyn std::error::Error + Send + Sync + 'static>>{


    let buffer_size = std::env::var("BUFFER_SIZE").unwrap_or("1024".to_string()).parse::<usize>().unwrap();
    let host = std::env::var("HOST").unwrap();
    let port = std::env::var("PORT").unwrap().parse::<u16>().unwrap();
    let (deposit_tx_hash_sender, mut deposit_tx_hash_receiver) = tokio::sync::mpsc::channel::<String>(buffer_size);
    let mut tx = String::from("");
    
    tokio::spawn(async move{


        /* call deposit api */
        let tx_hash = "".to_string();
        deposit_tx_hash_sender.send(tx_hash).await;

    });

    
    while let Some(tx_hash) = deposit_tx_hash_receiver.recv().await{
        
        tx = tx_hash;
    }

    return Ok(tx);

}

pub async fn withdraw(contract: Contract) -> Result<String, Box<dyn std::error::Error + Send + Sync + 'static>>{
    
    let buffer_size = std::env::var("BUFFER_SIZE").unwrap_or("1024".to_string()).parse::<usize>().unwrap();
    let host = std::env::var("HOST").unwrap();
    let port = std::env::var("PORT").unwrap().parse::<u16>().unwrap();
    let (withdraw_tx_hash_sender, mut withdraw_tx_hash_receiver) = tokio::sync::mpsc::channel::<String>(buffer_size);
    let mut tx = String::from("");

    tokio::spawn(async move{


        /* call withdraw api */
        let tx_hash = "".to_string();
        withdraw_tx_hash_sender.send(tx_hash).await;

    });

    while let Some(tx_hash) = withdraw_tx_hash_receiver.recv().await{
        
        tx = tx_hash;
    }

    return Ok(tx);


}