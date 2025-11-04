use wabble_core::language::Language;

pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("No kaikki file found for language: {0:?}")]
    NoKaikkiFile(Language),
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
    #[error("Regex error: {0}")]
    Regex(#[from] regex::Error),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}
