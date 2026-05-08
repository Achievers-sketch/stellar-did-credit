# @stellar-did-credit/sdk

TypeScript SDK for the Stellar DID Credit Protocol.

## Installation

```bash
npm install @stellar-did-credit/sdk
```

## Usage

```typescript
import { StellarDIDCreditSDK } from "@stellar-did-credit/sdk";

const sdk = new StellarDIDCreditSDK({
  identityOracleId: "C...",
  creditOracleId: "C...",
  revocationRegistryId: "C...",
  networkPassphrase: "Test SDF Network ; September 2015",
  rpcUrl: "https://soroban-testnet.stellar.org",
});

const score = await sdk.getScore("G...");
console.log(score.score); // e.g. 612
```

## API

### `getScore(subjectAddress: string): Promise<ScoreRecord>`

Fetches the on-chain credit score for a subject address. Uses a read-only simulation — no signing or fees required.

```typescript
interface ScoreRecord {
  score: number; // 300–850
  lastUpdated: number; // ledger timestamp
  vcCount: number; // number of verified credentials
  repaymentRate: number; // basis points (0–10000)
  txVolume30d: bigint; // 30-day transaction volume in stroops
}
```

### Other methods (coming soon)

- `anchorDID(subjectKeypair, didDocCid)` — anchor a DID document CID on-chain
- `issueVC(issuerKeypair, subjectAddress, vcHash)` — anchor a verifiable credential
- `verifyVC(subjectAddress, vcHash)` — check if a specific VC is valid
- `isVerified(subjectAddress)` — check if a subject has any active VC

## Testnet contract addresses

See [`deployments.testnet.json`](../../deployments.testnet.json) at the repo root.
