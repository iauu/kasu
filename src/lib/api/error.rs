use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed serde serialization")]
    SerdeJsonSerializationError(#[from] serde_json::Error),
    #[error("Request error")]
    RequestError(#[from] reqwest::Error),
    #[error("Response ok attribute is false")]
    RespNotOk,
    #[error("File access error")]
    FileAccessError(#[from] std::io::Error)
}