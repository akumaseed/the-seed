# AI Agent Instructions — THE SEED

Before changing this repository, read `THE_SEED_MASTER_SPEC.md` and `docs/CHARACTER_PHILOSOPHY.md` completely.

## Product principle agents must preserve

A character is not just generic game data. It may be the character a player loves, has shaped through choices, and experiences as part of themselves within a story. THE SEED must preserve character identity, provenance, player-approved continuity, privacy, and player choice. This is a functional requirement that affects schemas, APIs, transfers, storage, UX, security, and tests.

## Rules

- Treat the master specification as the current product/architecture authority unless a newer accepted ADR supersedes it.
- Treat `docs/CHARACTER_PHILOSOPHY.md` as normative for character-related work.
- Do not claim planned features are implemented.
- Implement one small, testable vertical slice at a time.
- Inspect the repository and current milestone before editing.
- Rust is the primary trusted-systems language. Use stable Rust and the 2024 edition unless an accepted ADR says otherwise. Python is permitted for isolated AI orchestration/evaluation. TypeScript is for the dashboard. C#, C++, and GDScript are for game SDKs.
- Do not invent consensus, cryptography, identity, or transfer semantics.
- Do not put private data or private character continuity on a public ledger.
- Never give PANDORA direct access to production credentials, release-signing keys, or validator keys.
- AI-generated code must be sandboxed, tested, reviewed, and signed by a separate release process.
- Any change to consensus, identity, cryptography, transfer authority, global patch policy, governance, or character continuity schema requires an ADR or SEP as specified.
- Never silently rename, erase, flatten, genericize, or overwrite a canonical character identity.
- Never infer character memories, biography, relationships, or feelings from private chat or telemetry. AI-generated story content remains non-canonical until player approval.
- Private continuity is not sent to YURI/model providers by default.
- A transfer must disclose what is preserved, adapted, omitted, private, or unsupported and allow the player to cancel.
- Respect game creator rights and asset licenses. A player's emotional attachment does not automatically grant redistribution rights.
- Do not use character attachment as a metric for manipulative monetization or engagement optimization.
- Add tests, update docs, and report exact validation commands/results.

## Completion report

Report scope, files changed, design decisions, tests run, limitations, and next recommended task. For character-related work, state explicitly how character identity and continuity were preserved.
