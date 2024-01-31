# hello world solana

Simple hello world program for the solana blockchain

> Make sure you have solana-cli tools installed and local dev environment setup.
> Refer: https://solana.com/developers/guides/getstarted/setup-local-development

### Build program

```bash
cargo build-sbf
```

### Run solana-test-validator

```bash
solana-test-validator
```

### Get some SOL for deployment

```bash
solana airdrop <AMOUNT_IN_SOL>
```

> NOTE: Pass number of SOL in place of <AMOUNT_IN_SOL>

### Deploy program

> Make sure solana-test-validator is running in a different terminal instance

> Make sure program is built and you have enought SOL to deploy

```bash
solana program deploy ./target/deploy/hello_world_solana.so
```

This will give you the deployed program id.

Example:
`Program Id: 8ud8ATnRpt4s89vFVaieug1ifmf3EhU6mnbvQkddEfXc`

### Check balance of your account

```bash
solana balance
```
