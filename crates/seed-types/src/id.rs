//! Namespaced opaque identifiers.
//!
//! # Format (proposed, Phase 0)
//!
//! ```text
//! seed:<namespace>:<opaque>
//! ```
//!
//! - `namespace` is one of [`IdNamespace`]. Identity domains are separate: a key or
//!   identifier from one domain never authorizes another domain.
//! - `opaque` is 1–128 characters of lowercase ASCII letters, digits and `-`, and it
//!   must not start or end with `-`. It carries **no meaning**: never encode names,
//!   e-mail addresses or other personal data in it.
//!
//! How opaque values are generated (random, content-derived, ...) is not decided
//! here and must be fixed by an architecture decision before identifiers are
//! issued on a shared network.

use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

/// Prefix shared by every THE SEED identifier.
pub const ID_PREFIX: &str = "seed";

/// Maximum length of the opaque part of an identifier.
pub const MAX_OPAQUE_LEN: usize = 128;

/// Identity domains. Each domain is separate and is not interchangeable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IdNamespace {
    /// Player/account identity (a privacy-preserving reference, not legal identity).
    Identity,
    /// Character identity, distinct from the player and from any world's local save.
    Character,
    /// World (game) identity.
    World,
    /// Network node identity.
    Node,
    /// Hub/operator identity.
    Hub,
    /// Validator identity.
    Validator,
    /// Release-signing identity.
    Release,
    /// Consent grant.
    Consent,
    /// Reference to player-approved continuity content.
    Continuity,
}

impl IdNamespace {
    /// All namespaces, in declaration order.
    pub const ALL: [IdNamespace; 9] = [
        IdNamespace::Identity,
        IdNamespace::Character,
        IdNamespace::World,
        IdNamespace::Node,
        IdNamespace::Hub,
        IdNamespace::Validator,
        IdNamespace::Release,
        IdNamespace::Consent,
        IdNamespace::Continuity,
    ];

    /// The stable wire name of the namespace.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            IdNamespace::Identity => "identity",
            IdNamespace::Character => "character",
            IdNamespace::World => "world",
            IdNamespace::Node => "node",
            IdNamespace::Hub => "hub",
            IdNamespace::Validator => "validator",
            IdNamespace::Release => "release",
            IdNamespace::Consent => "consent",
            IdNamespace::Continuity => "continuity",
        }
    }

    fn from_wire(s: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|ns| ns.as_str() == s)
    }
}

impl fmt::Display for IdNamespace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Why an identifier was rejected.
///
/// Error messages never echo the rejected opaque value, so a malformed identifier
/// that accidentally contains personal data is not copied into logs.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum IdError {
    /// The value does not have the `seed:<namespace>:<opaque>` shape.
    #[error("identifier must have the form `seed:<namespace>:<opaque>`")]
    Malformed,
    /// The namespace is not one of the known identity domains.
    #[error("unknown identifier namespace")]
    UnknownNamespace,
    /// The identifier belongs to a different identity domain.
    #[error("identifier namespace `{found}` is not accepted here; expected `{expected}`")]
    NamespaceMismatch {
        /// Namespace required by the caller.
        expected: IdNamespace,
        /// Namespace found in the identifier.
        found: IdNamespace,
    },
    /// The opaque part is empty or longer than [`MAX_OPAQUE_LEN`].
    #[error("identifier opaque part must be 1-{MAX_OPAQUE_LEN} characters")]
    InvalidLength,
    /// The opaque part contains characters outside the allowed set.
    #[error(
        "identifier opaque part may contain only a-z, 0-9 and '-', and must not start or end with '-'"
    )]
    InvalidCharacters,
}

/// A validated identifier of any namespace.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SeedId {
    namespace: IdNamespace,
    opaque: String,
}

impl SeedId {
    /// Builds an identifier from a namespace and an opaque value, validating the value.
    ///
    /// # Errors
    /// Returns [`IdError`] if the opaque value does not follow the identifier rules.
    pub fn new(namespace: IdNamespace, opaque: &str) -> Result<Self, IdError> {
        validate_opaque(opaque)?;
        Ok(Self {
            namespace,
            opaque: opaque.to_owned(),
        })
    }

    /// The identity domain of this identifier.
    #[must_use]
    pub fn namespace(&self) -> IdNamespace {
        self.namespace
    }

    /// The opaque part of this identifier.
    #[must_use]
    pub fn opaque(&self) -> &str {
        &self.opaque
    }
}

fn validate_opaque(opaque: &str) -> Result<(), IdError> {
    if opaque.is_empty() || opaque.len() > MAX_OPAQUE_LEN {
        return Err(IdError::InvalidLength);
    }
    let allowed = |b: u8| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-';
    if !opaque.bytes().all(allowed) || opaque.starts_with('-') || opaque.ends_with('-') {
        return Err(IdError::InvalidCharacters);
    }
    Ok(())
}

impl FromStr for SeedId {
    type Err = IdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.splitn(3, ':');
        let (Some(prefix), Some(ns), Some(opaque)) = (parts.next(), parts.next(), parts.next())
        else {
            return Err(IdError::Malformed);
        };
        if prefix != ID_PREFIX {
            return Err(IdError::Malformed);
        }
        let namespace = IdNamespace::from_wire(ns).ok_or(IdError::UnknownNamespace)?;
        Self::new(namespace, opaque)
    }
}

impl fmt::Display for SeedId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{ID_PREFIX}:{}:{}", self.namespace, self.opaque)
    }
}

