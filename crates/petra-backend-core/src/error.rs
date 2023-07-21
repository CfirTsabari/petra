use thiserror::Error;

pub type Result<T> = std::result::Result<T, PetraBackendError>;

// Personal choice
#[allow(clippy::module_name_repetitions)]
#[derive(Error, Debug)]
pub enum PetraBackendError {
    #[error("failed writing output")]
    OutputError(#[from] std::io::Error),
    #[error("backend is not supported:{0}")]
    BackendNotSupported(String),
}
