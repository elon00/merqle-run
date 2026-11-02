# Verification status — Batch 019 engine (historical alias: merqle-run)

This repository has stable GitHub repository ID `1425216878`, but its current name
is `merqle-run`, not the approved QMOOSA-1000 canonical name
`qmoosa-pqc-merkle-engine`. Identity mapping needs owner approval.

## What the proposed CI checks actually do

- `cargo fmt --all -- --check` checks Rust formatting.
- `python3 tests/reality_check_harness.py` invokes real local Rust unit tests
  using `cargo test --offline --all-targets`. It fails if cargo is absent.
- The separate CodeQL workflow attempts source analysis for Rust, Python and
  GitHub Actions with explicit initialization and security-event permissions.

These checks **do not** verify real QuickNode connectivity, NIST/FIPS
cryptography, Conway's Game of Life, smart-contract correctness, network
consensus, financial settlement, or public deployment. The Rust program is
a local bit-mixing demonstration, not a production automaton.

Older output claiming `100/100 VERIFIED` came from unconditional print
statements and trivial assertions. Such output must not be treated as
implementation, security approval or deployment evidence.

Status: **IMPLEMENTED unverified; SECURITY_APPROVED NO; DEPLOYMENT_READY NO**.
Do not merge or change repository visibility/identity until reviewed.
