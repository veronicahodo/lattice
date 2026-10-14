use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformError {
    pub kind: TransformErrorKind,
    pub severity: TransformErrorSeverity,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransformErrorKind {
    Input,
    Output,
    Hardware,
    Runtime,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransformErrorSeverity {
    Fatal,
    Error,
    Warning,
    Info,
    Debug,
}
