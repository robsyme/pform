use serde::{Deserialize, Serialize};
use std::fmt;

/// Strongly typed organization ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OrgId(pub i64);

impl fmt::Display for OrgId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<i64> for OrgId {
    fn from(id: i64) -> Self {
        Self(id)
    }
}

/// Strongly typed workspace ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct WorkspaceId(pub i64);

impl fmt::Display for WorkspaceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<i64> for WorkspaceId {
    fn from(id: i64) -> Self {
        Self(id)
    }
}

/// Role within an organization
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OrgRole {
    Owner,
    Member,
    Collaborator,
}

/// Role within a workspace
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum WspRole {
    Owner,
    Admin,
    Maintain,
    Launch,
    Connect,
    View,
}

/// Type of participant in a workspace
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum ParticipantType {
    Member,
    Team,
    Collaborator,
}

/// Validation errors for field constraints
#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("Field '{field}' exceeds maximum length of {max} characters")]
    MaxLength { field: &'static str, max: usize },
    #[error("Field '{field}' does not match required pattern")]
    Pattern { field: &'static str },
    #[error("Field '{field}' is required")]
    Required { field: &'static str },
}

/// Validation trait for request types
pub trait Validate {
    fn validate(&self) -> Result<(), ValidationError>;
}

/// Helper functions for validation
pub mod validation {
    use super::ValidationError;

    pub fn validate_max_length(
        value: &str,
        max: usize,
        field: &'static str,
    ) -> Result<(), ValidationError> {
        if value.len() > max {
            Err(ValidationError::MaxLength { field, max })
        } else {
            Ok(())
        }
    }

    pub fn validate_pattern(
        value: &str,
        pattern: &regex::Regex,
        field: &'static str,
    ) -> Result<(), ValidationError> {
        if !pattern.is_match(value) {
            Err(ValidationError::Pattern { field })
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;

    #[test]
    fn test_max_length_validation() {
        assert!(validation::validate_max_length("test", 5, "field").is_ok());
        assert!(validation::validate_max_length("test", 4, "field").is_ok());
        assert!(validation::validate_max_length("test", 3, "field").is_err());
    }

    #[test]
    fn test_pattern_validation() {
        let pattern = Regex::new(r"^[a-z]+$").unwrap();
        assert!(validation::validate_pattern("test", &pattern, "field").is_ok());
        assert!(validation::validate_pattern("Test", &pattern, "field").is_err());
        assert!(validation::validate_pattern("test123", &pattern, "field").is_err());
    }

    #[test]
    fn test_validation_error_display() {
        let err = ValidationError::MaxLength {
            field: "test",
            max: 5,
        };
        assert!(err.to_string().contains("test"));
        assert!(err.to_string().contains("5"));

        let err = ValidationError::Pattern { field: "test" };
        assert!(err.to_string().contains("test"));
        assert!(err.to_string().contains("pattern"));
    }
} 