#![warn(missing_docs)]
use thiserror::Error;

/// Every api which can fail will return a [`DeezerError`].
#[derive(Debug, Error)]
pub enum DeezerError {
    #[error(transparent)]
    HttpError(#[from] reqwest::Error),
}
