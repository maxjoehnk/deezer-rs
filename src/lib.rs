//! Provides a rust implementation of the Deezer Api.
//!
//! To interact with the api create a new instance of [`DeezerClient`] which provides all available
//! apis and also handles authentication.
//!
//! Additionally each Deezer Object which is queryable by id implements the
//! [`DeezerObject`](crate::models::DeezerObject) trait which allows direct fetching of the given
//! object via [`DeezerObject::get()`](crate::models::DeezerObject::get).

mod client;
mod error;
pub mod models;
mod search;
mod clients;

pub use self::client::DeezerClient;
pub use self::error::DeezerError;
pub use self::search::*;
pub use self::clients::*;

pub(crate) type Result<T> = std::result::Result<T, DeezerError>;
