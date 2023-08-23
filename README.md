



Payma is a WASM based virtual smart contract supports web3 crypto wallet on top of **ZKP** algorithm to build trust with coworkers. It can also be used and injected as a dynamic library or shared object (`.so`) file in other codes and as a `.wasm` file into `js` projects.

## Process 

- generate keypair for both parties
- employee deposit the full payment into this contract 
- the whole moeny will be mapped to the contractor in this contract
- on every 10 commits, confirmed by the employee `full = full/10` will be transferred to the contractor
- we run step 4 inside a `loop{}` until all the moeny gets paid to the contractor