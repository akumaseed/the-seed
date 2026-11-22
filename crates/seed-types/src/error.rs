//! Common error model.
//!
//! Wire shape:
//!
//! ```json
//! {
//!   "error": {
//!     "code": "SCHEMA_VALIDATION_FAILED",
//!     "message": "The document does not match its schema.",
//!     "request_id": "req_...",
//!     "retryable": false,
//!     "details": {}
//!   }
//! }
//! ```
//!
//! Codes are stable wire values: once published, a code is never renamed or reused
//! with a different meaning. Errors must not carry stack traces, secrets, internal
//! file paths, private prompts, personal data, or private character continuity.

use std::fmt;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

/// Stable error codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ErrorCode {
    /// A `seed:<namespace>:<opaque>` identifier was malformed or from the wrong domain.
    InvalidIdentifier,
    /// The two parties share no protocol version.
    UnsupportedProtocolVersion,
    /// The document declares a schema that is unknown to this implementation.
    UnknownSchema,
    /// The document is not well-formed (not JSON, missing `schema`, ...).
    MalformedDocument,
    /// The document does not satisfy its schema.
    SchemaValidationFailed,
    /// An unexpected internal failure. Details are never exposed.
    InternalError,
}

impl ErrorCode {
    /// Every code, used to test stability and uniqueness.
    pub const ALL: [ErrorCode; 6] = [
        ErrorCode::InvalidIdentifier,
        ErrorCode::UnsupportedProtocolVersion,
        ErrorCode::UnknownSchema,
        ErrorCode::MalformedDocument,
        ErrorCode::SchemaValidationFailed,
        ErrorCode::InternalError,
    ];

    /// The stable wire value of the code.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            ErrorCode::InvalidIdentifier => "INVALID_IDENTIFIER",
            ErrorCode::UnsupportedProtocolVersion => "UNSUPPORTED_PROTOCOL_VERSION",
            ErrorCode::UnknownSchema => "UNKNOWN_SCHEMA",
            ErrorCode::MalformedDocument => "MALFORMED_DOCUMENT",
            ErrorCode::SchemaValidationFailed => "SCHEMA_VALIDATION_FAILED",
            ErrorCode::InternalError => "INTERNAL_ERROR",
        }
    }

    /// Whether retrying the same request unchanged can succeed.
    #[must_use]
    pub const fn default_retryable(self) -> bool {
        matches!(self, ErrorCode::InternalError)
    }
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// The body of an API error.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiError {
    /// Stable machine-readable code.
    pub code: ErrorCode,
    /// Human-readable message, safe to show publicly.
    pub message: String,
    /// Request identifier for correlation, when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// Whether retrying the same request unchanged can succeed.
    pub retryable: bool,
    /// Structured, non-sensitive details.
    #[serde(default)]
    pub details: Map<String, Value>,
}

/// Wire envelope `{ "error": { ... } }`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ErrorEnvelope {
    /// The error body.
    pub error: ApiError,
}

impl ApiError {
    /// Creates an error with the code's default retry semantics and no details.
    #[must_use]
    pub fn new(code: ErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            request_id: None,
            retryable: code.default_retryable(),
            details: Map::new(),
        }
    }

    /// Attaches a request identifier.
    #[must_use]
    pub fn with_request_id(mut self, request_id: impl Into<String>) -> Self {
        self.request_id = Some(request_id.into());
        self
    }

    /// Adds a structured detail. Callers must not put sensitive data here.
    #[must_use]
    pub fn with_detail(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.details.insert(key.into(), value.into());
        self
    }

    /// Wraps the error in its wire envelope.
    #[must_use]
    pub fn into_envelope(self) -> ErrorEnvelope {
        ErrorEnvelope { error: self }
    }
}

impl From<crate::IdError> for ApiError {
    fn from(err: crate::IdError) -> Self {
        ApiError::new(ErrorCode::InvalidIdentifier, err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn codes_are_stable_wire_values() {
        let expected = [
            "INVALID_IDENTIFIER",
            "UNSUPPORTED_PROTOCOL_VERSION",
            "UNKNOWN_SCHEMA",
            "MALFORMED_DOCUMENT",
            "SCHEMA_VALIDATION_FAILED",
            "INTERNAL_ERROR",
        ];
        let actual: Vec<_> = ErrorCode::ALL.iter().map(|c| c.as_str()).collect();
        assert_eq!(actual, expected);
        for code in ErrorCode::ALL {
            let json = serde_json::to_value(code).unwrap();
            assert_eq!(json, Value::String(code.as_str().to_owned()));
            assert_eq!(serde_json::from_value::<ErrorCode>(json).unwrap(), code);
        }
    }

    #[test]
    fn codes_are_unique() {
        let unique: HashSet<_> = ErrorCode::ALL.iter().map(|c| c.as_str()).collect();
        assert_eq!(unique.len(), ErrorCode::ALL.len());
    }

    #[test]
    fn serializes_to_documented_envelope() {
        let err = ApiError::new(ErrorCode::SchemaValidationFailed, "Invalid document.")
            .with_request_id("req_123")
            .with_detail("violations", 2);
        let json = serde_json::to_value(err.into_envelope()).unwrap();
        assert_eq!(
            json,
            serde_json::json!({
                "error": {
                    "code": "SCHEMA_VALIDATION_FAILED",
                    "message": "Invalid document.",
                    "request_id": "req_123",
                    "retryable": false,
                    "details": { "violations": 2 }
                }
            })
        );
    }

    #[test]
    fn internal_errors_are_retryable_by_default() {
        assert!(ApiError::new(ErrorCode::InternalError, "x").retryable);
        assert!(!ApiError::new(ErrorCode::InvalidIdentifier, "x").retryable);
    }

    #[test]
    fn identifier_errors_map_to_invalid_identifier() {
        let err: ApiError = "seed:nope:x".parse::<crate::SeedId>().unwrap_err().into();
        assert_eq!(err.code, ErrorCode::InvalidIdentifier);
    }
}
