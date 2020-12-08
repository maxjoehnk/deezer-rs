//! [Genre API](https://developers.deezer.com/api/genre)
#![warn(missing_docs)]
use serde::{Deserialize, Serialize};

use crate::models::{DeezerEnumerable, DeezerObject};

/// Contains all the information provided for a Genre.
///
/// # Examples
///
/// You can query a genre by id via the [`DeezerObject::get()`] method:
///
/// ```rust
/// # use deezer::models::*;
/// # use deezer::DeezerError;
/// # #[tokio::main]
/// # async fn main() -> Result<(), DeezerError> {
/// let genre = Genre::get(0).await?.unwrap();
/// # assert_eq!(genre.id, 0);
/// # Ok(())
/// # }
/// ```
///
/// Or you can use [`DeezerClient::genre()`](crate::DeezerClient::genre()):
///
/// ```rust
/// # use deezer::models::*;
/// # use deezer::{DeezerClient, DeezerError};
/// # #[tokio::main]
/// # async fn main() -> Result<(), DeezerError> {
/// let deezer = DeezerClient::new();
/// let genre = deezer.genre(0).await?.unwrap();
/// # assert_eq!(genre.id, 0);
/// # Ok(())
/// # }
///
/// ```
///
/// To fetch all genres use [`DeezerEnumerable::get_all()`]:
///
/// ```rust
/// # use deezer::models::*;
/// # use deezer::DeezerError;
/// # #[tokio::main]
/// # async fn main() -> Result<(), DeezerError> {
/// let genres = Genre::get_all().await?;
/// # assert!(genres.len() > 0);
/// # Ok(())
/// # }
/// ```
///
/// Or [`DeezerClient::genres()`](crate::DeezerClient::genres()):
///
/// ```rust
/// # use deezer::models::*;
/// # use deezer::{DeezerClient, DeezerError};
/// # #[tokio::main]
/// # async fn main() -> Result<(), DeezerError> {
/// let deezer = DeezerClient::new();
/// let genres = deezer.genres().await?;
/// # assert!(genres.len() > 0);
/// # Ok(())
/// # }
/// ```
#[derive(Deserialize, Serialize, Debug)]
pub struct Genre {
    /// The editorial's Deezer id
    pub id: u64,

    /// The editorial's name
    pub name: String,

    /// The url of the genre picture.
    pub picture: String,

    /// The url of the genre picture in size small.
    pub picture_small: String,

    /// The url of the genre picture in size medium.
    pub picture_medium: String,

    /// The url of the genre picture in size big.
    pub picture_big: String,

    /// The url of the genre picture in size xl.
    pub picture_xl: String,
}

impl DeezerObject for Genre {
    fn get_api_url(id: u64) -> String {
        format!("genre/{}", id)
    }
}

impl DeezerEnumerable for Genre {
    fn get_all_api_url() -> String {
        "genre".into()
    }
}
