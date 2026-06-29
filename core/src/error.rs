pub type Acos7Result<T> = Result<T, Acos7Error>;

#[derive(Debug, thiserror::Error)]
pub enum Acos7Error {
    #[error("Encode error: {0}")]
    Encode(#[from] rmp_serde::encode::Error),
    #[error("Decode error: {0}")]
    Decode(#[from] rmp_serde::decode::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
