



payma is a WASM based virtual smart contract supports web3 crypto based `secp256k1` wallet on top of polygon chain to build trust with coworkers. It can also be used and injected as a `.wasm` file into `js` projects.

## 🤝🏻 How it works 

> note that the transfer process must be called by the payma authority itself.

- generate `secp256k1` keypair for both parties
- employee deposit the full payment into the polygon split contract
- on for example every 10 commits, confirmed by the employee followings will be transferred to the contractor and the payma wallet:
```rust
fn receive_amount(amount: u64, perc: u8) -> u64{
    let percent = Percentage::from(perc);
    let amount_receive = percent.apply_to(amount);
    amount_receive
}
let payma_commission = receive_amount(deposited_amount, 20);
let mut pure_fee_after_tax = deposited_amount - payma_commission;
while let Ok(confirmed_commits) = get_employee_confirmation().await{
    let cn = confirmed_commits.number;
    pure_fee_after_tax /= cn;
    let contractor_fee = pure_fee_after_tax;
    
    // transfer contractor_fee to the contractor wallet
    // ...
    
}
```

## 🎯 Run 

```bash
cargo run --bin payma
```

## 🏗️ Build WASM

```bash 
sudo chmod +x build.sh && ./build.sh
```

## 🚀 Create, Build, Deploy, Publish Contract on Thirdweb

```bash
# Create contract 
sudo npx thirdweb create contract && sudo chmod -R 777 .
# Compiles contract and detects thirdweb extensions implemented on them.
yarn build
# Deploys contract with the thirdweb deploy flow.
yarn deploy
# Publishes contract with the thirdweb publish flow.
yarn publish
# Release contract publicly
yarn release
```

## 🌋 Deploy Payma fastapi Server to Interact with Thirdweb

```bash
sudo docker network create -d bridge payma || true
sudo docker build -t thirdweb -f $(pwd)/infra/docker/thirdweb/Dockerfile . --no-cache
sudo docker run -d --restart unless-stopped --network payma --name thirdweb -p 7651:7650 thirdweb
```

## 🚧 WIPs

- update crates, rust edition, [wasm32 updation](https://blog.rust-lang.org/2024/04/09/updates-to-rusts-wasi-targets.html)
- code and deploy split contract
- fastapi server using thirdweb sdk to call split contract apis
- diagram
- cli args using clap
- wasm compilation
