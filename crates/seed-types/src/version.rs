//! Protocol versions and versioned schema identifiers.
//!
//! # Rules (proposed, Phase 0)
//!
//! - A protocol version is `MAJOR.MINOR`, both non-negative integers without
//!   leading zeros.
//! - Two parties negotiate the **highest version both explicitly support**. There
//!   is no implicit "compatible enough" fallback: if they share no version,
//!   negotiation fails and the caller must report
//!   [`ErrorCode::UnsupportedProtocolVersion`](crate::ErrorCode::UnsupportedProtocolVersion).
//! - A change of `MAJOR` means breaking change. [`ProtocolVersion::same_major`] is
//!   informational only and never substitutes for negotiation.
//! - Documents name their schema as `seed.<name>/<MAJOR.MINOR>`, for example
//!   `seed.character-continuity-manifest/1.0`.

use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

/// Why a version or schema identifier was rejected.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum VersionError {
    /// Not of the form `MAJOR.MINOR`.
    #[error("version must have the form MAJOR.MINOR")]
    MalformedVersion,
    /// Not of the form `seed.<name>/<MAJOR.MINOR>`.
    #[error("schema identifier must have the form seed.<name>/<MAJOR.MINOR>")]
    MalformedSchemaId,
}

/// A `MAJOR.MINOR` protocol or schema version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ProtocolVersion {
    /// Breaking-change counter.
    pub major: u16,
    /// Additive-change counter.
    pub minor: u16,
}

impl ProtocolVersion {
    /// The protocol version implemented by this code base.
    pub const CURRENT: ProtocolVersion = ProtocolVersion::new(1, 0);

    /// Creates a version.
    #[must_use]
    pub const fn new(major: u16, minor: u16) -> Self {
        Self { major, minor }
    }

    /// Whether both versions share the same `MAJOR`. Informational only.
    #[must_use]
    pub fn same_major(self, other: ProtocolVersion) -> bool {
        self.major == other.major
    }

    /// Picks the highest version present in both lists, or `None` if there is none.
    #[must_use]
    pub fn negotiate(local: &[ProtocolVersion], remote: &[ProtocolVersion]) -> Option<Self> {
        local.iter().filter(|v| remote.contains(v)).max().copied()
    }
}

fn parse_component(s: &str) -> Option<u16> {
    let leading_zero = s.len() > 1 && s.starts_with('0');
    if s.is_empty() || leading_zero || !s.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    s.parse().ok()
}

impl FromStr for ProtocolVersion {
    type Err = VersionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (major, minor) = s.split_once('.').ok_or(VersionError::MalformedVersion)?;
        match (parse_component(major), parse_component(minor)) {
            (Some(major), Some(minor)) => Ok(Self::new(major, minor)),
            _ => Err(VersionError::MalformedVersion),
        }
    }
}

impl fmt::Display for ProtocolVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}

impl Serialize for ProtocolVersion {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

impl<'de> Deserialize<'de> for ProtocolVersion {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

/// A versioned schema identifier such as `seed.character-continuity-manifest/1.0`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SchemaId {
    name: String,
    version: ProtocolVersion,
}

impl SchemaId {
    /// Schema name without the `seed.` prefix, e.g. `character-continuity-manifest`.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Schema version.
    #[must_use]
    pub fn version(&self) -> ProtocolVersion {
        self.version
    }
}

impl FromStr for SchemaId {
    type Err = VersionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rest = s
            .strip_prefix("seed.")
            .ok_or(VersionError::MalformedSchemaId)?;
        let (name, version) = rest
            .split_once('/')
            .ok_or(VersionError::MalformedSchemaId)?;
        let name_ok = !name.is_empty()
            && name
                .bytes()
                .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
            && !name.starts_with('-')
            && !name.ends_with('-');
        if !name_ok {
            return Err(VersionError::MalformedSchemaId);
        }
        let version = version
            .parse()
            .map_err(|_| VersionError::MalformedSchemaId)?;
        Ok(Self {
            name: name.to_owned(),
            version,
        })
    }
}

impl fmt::Display for SchemaId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "seed.{}/{}", self.name, self.version)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const V1_0: ProtocolVersion = ProtocolVersion::new(1, 0);
    const V1_1: ProtocolVersion = ProtocolVersion::new(1, 1);
    const V2_0: ProtocolVersion = ProtocolVersion::new(2, 0);

    #[test]
    fn parses_and_displays() {
        assert_eq!("1.0".parse::<ProtocolVersion>().unwrap(), V1_0);
        assert_eq!(
            "12.34".parse::<ProtocolVersion>().unwrap().to_string(),
            "12.34"
        );
    }

    #[test]
    fn rejects_malformed_versions() {
        for bad in [
            "", "1", "1.", ".1", "1.0.0", "01.0", "1.01", "-1.0", "a.b", "1.0 ", "99999.0",
        ] {
            assert!(bad.parse::<ProtocolVersion>().is_err(), "{bad:?}");
        }
    }

    #[test]
    fn orders_numerically_not_lexically() {
        assert!(ProtocolVersion::new(1, 10) > ProtocolVersion::new(1, 9));
        assert!(V2_0 > V1_1);
    }

    #[test]
    fn negotiates_highest_common_version() {
        assert_eq!(
            ProtocolVersion::negotiate(&[V1_0, V1_1], &[V1_0, V1_1, V2_0]),
            Some(V1_1)
        );
        assert_eq!(
            ProtocolVersion::negotiate(&[V1_0], &[V1_0, V1_1]),
            Some(V1_0)
        );
    }

    #[test]
    fn negotiation_has_no_implicit_fallback() {
        // Same major but no explicitly shared version: no agreement.
        assert_eq!(ProtocolVersion::negotiate(&[V1_0], &[V1_1]), None);
        assert_eq!(ProtocolVersion::negotiate(&[V1_0], &[V2_0]), None);
        assert_eq!(ProtocolVersion::negotiate(&[], &[V1_0]), None);
        assert!(V1_0.same_major(V1_1));
        assert!(!V1_1.same_major(V2_0));
    }

    #[test]
    fn schema_ids_round_trip() {
        let id: SchemaId = "seed.character-continuity-manifest/1.0".parse().unwrap();
        assert_eq!(id.name(), "character-continuity-manifest");
        assert_eq!(id.version(), V1_0);
        assert_eq!(id.to_string(), "seed.character-continuity-manifest/1.0");
    }

    #[test]
    fn rejects_malformed_schema_ids() {
        for bad in [
            "character-passport/1.0",
            "seed./1.0",
            "seed.Passport/1.0",
            "seed.passport",
            "seed.passport/1",
            "seed.-passport/1.0",
            "seed.passport/1.0/x",
        ] {
            assert!(bad.parse::<SchemaId>().is_err(), "{bad:?}");
        }
    }
}
