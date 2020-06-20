//! [Artist API](https://developers.deezer.com/api/artist)
#[warn(missing_docs)]
use crate::models::DeezerObject;
use crate::Result;
use serde::{Deserialize, Serialize};

/// Contains all the information provided for an Artist.
///
/// # Examples
///
/// You can query an artist by id via the [`DeezerObject::get()`] method:
///
/// ```rust
/// # use deezer::models::*;
/// # use deezer::DeezerError;
/// #
/// # #[tokio::main]
/// # async fn main() -> Result<(), DeezerError> {
/// let artist = Artist::get(27).await?.unwrap();
/// # assert_eq!(artist.id, 27);
/// # println!("{:?}", artist);
/// # Ok(())
/// # }
/// ```
///
/// Or you can use [`DeezerClient::artist()`](crate::DeezerClient::artist):
///
/// ```rust
/// # use deezer::models::*;
/// # use deezer::{DeezerClient, DeezerError};
/// #
/// # #[tokio::main]
/// # async fn main() -> Result<(), DeezerError> {
/// let deezer = DeezerClient::new();
///
/// let artist = deezer.artist(27).await?.unwrap();
/// # assert_eq!(artist.id, 27);
/// # Ok(())
/// # }
///
/// ```
#[derive(Deserialize, Serialize, Debug)]
pub struct Artist {
    /// `The artist's Deezer id`
    pub id: u64,

    /// `The artist's name`
    pub name: String,

    /// `The url of the artist on Deezer`
    pub link: String,

    /// `The share link of the artist on Deezer`
    #[serde(rename = "share")]
    pub share_link: String,

    /// `The url of the artist picture.`
    pub picture: String,

    /// `The url of the artist picture in size small`
    pub picture_small: String,

    /// `The url of the artist picture in size medium`
    pub picture_medium: String,

    /// `The url of the artist picture in size big`
    pub picture_big: String,

    /// `The url of the artist picture in size xl`
    pub picture_xl: String,

    /// `The number of artist's albums`
    pub nb_album: u64,

    /// `The number of artist's fans`
    pub nb_fan: u64,

    /// `True if the artist has a smartradio`
    #[serde(rename = "radio")]
    pub has_radio: bool,

    /// `API Link to the top of this artist`
    pub tracklist: String,
}

impl DeezerObject for Artist {
    fn get_api_url(id: u64) -> String {
        format!("artist/{}", id)
    }
}

/// Subset of [`Artist`].
///
/// Use [`get_full()`] for the corresponding [`Artist`] struct.
///
/// [`get_full()`]: ContributorArtist::get_full
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ContributorArtist {
    /// `The artist's Deezer id`
    pub id: u64,

    /// `The artist's name`
    pub name: String,

    /// `The url of the artist on Deezer`
    pub link: String,

    /// `The share link of the artist on Deezer`
    #[serde(rename = "share")]
    pub share_link: String,

    /// `The url of the artist picture in size small.`
    pub picture_small: String,

    /// `The url of the artist picture in size medium.`
    pub picture_medium: String,

    /// `The url of the artist picture in size big.`
    pub picture_big: String,

    /// `The url of the artist picture in size xl.`
    pub picture_xl: String,

    /// `True if the artist has a smartradio`
    #[serde(rename = "radio")]
    pub has_radio: bool,

    /// `API Link to the top of this artist`
    pub tracklist: String,
}

impl ContributorArtist {
    /// Returns the corresponding [`Artist`].
    pub async fn get_full(&self) -> Result<Artist> {
        // Safety: unwrap should be okay here, as the artist is referenced by the deezer api
        let artist = Artist::get(self.id).await?.unwrap();
        Ok(artist)
    }
}
