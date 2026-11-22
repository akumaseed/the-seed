//! Protocol contracts for THE SEED.
//!
//! This crate embeds the versioned JSON Schemas from `protocol/json-schema/` and
//! validates documents against them. A document names its own schema in its
//! top-level `schema` field (for example `seed.character-continuity-manifest/1.0`);
//! only schemas registered here are accepted.
//!
//! Validation reports **where** a document is invalid (JSON Pointer and failing
//! keyword) but never **what** the offending value was. Continuity documents can
//! hold private player content, so error output must be safe to log.
//!
//! Status: Phase 0 contracts. Passing schema validation does not verify any
//! signature, consent grant, or authority; those checks belong to later phases.

use std::sync::OnceLock;

use jsonschema::{Draft, Validator};
use seed_types::{ApiError, ErrorCode, SchemaId};
use serde_json::Value;

/// Largest document accepted by [`validate_json`], in bytes.
pub const MAX_DOCUMENT_BYTES: usize = 1024 * 1024;

/// Maximum number of violations reported for one document.
pub const MAX_REPORTED_VIOLATIONS: usize = 100;

/// Schemas known to this implementation: `(schema id, schema source)`.
const SCHEMAS: &[(&str, &str)] = &[
    (
        "seed.character-continuity-manifest/1.0",
        include_str!("../../../protocol/json-schema/character-continuity-manifest-1.0.json"),
    ),
    (
        "seed.character-identity-anchor/1.0",
        include_str!("../../../protocol/json-schema/character-identity-anchor-1.0.json"),
    ),
];

struct Registered {
    id: SchemaId,
    validator: Validator,
}

fn registry() -> &'static [Registered] {
    static REGISTRY: OnceLock<Vec<Registered>> = OnceLock::new();
    REGISTRY.get_or_init(|| {
        SCHEMAS
            .iter()
            .map(|(id, source)| {
                // Embedded schemas are part of the build; failing here is a programming
                // error caught by this crate's tests, not a runtime input error.
                let schema: Value =
                    serde_json::from_str(source).expect("embedded schema is valid JSON");
                let validator = jsonschema::options()
                    .with_draft(Draft::Draft202012)
                    .should_validate_formats(true)
                    .build(&schema)
                    .expect("embedded schema compiles");
                Registered {
                    id: id.parse().expect("embedded schema id is well-formed"),
                    validator,
                }
            })
            .collect()
    })
}

/// Identifiers of every schema this implementation accepts.
#[must_use]
pub fn known_schemas() -> Vec<SchemaId> {
    registry().iter().map(|r| r.id.clone()).collect()
}

/// One schema violation: where it happened and which rule failed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Violation {
    /// JSON Pointer to the invalid location in the document.
    pub instance_path: String,
    /// The JSON Schema keyword that failed, such as `required` or `const`.
    pub keyword: String,
}

impl std::fmt::Display for Violation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let path = if self.instance_path.is_empty() {
            "/"
        } else {
            &self.instance_path
        };
        write!(f, "{path}: failed `{}`", self.keyword)
    }
}

/// Why a document was rejected. Messages never include document values.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DocumentError {
    /// The document exceeds [`MAX_DOCUMENT_BYTES`].
    #[error("document exceeds {MAX_DOCUMENT_BYTES} bytes")]
    TooLarge,
    /// The input is not valid JSON.
    #[error("document is not valid JSON")]
    NotJson,
    /// There is no top-level string field `schema`.
    #[error("document has no top-level `schema` field")]
    MissingSchema,
    /// The `schema` field is not of the form `seed.<name>/<MAJOR.MINOR>`.
    #[error("document `schema` field is malformed")]
    MalformedSchemaId,
    /// The schema is well-formed but not supported by this implementation.
    #[error("schema `{0}` is not supported")]
    UnknownSchema(SchemaId),
    /// The document does not satisfy its schema.
    #[error("document does not match schema `{schema}` ({} violation(s))", violations.len())]
    Invalid {
        /// The schema the document declared.
        schema: SchemaId,
        /// Up to [`MAX_REPORTED_VIOLATIONS`] violations.
        violations: Vec<Violation>,
    },
}

