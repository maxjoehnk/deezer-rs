//! [Radio API](https://developers.deezer.com/api/radio)
#![warn(missing_docs)]
use crate::models::{DeezerEnumerable, DeezerObject};
use serde::{Deserialize, Serialize};

/// Contains all the information provided for a Radio.
///
/// # Examples
///
/// You can query a radio by id via the [`DeezerObject::get()`] method:
///
/// ```rust
/// # use deezer::models::*;
/// # use deezer::DeezerError;
/// # #[tokio::main]
/// # async fn main() -> Result<(), DeezerError> {
/// let radio = Radio::get(6).await?.unwrap();
/// # assert_eq!(radio.id, 6);
/// # Ok(())
/// # }
/// ```
///
/// Or you can use [`DeezerClient::radio()`](crate::DeezerClient::radio()):
///
/// ```rust
/// # use deezer::models::*;
/// # use deezer::{DeezerClient, DeezerError};
/// # #[tokio::main]
/// # async fn main() -> Result<(), DeezerError> {
/// let deezer = DeezerClient::new();
/// let radio = deezer.radio(6).await?.unwrap();
/// # assert_eq!(radio.id, 6);
/// # Ok(())
/// # }
///
/// ```
///
/// To fetch all radios use [`DeezerEnumerable::get_all()`]:
///
/// ```rust
/// # use deezer::models::*;
/// # use deezer::DeezerError;
/// # #[tokio::main]
/// # async fn main() -> Result<(), DeezerError> {
/// let radios = Radio::get_all().await?;
/// # assert!(radios.len() > 0);
/// # Ok(())
/// # }
/// ```
///
/// Or [`DeezerClient::radios()`](crate::DeezerClient::radios()):
///
/// ```rust
/// # use deezer::models::*;
/// # use deezer::{DeezerClient, DeezerError};
/// # #[tokio::main]
/// # async fn main() -> Result<(), DeezerError> {
/// let deezer = DeezerClient::new();
/// let radios = deezer.radios().await?;
/// # assert!(radios.len() > 0);
/// # Ok(())
/// # }
/// ```
#[derive(Deserialize, Serialize, Debug)]
pub struct Radio {
    /// The radio deezer ID
    pub id: u64,

    /// The radio title
    pub title: String,

    /// The radio description
    ///
    /// None when listing all radios
    pub description: Option<String>,

    /// The share link of the radio on Deezer
    ///
    /// None when listing all radios
    #[serde(rename = "share")]
    pub share_link: Option<String>,

    /// The url of the radio picture
    pub picture: String,

    /// The url of the radio picture in size small
    pub picture_small: String,

    /// The url of the radio picture in size medium
    pub picture_medium: String,

    /// The url of the radio picture in size big
    pub picture_big: String,

    /// The url of the radio picture in size xl
    pub picture_xl: String,

    /// API Link to the tracklist of this radio
    #[serde(rename = "tracklist")]
    pub track_list: String,
}

impl DeezerObject for Radio {
    fn get_api_url(id: u64) -> String {
        format!("radio/{}", id)
    }
}

impl DeezerEnumerable for Radio {
    fn get_all_api_url() -> String {
        "radio".into()
    }
}
