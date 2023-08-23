



Payma is a WASM based Virtual Smart Contract Supports Crypto Wallet on top of ZKP to Build Trust with Coworkers

## Process 

- generate keypair for both parties
- employee deposit the full payment into this contract 
- the whole moeny will be mapped to the contractor in this contract
- on every 10 commits, confirmed by the employee `full = full/10` will be transferred to the contractor
- we run step 4 inside a `loop{}` until all the moeny gets paid to the contractor