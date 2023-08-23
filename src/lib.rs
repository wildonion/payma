
/*


    zero copy      ::::: https://github.com/wildonion/uniXerr/blob/a30a9f02b02ec7980e03eb8e31049890930d9238/infra/valhalla/coiniXerr/src/schemas.rs#L1621C6-L1621C6
    data collision ::::: https://github.com/wildonion/uniXerr/blob/a30a9f02b02ec7980e03eb8e31049890930d9238/infra/valhalla/coiniXerr/src/utils.rs#L640 
    https://crates.io/crates/pyo3
    https://crates.io/crates/wasmtime
    https://wasmer.io/
    https://github.com/skerkour/black-hat-rust/tree/main/ch_11
    https://cryptobook.nakov.com/digital-signatures
    https://github.com/wildonion/cs-concepts
    https://github.com/mozilla/cbindgen -> generate c bindings and .so from rust code


    malware in rust using ram concepts (static and const are in segment data and let is on the stack)
    injectable code like .so and .dll
    hardware coding, 
    memory layout and offset, 
    playing with byte, hex and pointers
    unsafe coding
    writing engines
    convert this contract into a wasm module to be loadable inside js
    lle (hex editor, bytes, seeds, xor, nor, &, |, include!, liefetime, 
        generic, bytes, hex, base64, raw parts, &mut pointer, unpin 
        and box, phantomdata) 
    binding using .so and https://crates.io/crates/pyo3
    encrypt the whole os using ring based on PGP keypairs
        we can decrypt it with private key only and the victim
        must pay for the private key
    
    

*/


pub mod zkp{

    // https://noir-lang.org/index.html
    // https://github.com/rust-cc/awesome-cryptography-rust#zero-knowledge-proofs

}