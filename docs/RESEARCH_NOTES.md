# Research Notes — Technology Baseline

Research snapshot: 21 November 2026. Re-check every dependency and provider before implementation.

## Rust

Rust 1.100.0 was announced on 12 November 2026. The project pins that stable toolchain for the initial blueprint. Upgrade only through a reviewed change to `rust-toolchain.toml` and CI.

## PostgreSQL

PostgreSQL 19 was released on 19 November 2026. PostgreSQL 18.7 is the conservative initial production baseline for the MVP while testing driver, extension, backup, and migration compatibility with 19.

## P2P

The Rust implementation of libp2p is the preferred first candidate. Evaluate current transport and protocol capabilities before freezing the wire-level design.

## Blockchain

The Polkadot SDK is a Rust-based modular blockchain framework and is a candidate for a future custom chain. It is not selected. Do not write bespoke consensus in an early MVP.

## Sandbox

Wasmtime is suitable for executing WASM components with controlled host interfaces. Arbitrary source-code compilation and tests still require hardened OS-level isolation; Wasmtime alone is not a complete build sandbox.

## AI security

OWASP LLM risk categories particularly relevant to PANDORA include prompt injection, insecure output handling, training-data poisoning, sensitive-information disclosure, insecure tool/plugin design, excessive agency, and overreliance.

## FreeLLM

The NewAran/FreeLLM project currently describes an early-V1 self-hosted OpenAI-compatible gateway with Chat Completions and model listing endpoints, provider routing/fallback, and provider-specific limitations. Treat it as an optional adapter.

## Supply chain

Use SLSA provenance principles and Sigstore Cosign to make software origin and artifact signature verification auditable. These controls do not prove code correctness.

## Character continuity research and design questions

The character model should be evaluated against established identity, credential, avatar, asset licensing, and privacy standards. The Character Philosophy is a product requirement, not a claim that every technical standard preserves emotional meaning. The project must separately specify: (1) persistent identity references, (2) player-curated narrative/continuity data, (3) licensed assets, (4) world-local progress, and (5) destination portrayals.

Research questions for future ADRs:

- Which established credential format best represents issuer-signed achievements and provenance?
- Which avatar/3D formats preserve useful appearance data without claiming universal visual equivalence?
- How can selective disclosure reveal an achievement without exposing unrelated character history?
- How should encrypted continuity archives support export, key recovery, and deletion requests?
- How can a destination world prove that its adaptation preview matches the data it will actually import?
- What license metadata is required before an asset can travel between worlds?

Do not treat a content hash, a signed claim, or a DID as proof that a character's emotional significance is preserved. That is a product/UX responsibility expressed through consent, provenance, and continuity behavior.
