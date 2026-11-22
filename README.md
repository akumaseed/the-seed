# THE SEED

**The Open Network of Virtual Worlds**  
*One identity. Multiple worlds. A character is more than data.*

THE SEED is an early-stage open architecture project exploring interoperability between independent games and virtual worlds. The goal is to let participating worlds recognize portable player identities and character continuity while retaining their own engines, rules, economies, progression, and databases.

## The principle at the heart of THE SEED

**A character is not just another character. It is *your* character — the one you chose, shaped, cared for, and may feel is part of you.**

A character can carry years of choices, stories, achievements, friendships, creative expression, and personal meaning. THE SEED is designed to respect that bond. It must preserve the character's identity and player-approved continuity when compatible worlds connect, rather than reducing the character to a generic avatar, level, inventory, or database row.

This does not claim that game characters are sentient, and it does not override creators' intellectual-property rights. It means the player's relationship with the character is treated with care, consent, privacy, and continuity.

Read [`docs/CHARACTER_PHILOSOPHY.md`](docs/CHARACTER_PHILOSOPHY.md) for the detailed product and engineering requirements.

## The four pillars

- **THE SEED** — the complete network, protocols, SDKs, and ecosystem connecting worlds.
- **CARDINAL** — verifiable records, ledger research, trust, and consensus.
- **PANDORA** — AI-assisted diagnostics, self-healing workflows, and patch proposals.
- **YURI** — a provider-neutral gateway connecting PANDORA to external and local AI models.

## What we are exploring

- Peer-to-peer networking without a mandatory central server.
- Player-controlled identities and signed character passports.
- Character Identity Anchors and player-curated continuity manifests.
- Respectful world-specific portrayals that do not overwrite the canonical character identity.
- World discovery and capability negotiation.
- Visitor and migration transfer modes with a continuity preview.
- Engine-neutral protocol contracts.
- AI-assisted repair with strict sandboxing and independent approval.
- Signed, auditable software updates.

## Important status notice

**Early-stage research and architecture. Not production-ready.** The repository currently begins with specifications and design work. A production blockchain, public validator network, working game SDKs, universal character transfers, and autonomous global patch deployment are not claimed to exist. APIs and schemas may change.

## Start here

1. Read [`THE_SEED_MASTER_SPEC.md`](THE_SEED_MASTER_SPEC.md).
2. Read [`docs/CHARACTER_PHILOSOPHY.md`](docs/CHARACTER_PHILOSOPHY.md).
3. Read [`AGENTS.md`](AGENTS.md) before using an AI coding agent.
4. Review the initial implementation backlog in the master specification.
5. Open an issue or a Seed Enhancement Proposal (SEP) before changing foundational protocol decisions.

## Technology direction

Rust-first for trusted systems and networking; Python for isolated AI evaluation/orchestration; TypeScript for the developer dashboard; C# for Unity; C++ for Unreal; GDScript/C# for Godot. Exact dependencies are selected and pinned during implementation after checking current stable releases.

## Design principles

No single server, world operator, AI model, or original maintainer should have unilateral authority over the entire network. Character identity and continuity must also remain under explicit player control, with privacy and creator rights respected. These are design goals that must be demonstrated through independent operators, verified protocols, key separation, consent tests, governance, and security review—not assumed from an architecture diagram.

## Contributing

Contributions are welcome after the project governance, license, and security disclosure policies are finalized. Until then, use issues and discussions for research, specification review, and prototype feedback. Do not submit secrets, private player data, or exploit details in public issues. Any change touching character data must follow the Character Philosophy standard.
