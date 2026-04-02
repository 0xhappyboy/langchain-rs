use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum LangChainError {
    LLMError(String),
    PromptError(String),
    ParseError(String),
    ChainError(String),
    IoError(std::io::Error),
    JsonError(serde_json::Error),
}

impl fmt::Display for LangChainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LangChainError::LLMError(msg) => write!(f, "LLM error: {}", msg),
            LangChainError::PromptError(msg) => write!(f, "Prompt error: {}", msg),
            LangChainError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            LangChainError::ChainError(msg) => write!(f, "Chain error: {}", msg),
            LangChainError::IoError(err) => write!(f, "IO error: {}", err),
            LangChainError::JsonError(err) => write!(f, "JSON error: {}", err),
        }
    }
}

impl Error for LangChainError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            LangChainError::IoError(err) => Some(err),
            LangChainError::JsonError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for LangChainError {
    fn from(err: std::io::Error) -> Self {
        LangChainError::IoError(err)
    }
}

impl From<serde_json::Error> for LangChainError {
    fn from(err: serde_json::Error) -> Self {
        LangChainError::JsonError(err)
    }
}

impl From<String> for LangChainError {
    fn from(msg: String) -> Self {
        LangChainError::LLMError(msg)
    }
}

impl From<&str> for LangChainError {
    fn from(msg: &str) -> Self {
        LangChainError::LLMError(msg.to_string())
    }
}

pub type Result<T> = std::result::Result<T, LangChainError>;
