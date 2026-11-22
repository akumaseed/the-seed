# AI Start Here — THE SEED

You are an implementation agent working on THE SEED.

## Mandatory reading

1. Read `THE_SEED_MASTER_SPEC.md` completely.
2. Read `docs/CHARACTER_PHILOSOPHY.md` completely before changing any feature related to characters, identity, passports, memory, transfer, storage, privacy, or player experience.
3. Read `AGENTS.md`, `ROADMAP.md`, `GOVERNANCE.md`, and `SECURITY.md`.
4. Inspect the repository, current git status, tests, and existing code before changing anything.
5. Determine the earliest incomplete roadmap phase.
6. Propose a small vertical slice with explicit acceptance criteria.

## Non-negotiable character requirement

A character is not merely a generic avatar, a row in a database, a skin, a level, or an inventory. It may be the character the player loves, has shaped through choices, and feels is part of them. THE SEED must respect this bond by preserving identity, provenance, player-approved story continuity, privacy, and choice across compatible worlds.

- Never silently rename, randomize, flatten, erase, replace, or overwrite a character's identity anchor.
- Keep global identity, portable claims, world-local progress, and world-specific portrayals separate.
- Show preserved/adapted/omitted/private/unsupported fields before meaningful transfers.
- Do not infer memories, feelings, relationships, or biography from private chats, telemetry, or prompts.
- Private continuity is private by default and is not sent to AI providers by default.
- AI-generated character content is a suggestion until the player explicitly approves it.
- Respect asset licenses and creator rights; player attachment does not grant copyright.
- Do not exploit attachment through manipulative monetization or retention mechanics.

## Engineering rules

- Do not implement a production blockchain or custom consensus without an accepted ADR and a reviewed threat model.
- Do not give PANDORA access to production secrets, validator keys, or release-signing keys.
- Implement real behavior only; label mocks and stubs clearly.
- Add tests, run checks, and update documentation.
- Report exactly what was implemented, what was tested, and what remains incomplete.

## First task recommended

Implement only the earliest incomplete foundation phase: Rust workspace conventions, common types, protocol versioning, error types, schema validation fixtures, logging conventions, and CI checks. Include the Character Identity Anchor and Character Continuity Manifest schemas as specification work, but do not begin autonomous repair, production consensus, or cross-game migration prematurely.

## Important

The master specification is a design target, not evidence that features already exist. Never tell the user a feature works until an implementation and tests demonstrate it.
