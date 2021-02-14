//! [Artist API](https://developers.deezer.com/api/artist)
#![warn(missing_docs)]

use serde::{Deserialize, Serialize};

use crate::models::{Album, DeezerObject, DeezerConnection};
use crate::Result;

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
    fn get_by_id(id: u64) -> String {
        format!("artist/{}", id)
    }
}

impl DeezerConnection<ArtistAlbum> for Artist {
    fn get_connection_url(identifier: &str) -> String {
        format!("artist/{}/albums", identifier)
    }

    fn get_url(&self) -> String {
        format!("artist/{}/albums", self.id)
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

/// Artist's album - Subset of [`Album`].
///
/// [Reference](https://developers.deezer.com/api/artist/albums)
///
///
/// Use [`get_full()`] for the corresponding [`Album`] struct.
///
/// [`get_full()`]: ArtistAlbum::get_full
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ArtistAlbum {
    /// `The Deezer album id`
    pub id: u64,

    /// `The album title`
    pub title: String,

    /// `The url of the album on Deezer`
    pub link: String,

    /// `The url of the album's cover.`
    pub cover: String,

    /// `The url of the album's cover in size small.`
    pub cover_small: String,

    /// `The url of the album's cover in size medium.`
    pub cover_medium: String,

    /// `The url of the album's cover in size big.`
    pub cover_big: String,

    /// `The url of the album's cover in size xl.`
    pub cover_xl: String,

    /// `The album's first genre id (You should use the genre list instead).`
    pub genre_id: Option<i32>,

    /// `The number of album's Fans`
    pub fans: u32,
    /// `The album's release date`

    pub release_date: String,
    /// `The record type of the album (EP / ALBUM / etc..)`

    pub record_type: String,

    /// `Whether the album contains explicit lyrics`
    pub explicit_lyrics: bool,
}

impl DeezerObject for ArtistAlbum {
    fn get_by_id(id: u64) -> String {
        format!("artist/{}/albums", id)
    }
}

impl ArtistAlbum {
    /// Returns the corresponding [`Album`].
    pub async fn get_full(&self) -> Result<Album> {
        // Safety: unwrap should be okay here, as the artist is referenced by the deezer api
        let artist = Album::get(self.id).await?.unwrap();
        Ok(artist)
    }
}
