# THE SEED — Character Philosophy & Continuity Standard

**Document ID:** `SEED-CHARACTER-001`  
**Status:** Product principle and proposed implementation standard  
**Applies to:** THE SEED, SEED IDENTITY, SEED PASSPORT, SEED TRANSIT, SEED WORLDS, CARDINAL, PANDORA, YURI, SDKs, dashboards, and integrated worlds.

> **Um personagem não é apenas um personagem qualquer. É o seu personagem — aquele que você adora e que pode sentir como uma parte de você.**
>
> **A character is not just another character. It is *your* character — the one you love, shape, carry through stories, and may feel is part of you.**

This is a foundational product requirement, not a decorative slogan.

## 1. Why this principle exists

A character in a game can become much more than a collection of attributes. A player may spend years with the same character, make meaningful choices through them, build a visual identity, complete difficult journeys, form friendships, create stories, and associate important personal experiences with that character. The character can become a chosen companion, a creative expression, and a persistent part of the player's story.

THE SEED is being designed so that independent virtual worlds can recognize a persistent character identity. That technical capability must be built around the human meaning of the character. The system must not treat the character as an interchangeable avatar slot or disposable profile record.

The emotional bond is real as an experience of the player. This document does **not** claim that a software character is sentient, conscious, or capable of human feelings. THE SEED must not fabricate sentience or use simulated affection to manipulate the player. Its responsibility is to respect the player's meaning, choices, and control.

## 2. The promise

THE SEED aims to let a player carry the identity and chosen continuity of a character across compatible worlds without forcing every world to use the same mechanics, art style, engine, or progression system.

The character should remain recognizable as the same character even when a destination world must adapt its class, abilities, equipment, or visual presentation. A fantasy mage may be represented as an ether engineer in a science-fiction world, but the destination should preserve the identity elements the player chose to carry. A conversion must be described honestly as an adaptation, not falsely presented as a perfect copy.

The promise is not that everything transfers. The promise is that the player is told what transfers, what changes, what stays private, what cannot move, and why—before committing to a meaningful change.

## 3. What makes a character meaningful in THE SEED

A character may contain several kinds of continuity:

1. **Identity:** stable character ID, player-chosen name, chosen pronouns or presentation where supported, and recognizable identity references.
2. **Appearance:** a player-selected canonical look, signature visual elements, approved variants, and references to authorized assets.
3. **Story:** player-authored biography, selected milestones, important decisions, titles, and narrative facts the player chose to preserve.
4. **Journey:** worlds visited, major achievements, origin references, and a history of approved adaptations.
5. **Relationships:** optional guild, party, friendship, or character relationship markers—but only when all affected people have consented to the relevant sharing.
6. **Expression:** outfits, emblems, animations, voice/presentation preferences, and other elements that help the player recognize the character.
7. **Local life:** game-specific quests, items, housing, levels, and mechanics that may remain inside a particular world.

Not every player will want all of these. No field is mandatory merely because the schema supports it.

## 4. Core architecture concepts

### 4.1 Character Identity Anchor

Every portable character SHOULD have a stable Character Identity Anchor. It connects the global `character_id` to a versioned, player-approved identity and continuity record. It should be durable across worlds and must be distinct from any one game's local save.

The anchor is not a single public database row containing the player's life story. It can reference signed manifests and encrypted off-ledger data. The public network should learn only what it needs to verify identity, provenance, consent, and transfer authority.

### 4.2 World portrayals

Each game may maintain a separate portrayal of the same character. A portrayal may adapt the character to the world's art direction and mechanics. It must be scoped to that world and must not silently overwrite the global anchor.

A world can say, for example: "In this world, your character's arcane magic is represented as ether technology." It must not silently rename the character, replace their chosen appearance, erase milestones, or claim that local progression is the character's universal identity.

### 4.3 Character Continuity Manifest

The player may curate a manifest of entries such as biography, milestones, titles, visual references, or selected story memories. Every entry must have:

- Unique entry ID and schema version.
- Content type and content reference.
- Author/issuer and provenance.
- Visibility: private, selected worlds, or public.
- Portability: local-only, selected destinations, or globally portable.
- Consent state and, where relevant, consent from other participants.
- Retention, export, deletion, and revocation rules.
- AI-use permission for a specific purpose; default is **not shared with AI providers**.

A continuity manifest is optional and player-curated. It is not a raw transcript archive.

## 5. The rules THE SEED must follow

### Rule 1 — Never reduce a character to statistics

Level, class, attributes, inventory, and combat rating are local representations. They are not the total identity of a character. If mechanics cannot transfer, preserve identity and story where allowed rather than inventing a misleading stat conversion.

### Rule 2 — Never silently overwrite canonical identity

A world-local portrayal can change independently. The Character Identity Anchor changes only through an authorized, versioned action. Every material change must have provenance and a recoverable prior version where practical.

### Rule 3 — Never force a generic replacement

If a destination world cannot represent a signature visual element, name, title, or story detail, it must disclose the limitation. It should offer a safe fallback (such as a cosmetic/narrative token or an archive reference) rather than silently replacing the character with a generic avatar.

### Rule 4 — Always preview meaningful adaptations

Before a transfer, show which elements are preserved, adapted, omitted, private, or unsupported. The player must be able to cancel without losing the source character.

### Rule 5 — Keep private memories private

