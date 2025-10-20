# Test Keypair Fixture

This directory should contain test keypair files for Solana integration tests.

## Setup

1. **Generate a test keypair:**
   ```bash
   solana-keygen new --outfile tests/fixtures/test-keypair.json --no-bip39-passphrase
   ```

2. **Fund the test wallet on Devnet:**
   ```bash
   # Get your test wallet address
   solana-keygen pubkey tests/fixtures/test-keypair.json
   
   # Airdrop 2 SOL to test wallet
   solana airdrop 2 <YOUR_PUBKEY> --url devnet
   ```

3. **Verify balance:**
   ```bash
   solana balance <YOUR_PUBKEY> --url devnet
   ```

## Security

⚠️ **IMPORTANT:** Never commit real keypairs with mainnet funds!

- Test keypairs in this directory should ONLY be used on Devnet
- Add `test-keypair.json` to `.gitignore`
- Only use for automated testing

## .gitignore

Make sure these files are ignored:
```
tests/fixtures/*.json
tests/fixtures/*-keypair.json
!tests/fixtures/.gitkeep
```
