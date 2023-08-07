rustup target add wasm32-wasi # compilation target for wasm32 wasi WebAssembly
# rustup target add wasm32-unknown-unknown # compilation target for browser-based WebAssembly
cargo build --bin payma --target wasm32-wasi --release 
sudo cp ../../../target/wasm32-wasi/release/payma.wasm ./payma.wasm
wasm-opt -Oz payma.wasm -o payma.wasm # execute default optimization, passes, super-focusing on code
wasmtime payma.wasm
wasmer run payma.wasm 