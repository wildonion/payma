
/*

    https://thirdweb.com/thirdweb.eth/Split
    https://crates.io/crates/wasmtime
    https://wasmer.io/

    TODO - wasm compilation
    TODO - fastapi server to call thirdweb split contract apis

*/

use serde::{Serialize, Deserialize};
use std::fmt::Write;

/* 
    don't include wallet module in here cause it main.rs modules 
    can't be accessible inside the wallet.rs since by using wallet 
    in here the root would be the lib.rs intead of main.rs
*/