impl Serialize for SeedId {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

impl<'de> Deserialize<'de> for SeedId {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

macro_rules! typed_id {
    ($(#[$doc:meta])* $name:ident => $ns:ident) => {
        $(#[$doc])*
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct $name(SeedId);

        impl $name {
            /// The only namespace this type accepts.
            pub const NAMESPACE: IdNamespace = IdNamespace::$ns;

            /// Builds the identifier from an opaque value.
            ///
            /// # Errors
            /// Returns [`IdError`] if the opaque value does not follow the identifier rules.
            pub fn new(opaque: &str) -> Result<Self, IdError> {
                SeedId::new(Self::NAMESPACE, opaque).map(Self)
            }

            /// The underlying untyped identifier.
            #[must_use]
            pub fn as_seed_id(&self) -> &SeedId {
                &self.0
            }
        }

        impl TryFrom<SeedId> for $name {
            type Error = IdError;

            fn try_from(id: SeedId) -> Result<Self, Self::Error> {
                if id.namespace() == Self::NAMESPACE {
                    Ok(Self(id))
                } else {
                    Err(IdError::NamespaceMismatch {
                        expected: Self::NAMESPACE,
                        found: id.namespace(),
                    })
                }
            }
        }

        impl From<$name> for SeedId {
            fn from(id: $name) -> SeedId {
                id.0
            }
        }

        impl FromStr for $name {
            type Err = IdError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                s.parse::<SeedId>()?.try_into()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(f)
            }
        }

        impl Serialize for $name {
            fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                self.0.serialize(serializer)
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                let s = String::deserialize(deserializer)?;
                s.parse().map_err(serde::de::Error::custom)
            }
        }
    };
}

typed_id!(
    /// Privacy-preserving reference to a player/account identity.
    PlayerIdentityRef => Identity
);
typed_id!(
    /// Stable character identifier, distinct from the player and from any world save.
    CharacterId => Character
);
typed_id!(
    /// World (game) identifier.
    WorldId => World
);
typed_id!(
    /// Network node identifier.
    NodeId => Node
);
typed_id!(
    /// Hub/operator identifier.
    HubId => Hub
);
typed_id!(
    /// Validator identifier.
    ValidatorId => Validator
);
typed_id!(
    /// Release-signing identity identifier.
    ReleaseSignerId => Release
);
typed_id!(
    /// Consent grant identifier.
    ConsentId => Consent
);
typed_id!(
    /// Reference to player-approved continuity content.
    ContinuityRef => Continuity
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_and_round_trips() {
        let id: SeedId = "seed:character:a1b2-c3".parse().unwrap();
        assert_eq!(id.namespace(), IdNamespace::Character);
        assert_eq!(id.opaque(), "a1b2-c3");
        assert_eq!(id.to_string(), "seed:character:a1b2-c3");
    }

    #[test]
    fn every_namespace_round_trips() {
        for ns in IdNamespace::ALL {
            let text = format!("seed:{ns}:x1");
            let id: SeedId = text.parse().unwrap();
            assert_eq!(id.namespace(), ns);
            assert_eq!(id.to_string(), text);
        }
    }

    #[test]
    fn rejects_malformed_shapes() {
        for bad in [
            "",
            "seed",
            "seed:character",
            "other:character:abc",
            "character:abc",
        ] {
            assert_eq!(bad.parse::<SeedId>(), Err(IdError::Malformed), "{bad:?}");
        }
    }

    #[test]
    fn rejects_unknown_namespace() {
        assert_eq!(
            "seed:wallet:abc".parse::<SeedId>(),
            Err(IdError::UnknownNamespace)
        );
    }

    #[test]
    fn rejects_bad_opaque_values() {
        assert_eq!("seed:world:".parse::<SeedId>(), Err(IdError::InvalidLength));
        let too_long = format!("seed:world:{}", "a".repeat(MAX_OPAQUE_LEN + 1));
        assert_eq!(too_long.parse::<SeedId>(), Err(IdError::InvalidLength));
        let max = format!("seed:world:{}", "a".repeat(MAX_OPAQUE_LEN));
        assert!(max.parse::<SeedId>().is_ok());
        for bad in [
            "seed:world:ABC",
            "seed:world:a_b",
            "seed:world:-ab",
            "seed:world:ab-",
            "seed:world:a:b",
            "seed:world:a b",
            "seed:world:é",
        ] {
            assert_eq!(
                bad.parse::<SeedId>(),
                Err(IdError::InvalidCharacters),
                "{bad:?}"
            );
        }
    }

    #[test]
    fn identity_domains_are_not_interchangeable() {
        let err = "seed:world:origin".parse::<CharacterId>().unwrap_err();
        assert_eq!(
            err,
            IdError::NamespaceMismatch {
                expected: IdNamespace::Character,
                found: IdNamespace::World
            }
        );
        assert!("seed:identity:p1".parse::<CharacterId>().is_err());
        assert!("seed:character:c1".parse::<PlayerIdentityRef>().is_err());
        assert!("seed:node:n1".parse::<ValidatorId>().is_err());
        assert!("seed:validator:v1".parse::<ReleaseSignerId>().is_err());
    }

    #[test]
    fn typed_ids_round_trip_through_serde() {
        let id = CharacterId::new("traveler-01").unwrap();
        let json = serde_json::to_string(&id).unwrap();
        assert_eq!(json, "\"seed:character:traveler-01\"");
        let back: CharacterId = serde_json::from_str(&json).unwrap();
        assert_eq!(back, id);
        assert!(serde_json::from_str::<CharacterId>("\"seed:world:traveler-01\"").is_err());
    }

    #[test]
    fn errors_do_not_echo_the_rejected_value() {
        let secret = "seed:character:Alice.Smith@example.com";
        let err = secret.parse::<SeedId>().unwrap_err();
        let message = err.to_string();
        assert!(!message.contains("Alice"));
        assert!(!message.contains("example.com"));
    }
}
