![CI](https://github.com/cybermax4200/stellar-did-credit/actions/workflows/ci.yml/badge.svg)

# stellar-did-credit

stellar-did-credit is an open protocol built on Stellar and Soroban that lets individuals build a verifiable, self-sovereign credit identity on-chain. It combines decentralised identifiers (DIDs) with on-chain credit attestations so that any wallet address can accumulate a portable, tamper-proof credit history without relying on a centralised bureau.

The protocol addresses a fundamental gap in global financial inclusion: roughly 1.4 billion adults worldwide have no access to formal banking and therefore no way to prove creditworthiness to lenders, landlords, or employers. Because their financial behaviour — repaying informal loans, paying rent on time, running a small business — is never recorded in a system that others can verify, they are locked out of the credit economy entirely. stellar-did-credit gives those individuals a way to anchor real-world financial behaviour to a DID they control, making their credit history portable, auditable, and independent of any single institution.

| Component           | Status      |
| ------------------- | ----------- |
| identity-oracle     | ✅ Complete |
| credit-oracle       | 📋 Planned  |
| revocation-registry | 📋 Planned  |
| TypeScript SDK      | 📋 Planned  |
| CLI tool            | 📋 Planned  |

## Running tests

```bash
cargo test --workspace
```

This project is in early development. See CONTRIBUTING.md.

## Deployed contracts (Stellar testnet)

| Contract            | Address                                                  | Explorer                                                                                                          |
| ------------------- | -------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------- |
| identity-oracle     | CATORJPJOUUAH43UVY5WTVW4LF564XGINFPKXACD2DN2VDUXLPOEO26G | [view](https://stellar.expert/explorer/testnet/contract/CATORJPJOUUAH43UVY5WTVW4LF564XGINFPKXACD2DN2VDUXLPOEO26G) |
| credit-oracle       | CBMMX6GJPEGOAQWDXP37RUQMFKWHFH2Z7NXBQA52VV34D2WZHTWTQNAP | [view](https://stellar.expert/explorer/testnet/contract/CBMMX6GJPEGOAQWDXP37RUQMFKWHFH2Z7NXBQA52VV34D2WZHTWTQNAP) |
| revocation-registry | CDNQLXKKECN3AOWWA55KB5MLLDIKKP7S62O3SNVTBSREV3YQI6EAOVQ7 | [view](https://stellar.expert/explorer/testnet/contract/CDNQLXKKECN3AOWWA55KB5MLLDIKKP7S62O3SNVTBSREV3YQI6EAOVQ7) |