Raw chat, direct messages, roleplay transcripts, voice recordings, journals, personal notes, and inferred emotions must not become character memories by default. Only player-authored or explicitly approved entries may become persistent continuity data.

### Rule 6 — Respect other people in shared stories

One player cannot export another player's private messages, personal data, or relationship assertions. Shared memories and relationship markers need consent and privacy scope for every affected participant.

### Rule 7 — Preserve provenance

The system must distinguish original game-issued facts, player-authored facts, independently verified claims, destination-specific adaptations, and AI-generated suggestions. An AI suggestion is never canonical until the player approves it.

### Rule 8 — Respect creator rights

Player attachment does not automatically transfer copyright or redistribution rights. Asset licenses, game terms, creator attribution, and world policies remain in force. A passport cannot grant rights the player does not have.

### Rule 9 — Do not exploit attachment

Do not use a character's emotional meaning to create coercive monetization, deceptive scarcity, manipulative retention loops, or pressure to share private information. Do not sell continuity data or use it for model training without separate, informed, revocable consent.

### Rule 10 — Make preservation and exit possible

Where technically and legally supported, provide export/archive tools for a world closure, account change, or player departure. Explain limits honestly. Never promise recovery of data that was never exported or that an operator irretrievably destroyed.

## 6. Transfer experience requirements

Every visitor or migration flow must provide a continuity summary with at least these categories:

| Category | Required disclosure |
|---|---|
| Name and identity | Preserved, locally varied, or unsupported |
| Appearance | Preserved, adapted, or unavailable |
| Signature assets | License and compatibility status |
| Titles and achievements | Verified, adapted, or omitted |
| Story and milestones | Shared, private, local-only, or unsupported |
| Relationships | Consent and visibility status |
| Level and abilities | Destination conversion rules |
| Inventory and currency | Imported, converted, cosmetic-only, or blocked |
| Source world record | Retained, archived, or subject to stated policy |

A visitor session should be offered when migration is too invasive or irreversible. A migration must not destroy the original archive merely because active authority moved to a new world.

## 7. Privacy and data boundaries

Character continuity can reveal intimate preferences, relationships, creative work, and personal experiences. Treat it as potentially sensitive even when it concerns a fictional character.

- Public ledger: identifiers/commitments required for verification only; no private text or personal content.
- Player device: private keys and private notes where possible.
- Encrypted off-ledger storage: optional private continuity entries and export packages.
- World database: world-local game state under the operator's policy.
- AI context: only the minimum approved excerpts for a specific task; default no-sharing.
- Analytics: aggregate and minimize; never use attachment intensity as an engagement target.

Players must have clear controls for visibility, portability, AI processing, export, and deletion/revocation where feasible.

## 8. PANDORA and YURI rules

PANDORA may detect broken continuity handling, schema regressions, accidental data disclosure, or transfer bugs. It may create tests and propose a patch. It must not invent or alter a character's biography, relationships, memories, or identity to make a test pass.

YURI must enforce data classification before forwarding any context to a model provider. Private continuity entries are not sent to FreeLLM or any upstream provider unless the player has explicitly allowed the specific use and the provider's retention/data-processing policy has been assessed. Model output must be treated as untrusted suggestions.

AI-generated text may be offered as a draft. It becomes part of the character's canonical story only when the player explicitly accepts it.

## 9. Developer and SDK requirements

SDKs SHOULD expose operations with explicit names such as:

- `get_character_identity_anchor`
- `get_world_portrayal`
- `create_continuity_entry`
- `update_continuity_entry`
- `set_continuity_visibility`
- `set_continuity_portability`
- `request_transfer_preview`
- `approve_transfer`
- `export_character_archive`
- `revoke_continuity_consent`

Exact API names are implementation decisions, but API design must preserve these distinctions. Avoid one generic `sync_character()` method that silently uploads every field.

## 10. Acceptance tests

A feature touching character data should include tests proving:

1. A destination world cannot overwrite the global identity anchor through a local update.
2. A transfer preview clearly labels preserved, adapted, omitted, private, and unsupported data.
3. The player can cancel a transfer without losing the source instance.
4. Private continuity entries are excluded from default exports and AI context.
5. A player can explicitly select a continuity entry for a specific destination.
6. Shared relationship records cannot be exported without required participant consent.
7. An AI-generated biography or memory remains a suggestion until accepted.
8. An unsupported asset does not silently become a different asset without disclosure.
9. Character archive/export retains provenance and license metadata.
10. A world closure flow explains which data can and cannot be recovered.

## 11. Language for the project

Preferred Portuguese expression:

> **Seu personagem não é apenas um personagem qualquer. É o seu personagem — aquele que você construiu, escolheu, cuidou e que pode ser uma parte de você. A The Seed existe para respeitar essa história e permitir que ela continue em outros mundos.**

Preferred English expression:

> **Your character is more than a profile or an avatar. It is the character you chose, shaped, cared for, and may feel is part of you. THE SEED exists to respect that story and help it continue across worlds.**

Use this meaning consistently in the README, website, launch materials, product UX, architecture decisions, SDK behavior, privacy design, and AI-agent instructions. Do not turn it into a claim that characters are conscious or into a promise that every game asset can legally be transferred.

---

**Core reminder for every contributor:** preserve the character, respect the player, tell the truth about compatibility, and never treat a meaningful identity as disposable data.
