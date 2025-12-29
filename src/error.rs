use thiserror::Error;

#[derive(Error, Debug)]
pub enum GiError {
    #[error("Not a git repository (searched {depth} levels up)")]
    NotARepository { depth: usize },

    #[error("Git operation failed: {0}")]
    Git(#[from] git2::Error),

    #[error("UI interaction cancelled")]
    Cancelled,

    #[error("No items available: {context}")]
    Empty { context: &'static str },

    #[error("{0}")]
    Other(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, GiError>;

impl From<inquire::InquireError> for GiError {
    fn from(err: inquire::InquireError) -> Self {
        match err {
            inquire::InquireError::OperationCanceled
            | inquire::InquireError::OperationInterrupted => Self::Cancelled,
            _ => Self::Other(err.into()),
        }
    }
}
