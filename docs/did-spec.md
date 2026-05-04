# Stellar DID Method Specification

This document defines the `did:stellar` method, which utilizes the Stellar network and IPFS to provide decentralized identity.

## 1. DID Method Format

The DID method name is `stellar`. The method-specific identifier is composed of the network identifier and the Stellar account address (G-address).

**Format:** `did:stellar:<network>:<account_address>`

- **network**: Either `mainnet` or `testnet`.
- **account_address**: A standard Stellar public key (e.g., `G...`).

**Example:**
`did:stellar:testnet:GABC123...`

## 2. DID Document Structure

DID documents are JSON-LD files stored on IPFS. They MUST contain the `id`, `verificationMethod`, and `authentication` fields.

```json
{
  "@context": [
    "https://www.w3.org/ns/did/v1",
    "https://w3id.org/security/suites/ed25519-2020/v1"
  ],
  "id": "did:stellar:testnet:GABC123...",
  "verificationMethod": [
    {
      "id": "did:stellar:testnet:GABC123...#keys-1",
      "type": "Ed25519VerificationKey2020",
      "controller": "did:stellar:testnet:GABC123...",
      "publicKeyMultibase": "z6MkmL..."
    }
  ],
  "authentication": [
    "did:stellar:testnet:GABC123...#keys-1"
  ]
}
```

## 3. DID Document Anchoring

The DID document is anchored to the Stellar blockchain via the `identity-oracle` smart contract.

The IPFS Content Identifier (CID) of the DID document is stored in the contract's storage under the following key:
`DataKey::DIDDocument(Address)`

Subjects anchor their DID document by calling:
`anchor_did(env: Env, subject: Address, did_doc_cid: String)`

## 4. Key Rotation and Updates

A subject can update their DID document or rotate keys by:
1. Generating a new DID document.
2. Uploading the new document to IPFS to obtain a new CID.
3. Calling `anchor_did` on the `identity-oracle` contract with the new CID.

The contract overwrites the previous CID, ensuring only the latest version is active.

## 5. Resolution

To resolve a `did:stellar` identifier:

1. **Extract Address**: Parse the DID string to extract the Stellar `<account_address>`.
2. **Query Contract**: Call the `identity-oracle` contract to retrieve the CID associated with the subject address.
3. **Fetch from IPFS**: Use the retrieved CID to fetch the DID document from an IPFS gateway or node.
4. **Verify**: Ensure the `id` field in the document matches the resolved DID.
