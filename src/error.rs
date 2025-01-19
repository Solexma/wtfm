use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum WtfmError {
    // Input Validation
    EmptyProjectName,
    InvalidProjectName(String),
    InvalidEmail(String),
    InvalidVersion(String),
    EmptyInput(String),

    // File System
    ConfigNotFound(String),
    ConfigWriteError(String),
    PermissionDenied(String),

    // State
    SectionNotFound(String),
    RequiredSectionMissing(String),

    // Runtime
    SerializationError(String),
    DeserializationError(String),
    UnexpectedError(String),
    IoError(std::io::Error),
    SerdeError(serde_json::Error),
}

impl fmt::Display for WtfmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Input Validation
            WtfmError::EmptyProjectName => write!(f, "Project name cannot be empty"),
            WtfmError::InvalidProjectName(name) => write!(
                f,
                "Invalid project name '{}'. Use only letters, numbers, hyphens and underscores",
                name
            ),
            WtfmError::InvalidEmail(email) => write!(f, "Invalid email address: {}", email),
            WtfmError::InvalidVersion(version) => write!(
                f,
                "Invalid version format '{}'. Use semantic versioning (e.g., 1.0.0)",
                version
            ),
            WtfmError::EmptyInput(field) => write!(f, "Empty input for field: {}", field),

            // File System
            WtfmError::ConfigNotFound(path) => {
                write!(f, "Configuration file not found at: {}", path)
            }
            WtfmError::ConfigWriteError(path) => {
                write!(f, "Failed to write configuration to: {}", path)
            }
            WtfmError::PermissionDenied(path) => write!(f, "Permission denied accessing: {}", path),

            // State
            WtfmError::SectionNotFound(section) => write!(f, "Section not found: {}", section),
            WtfmError::RequiredSectionMissing(section) => {
                write!(f, "Required section not configured: {}", section)
            }

            // Runtime
            WtfmError::SerializationError(msg) => {
                write!(f, "Failed to serialize configuration: {}", msg)
            }
            WtfmError::DeserializationError(msg) => {
                write!(f, "Failed to deserialize configuration: {}", msg)
            }
            WtfmError::UnexpectedError(msg) => write!(f, "An unexpected error occurred: {}", msg),
            WtfmError::IoError(e) => write!(f, "IO error: {}", e),
            WtfmError::SerdeError(e) => write!(f, "Serialization error: {}", e),
        }
    }
}

impl Error for WtfmError {}

impl From<std::io::Error> for WtfmError {
    fn from(err: std::io::Error) -> WtfmError {
        WtfmError::IoError(err)
    }
}

impl From<serde_json::Error> for WtfmError {
    fn from(err: serde_json::Error) -> WtfmError {
        WtfmError::SerdeError(err)
    }
}
