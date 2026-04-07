# Otter Verify

- Stores data necessary to verify an on-chain program.
- This generates a PDA (Program Derived Address) using the `Program address + Seeds`. The PDA holds the parameters needed for the verification of the specific program.
- The program address of the on-chain otter-verify program is [`verifycLy8mB96wd9wqq3WDXQwM4oU6r42Th37Db9fC`](https://solana.fm/address/verifycLy8mB96wd9wqq3WDXQwM4oU6r42Th37Db9fC).

## Build Verification

 Use the [Solana Verify CLI](https://github.com/solana-foundation/solana-verifiable-build) to confirm that the deployed program at `verifycLy8mB96wd9wqq3WDXQwM4oU6r42Th37Db9fC` matches this repository. 
 
 After installing the CLI, run:

```bash
solana-verify -um verify-from-repo \
  https://github.com/otter-sec/otter-verify \
  --commit-hash a630c42962872ae9adac924151948bb17d87174d \
  --program-id verifycLy8mB96wd9wqq3WDXQwM4oU6r42Th37Db9fC \
  --base-image ellipsislabs/solana:1.18.11
```

This process may take some time, as it involves building the program in Docker and verifying that the computed hash matches the deployed program hash.
