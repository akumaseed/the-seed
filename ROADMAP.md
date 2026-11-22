# THE SEED Roadmap

Status: proposed planning baseline. Dates are intentionally not promised.

1. **Architecture foundation:** schemas, identifiers, Character Philosophy, identity-anchor/continuity contracts, threat model, ADRs, CI, Rust workspace.
2. **Local node:** local development ledger, signatures, world registration, PostgreSQL persistence.
3. **P2P test:** multiple nodes, libp2p, discovery, signed messages, replay protection.
4. **Identity and passport:** player/world keys, Character Identity Anchor, player-curated Continuity Manifest, consent, signed claims, revocation.
5. **Visitor MVP:** two different sample worlds, portable character presentation, and a preview of preserved/adapted/omitted continuity.
6. **Migration:** saga, snapshots, authority epochs, recovery, fault injection, and no-loss character archive guarantees within declared retention/licensing limits.
7. **Cardinal testnet:** consensus ADR, independent operators, partition and Byzantine tests.
8. **Yuri/Pandora prototype:** provider gateway, optional FreeLLM, incident analysis, isolated patch generation.
9. **Global patch registry:** reviews, attestations, SBOM, signatures, canary and rollback.
10. **SDK ecosystem:** Unity, Unreal, Godot, generic HTTP SDK, developer dashboard, conformance suite.

A production mainnet must not be scheduled until consensus, security, recovery, governance, and independent operation have been demonstrated.


## Character-care release gate

No milestone that changes character schemas, export, storage, AI context, or transfer behavior is complete until it has tests for identity-anchor preservation, provenance, consent, private continuity defaults, player-visible adaptation summaries, and cancellation/recovery.

The Character Philosophy is a cross-cutting requirement for every phase, not a late cosmetic feature. Identity anchors, continuity privacy, provenance, player approval, and respectful adaptation must be represented in the first schemas and tests.
