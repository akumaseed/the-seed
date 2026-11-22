# Security Policy

THE SEED is an early-stage project and is not yet production-ready. Do not use it to protect valuable assets or real player data.

## Supported versions

There are no releases yet. Until a first release exists, only the latest commit on `main` is considered for security fixes. A versioned support policy will be published together with the first release.

## Reporting a vulnerability

Do not publish exploitable details in public issues or discussions.

Report vulnerabilities privately through GitHub Private Vulnerability Reporting: open the repository's **Security** tab and choose **Report a vulnerability** (direct link: <https://github.com/akumaseed/the-seed/security/advisories/new>). No dedicated security e-mail address exists yet.

Never include in a report or a public issue: tokens, passwords, private keys, provider credentials, personal data, private player data, or game save files. Describe where a credential was exposed instead of copying its value.

## Response goals

These are goals for a small volunteer project, not guaranteed service levels:

- Acknowledge a report within 7 days.
- Share an initial assessment within 30 days.
- Coordinate disclosure with the reporter: details are published only after a fix or mitigation is available, or when both sides agree.

## Compromised credentials

If a credential, key, or token is exposed, revoke or rotate it immediately at its issuer. Removing it from the latest commit does not invalidate it, because it remains in git history and may already have been copied.

## Security scope

High-priority areas include CARDINAL validation/consensus, key management, transfer authority, SEED MESH message parsing, SDK trust boundaries, PANDORA tool permissions, YURI provider credentials, sandbox isolation, and release signing.

## Character continuity and privacy

Character history, selected memories, relationships, and player-authored story records can be sensitive even when they concern fictional characters. Keep private continuity off public ledgers, out of public logs, and out of YURI/model-provider context by default. Use scoped consent, encryption, selective disclosure, and retention limits. A character's identity anchor must not be overwritten by a destination world without player authorization.

## AI-generated fixes

An AI-generated patch is not trusted by default. An AI-generated character biography, memory, relationship, or identity change is also untrusted and non-canonical until the player explicitly approves it. It must pass deterministic tests, static analysis, dependency review, security review appropriate to its impact, build provenance generation, artifact signing, and the required release approval.

## Production warning

No production security guarantee is made by this architecture document. Security claims require implementation evidence, threat modeling, independent testing, and operational review.
