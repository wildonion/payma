

<p align = "center">
  <b>🚧 Payma was an idea about creating smart banking solutions to make a secure connections between two parties 🚧</b> 
</p>

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
