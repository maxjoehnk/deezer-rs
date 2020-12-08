//! [Editorial API](https://developers.deezer.com/api/editorial)
#![warn(missing_docs)]
use crate::models::{DeezerEnumerable, DeezerObject};
use serde::{Deserialize, Serialize};

/// Contains all the information provided for an Editorial.
///
/// # Examples
///
/// You can query an editorial by id via the [`DeezerObject::get()`] method:
///
/// ```rust
/// # use deezer::models::*;
/// # use deezer::DeezerError;
/// # #[tokio::main]
/// # async fn main() -> Result<(), DeezerError> {
/// let editorial = Editorial::get(0).await?.unwrap();
/// # assert_eq!(editorial.id, 0);
/// # Ok(())
/// # }
/// ```
///
/// Or you can use [`DeezerClient::editorial()`](crate::DeezerClient::editorial()):
///
/// ```rust
/// # use deezer::models::*;
/// # use deezer::{DeezerClient, DeezerError};
/// # #[tokio::main]
/// # async fn main() -> Result<(), DeezerError> {
/// let deezer = DeezerClient::new();
/// let editorial = deezer.editorial(0).await?.unwrap();
/// # assert_eq!(editorial.id, 0);
/// # Ok(())
/// # }
///
/// ```
///
/// To fetch all editorials use [`DeezerEnumerable::get_all()`]:
///
/// ```rust
/// # use deezer::models::*;
/// # use deezer::DeezerError;
/// # #[tokio::main]
/// # async fn main() -> Result<(), DeezerError> {
/// let editorials = Editorial::get_all().await?;
/// # assert!(editorials.len() > 0);
/// # Ok(())
/// # }
/// ```
///
/// Or [`DeezerClient::editorials()`](crate::DeezerClient::editorials()):
///
/// ```rust
/// # use deezer::models::*;
/// # use deezer::{DeezerClient, DeezerError};
/// # #[tokio::main]
/// # async fn main() -> Result<(), DeezerError> {
/// let deezer = DeezerClient::new();
/// let editorials = deezer.editorials().await?;
/// # assert!(editorials.len() > 0);
/// # Ok(())
/// # }
/// ```
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Editorial {
    /// The editorial's Deezer id
    pub id: u64,

    /// The editorial's name
    pub name: String,

    /// The url of the editorial picture
    pub picture: String,

    /// The url of the editorial picture in size small
    pub picture_small: String,

    /// The url of the editorial picture in size medium
    pub picture_medium: String,

    /// The url of the editorial picture in size big
    pub picture_big: String,

    /// The url of the editorial picture in size xl
    pub picture_xl: String,
}

impl DeezerObject for Editorial {
    fn get_api_url(id: u64) -> String {
        format!("editorial/{}", id)
    }
}

impl DeezerEnumerable for Editorial {
    fn get_all_api_url() -> String {
        "editorial".into()
    }
}
