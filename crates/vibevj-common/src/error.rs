use thiserror::Error;

/// Common error types for VibeVJ
#[derive(Error, Debug)]
pub enum VibeVJError {
    #[error("Render error: {0}")]
    RenderError(String),

    #[error("Audio error: {0}")]
    AudioError(String),

    #[error("Scene error: {0}")]
    SceneError(String),

    #[error("Scripting error: {0}")]
    ScriptingError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Resource not found: {0}")]
    ResourceNotFound(String),

    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
}

pub type Result<T> = std::result::Result<T, VibeVJError>;
