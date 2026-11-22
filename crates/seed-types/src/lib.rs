//! Stable domain types for THE SEED.
//!
//! This crate has no network, storage, or cryptographic side effects. It defines:
//!
//! - [`id`]: namespaced opaque identifiers, typed per identity domain so that an
//!   identifier from one domain (for example a world) can never be accepted where
//!   another domain (for example a character) is expected.
//! - [`version`]: protocol versions and versioned schema identifiers.
//! - [`error`]: the common, stable error model used by APIs.
//!
//! Status: early foundation work. These types are contracts under review, not a
//! finished protocol.

pub mod error;
pub mod id;
pub mod version;

pub use error::{ApiError, ErrorCode};
pub use id::{
    CharacterId, ConsentId, ContinuityRef, HubId, IdError, IdNamespace, NodeId, PlayerIdentityRef,
    ReleaseSignerId, SeedId, ValidatorId, WorldId,
};
pub use version::{ProtocolVersion, SchemaId, VersionError};