impl DocumentError {
    /// Converts the error to the public API error model.
    #[must_use]
    pub fn to_api_error(&self) -> ApiError {
        let code = match self {
            DocumentError::TooLarge
            | DocumentError::NotJson
            | DocumentError::MissingSchema
            | DocumentError::MalformedSchemaId => ErrorCode::MalformedDocument,
            DocumentError::UnknownSchema(_) => ErrorCode::UnknownSchema,
            DocumentError::Invalid { .. } => ErrorCode::SchemaValidationFailed,
        };
        let err = ApiError::new(code, self.to_string());
        match self {
            DocumentError::Invalid { violations, .. } => {
                let list: Vec<Value> = violations
                    .iter()
                    .map(|v| serde_json::json!({ "path": v.instance_path, "keyword": v.keyword }))
                    .collect();
                err.with_detail("violations", list)
            }
            _ => err,
        }
    }
}

/// Validates a parsed document against the schema it declares.
///
/// # Errors
/// Returns [`DocumentError`] if the schema is missing, malformed, unknown, or the
/// document does not satisfy it.
pub fn validate_document(document: &Value) -> Result<SchemaId, DocumentError> {
    let declared = document
        .get("schema")
        .and_then(Value::as_str)
        .ok_or(DocumentError::MissingSchema)?;
    let id: SchemaId = declared
        .parse()
        .map_err(|_| DocumentError::MalformedSchemaId)?;
    let registered = registry()
        .iter()
        .find(|r| r.id == id)
        .ok_or_else(|| DocumentError::UnknownSchema(id.clone()))?;

    let violations: Vec<Violation> = registered
        .validator
        .iter_errors(document)
        .take(MAX_REPORTED_VIOLATIONS)
        .map(|e| Violation {
            instance_path: e.instance_path().as_str().to_owned(),
            keyword: e.kind().keyword().to_owned(),
        })
        .collect();

    if violations.is_empty() {
        Ok(id)
    } else {
        Err(DocumentError::Invalid {
            schema: id,
            violations,
        })
    }
}

/// Parses and validates a JSON document, enforcing [`MAX_DOCUMENT_BYTES`].
///
/// # Errors
/// Returns [`DocumentError`] if the input is too large, not JSON, or invalid.
pub fn validate_json(input: &str) -> Result<SchemaId, DocumentError> {
    if input.len() > MAX_DOCUMENT_BYTES {
        return Err(DocumentError::TooLarge);
    }
    let document: Value = serde_json::from_str(input).map_err(|_| DocumentError::NotJson)?;
    validate_document(&document)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_embedded_schema_compiles() {
        assert_eq!(known_schemas().len(), SCHEMAS.len());
    }

    #[test]
    fn rejects_documents_without_a_known_schema() {
        assert_eq!(validate_json("{}"), Err(DocumentError::MissingSchema));
        assert_eq!(
            validate_json(r#"{"schema": "passport"}"#),
            Err(DocumentError::MalformedSchemaId)
        );
        assert!(matches!(
            validate_json(r#"{"schema": "seed.unknown-thing/1.0"}"#),
            Err(DocumentError::UnknownSchema(_))
        ));
        assert_eq!(validate_json("not json"), Err(DocumentError::NotJson));
    }

    #[test]
    fn rejects_oversized_documents_before_parsing() {
        let big = " ".repeat(MAX_DOCUMENT_BYTES + 1);
        assert_eq!(validate_json(&big), Err(DocumentError::TooLarge));
    }

    #[test]
    fn maps_to_stable_api_error_codes() {
        assert_eq!(
            DocumentError::NotJson.to_api_error().code,
            ErrorCode::MalformedDocument
        );
        let err = validate_json(r#"{"schema": "seed.character-identity-anchor/1.0"}"#)
            .unwrap_err()
            .to_api_error();
        assert_eq!(err.code, ErrorCode::SchemaValidationFailed);
        assert!(err.details.contains_key("violations"));
    }
}
