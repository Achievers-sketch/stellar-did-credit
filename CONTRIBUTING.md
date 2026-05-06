# Contributing to stellar-did-credit

## Prerequisites

- Rust stable (`rustup update stable`)
- `stellar-cli` 21+
- Node.js 18+
- pnpm (`npm i -g pnpm`)

## Setup

```bash
git clone https://github.com/cybermax4200/stellar-did-credit.git
cd stellar-did-credit
cargo build --workspace
pnpm install
```

## Running tests

```bash
cargo test --workspace
```

## Drips Wave

This project uses [Drips Wave](https://drips.network/wave) for contributor rewards. Contributors earn USDC by resolving labeled issues during weekly sprints. Look for issues tagged `wave` to find available tasks.

## PR guidelines

- One issue per PR
- All tests must pass (`cargo test --workspace`)
- Clippy must be clean (`cargo clippy --workspace -- -D warnings`)
- Follow conventional commit format (see below)
- Reference the issue number in your PR description

## Commit format

```
type(scope): short description

feat(sdk): implement anchorDID wrapper
fix(identity-oracle): handle empty vc list in is_verified
docs(contributing): add wave program section
test(revocation-registry): add batch revoke edge case
chore(deps): bump soroban-sdk to 25.3.1
```

Types: `feat`, `fix`, `docs`, `test`, `refactor`, `chore`
