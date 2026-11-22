# Contributing to THE SEED

THE SEED is in its early architecture and research stage. Contributions should improve clarity, correctness, security, interoperability, or demonstrable prototype behavior.

## Before contributing

1. Read `THE_SEED_MASTER_SPEC.md` and `AGENTS.md`.
2. Read `docs/CHARACTER_PHILOSOPHY.md`; character care and continuity are core requirements.
3. Search existing issues and proposals.
4. For a foundational change, write a Seed Enhancement Proposal (SEP) or ADR first.
5. Keep pull requests focused and explain the reason for the change.

## Code expectations

- Rust code uses stable Rust and the 2024 edition unless an accepted ADR says otherwise.
- Add tests for every behavior change.
- Do not add dependencies without checking maintenance, license, security, and necessity.
- Do not introduce custom cryptography or unreviewed consensus code.
- Do not add secrets, personal data, production logs, or real player save files.
- AI-generated code is acceptable only when the contributor takes responsibility for it and verifies it.
- Character-related changes must preserve identity anchors, provenance, player-approved continuity, privacy, and consent.
- Do not make silent or lossy character transformations; document and test transfer previews and cancellation.
- Do not send private character memories or notes to AI providers by default.

## Pull request checklist

- [ ] Scope is small and explained.
- [ ] Tests added or updated.
- [ ] Formatting and lint checks pass.
- [ ] Documentation updated.
- [ ] Security/privacy impact considered.
- [ ] No secrets or private data included.
- [ ] Compatibility impact explained.
- [ ] Character continuity impact reviewed, if applicable.
- [ ] Player consent, privacy, provenance, and creator-rights impact reviewed, if applicable.

## Security reports

Do not publish exploitable vulnerabilities in public issues. Use the private security reporting route configured by repository maintainers. Until that route exists, do not publish sensitive exploit details; contact the project owner through an independently verified private channel.
