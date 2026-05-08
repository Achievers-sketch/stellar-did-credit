![CI](https://github.com/cybermax4200/stellar-did-credit/actions/workflows/ci.yml/badge.svg)

# stellar-did-credit

stellar-did-credit is an open protocol built on Stellar and Soroban that lets individuals build a verifiable, self-sovereign credit identity on-chain. It combines decentralised identifiers (DIDs) with on-chain credit attestations so that any wallet address can accumulate a portable, tamper-proof credit history without relying on a centralised bureau.

The protocol addresses a fundamental gap in global financial inclusion: roughly 1.4 billion adults worldwide have no access to formal banking and therefore no way to prove creditworthiness to lenders, landlords, or employers. Because their financial behaviour — repaying informal loans, paying rent on time, running a small business — is never recorded in a system that others can verify, they are locked out of the credit economy entirely. stellar-did-credit gives those individuals a way to anchor real-world financial behaviour to a DID they control, making their credit history portable, auditable, and independent of any single institution.

## Status

| Component           | Status                                      |
| ------------------- | ------------------------------------------- |
| identity-oracle     | ✅ Complete                                 |
| credit-oracle       | ✅ Complete                                 |
| revocation-registry | ✅ Complete                                 |
| TypeScript SDK      | 🚧 In progress (`getScore` done, rest open) |
| CLI tool            | 📋 Planned                                  |

## Quick start

```bash
git clone https://github.com/cybermax4200/stellar-did-credit
cd stellar-did-credit
cargo test --workspace
pnpm install
```

All 21 contract tests should pass. The TypeScript SDK lives in `packages/sdk` — see [packages/sdk/README.md](packages/sdk/README.md) for usage.

## Deployed contracts (Stellar testnet)

| Contract            | Address                                                  | Explorer                                                                                                          |
| ------------------- | -------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------- |
| identity-oracle     | CATORJPJOUUAH43UVY5WTVW4LF564XGINFPKXACD2DN2VDUXLPOEO26G | [view](https://stellar.expert/explorer/testnet/contract/CATORJPJOUUAH43UVY5WTVW4LF564XGINFPKXACD2DN2VDUXLPOEO26G) |
| credit-oracle       | CBMMX6GJPEGOAQWDXP37RUQMFKWHFH2Z7NXBQA52VV34D2WZHTWTQNAP | [view](https://stellar.expert/explorer/testnet/contract/CBMMX6GJPEGOAQWDXP37RUQMFKWHFH2Z7NXBQA52VV34D2WZHTWTQNAP) |
| revocation-registry | CDNQLXKKECN3AOWWA55KB5MLLDIKKP7S62O3SNVTBSREV3YQI6EAOVQ7 | [view](https://stellar.expert/explorer/testnet/contract/CDNQLXKKECN3AOWWA55KB5MLLDIKKP7S62O3SNVTBSREV3YQI6EAOVQ7) |

Full deployment record: [deployments.testnet.json](deployments.testnet.json)

## Documentation

- [Architecture overview](docs/architecture.md) — contract design, storage layout, data flow
- [Scoring specification](docs/scoring-spec.md) — formula, worked examples, edge cases
- [DID spec](docs/did-spec.md) — DID document format and anchoring conventions

## Contributing

This project participates in the Stellar Wave Program on Drips. Contributors earn USDC rewards for resolving labeled issues. See [CONTRIBUTING.md](CONTRIBUTING.md) and the open issues below to get started.

### Open issues

| #                                                                   | Title                                                            | Label              |
| ------------------------------------------------------------------- | ---------------------------------------------------------------- | ------------------ |
| [#7](https://github.com/cybermax4200/stellar-did-credit/issues/7)   | Implement `anchorDID()` in TypeScript SDK                        | `good first issue` |
| [#8](https://github.com/cybermax4200/stellar-did-credit/issues/8)   | Implement `issueVC()` in TypeScript SDK                          | `good first issue` |
| [#9](https://github.com/cybermax4200/stellar-did-credit/issues/9)   | Implement `verifyVC()` and `isVerified()` in TypeScript SDK      | `good first issue` |
| [#10](https://github.com/cybermax4200/stellar-did-credit/issues/10) | Build CLI tool (`stellar-did` binary)                            | `enhancement`      |
| [#11](https://github.com/cybermax4200/stellar-did-credit/issues/11) | Cross-contract: credit-oracle calls identity-oracle for vc_count | `enhancement`      |
