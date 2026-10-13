//! Tokenizer and detokenizer

use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    InvalidVocab(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(e) => write!(f, "I/O error: {}", e),
            Error::InvalidVocab(msg) => write!(f, "Invalid vocab: {}", msg),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub struct Tokenizer {
    // TODO: vocab, merges, etc.
}

impl Tokenizer {
    pub fn load(_vocab_path: &str) -> Result<Self> {
        // TODO: load tokenizer from file
        todo!("Tokenizer loading not implemented")
    }

    pub fn encode(&self, _text: &str) -> Result<Vec<u32>> {
        // TODO: text → token IDs
        todo!("Encoding not implemented")
    }

    pub fn decode(&self, _tokens: &[u32]) -> Result<String> {
        // TODO: token IDs → text
        todo!("Decoding not implemented")
    }
}
