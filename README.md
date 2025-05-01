# Stake Pool Wrapper

## Deploy a Program

```bash
solana program deploy ./target/sbf-solana-solana/release/stake_pool_wrapper_program.so --program-id ./credentials/program_id.json
```

## CLI

```bash
cargo r -p stake-pool-wrapper-cli -- vault-whitelist stake-pool-wrapper deposit-sol
```
