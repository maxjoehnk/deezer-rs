//! [Album API](https://developers.deezer.com/api/album)
#[warn(missing_docs)]
use serde::{Deserialize, Serialize};

use crate::models::{Artist, ContributorArtist, DeezerArray, DeezerObject, Genre, Track};
use crate::Result;

/// Contains all the information provided for an Album.
///
/// # Examples
///
/// You can query an album by id via the [`DeezerObject::get()`] method:
///
/// ```rust
/// # use deezer::models::*;
/// # use deezer::DeezerError;
/// #
/// # #[tokio::main]
/// # async fn main() -> Result<(), DeezerError> {
/// let album = Album::get(302127).await?.unwrap();
/// # assert_eq!(album.id, 302127);
/// # println!("{:?}", album);
/// # Ok(())
/// # }
/// ```
///
/// Or you can use [`DeezerClient::album()`](crate::DeezerClient::album()):
///
/// ```rust
/// # use deezer::models::*;
/// # use deezer::{DeezerClient, DeezerError};
/// #
/// # #[tokio::main]
/// # async fn main() -> Result<(), DeezerError> {
/// let deezer = DeezerClient::new();
///
/// let album = deezer.album(302127).await?.unwrap();
/// # assert_eq!(album.id, 302127);
/// # Ok(())
/// # }
///
/// ```
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Album {
    /// `The Deezer album id`
    pub id: u64,

    /// `The album title`
    pub title: String,

    /// `The album UPC`
    pub upc: String,

    /// `The url of the album on Deezer`
    pub link: String,

    /// `The share link of the album on Deezer`
    #[serde(rename = "share")]
    pub share_link: String,

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

    /// `List of genre object`
    pub genres: DeezerArray<AlbumGenre>,

    /// `The album's label name`
    pub label: String,

    /// `Number of tracks in the album`
    pub nb_tracks: u64,

    /// `The album's duration in seconds`
    #[serde(rename = "duration")]
    pub duration_in_seconds: u64,

    /// `The number of album's Fans`
    pub fans: u64,

    /// `The album's rate`
    pub rating: u64,

    /// `The album's release date`
    pub release_date: String,

    /// `The record type of the album (EP / ALBUM / etc..)`
    pub record_type: String,

    /// `Whether it's available right now`
    pub available: bool,

    /// `Return an alternative album object if the current album is not available`
    #[serde(default)]
    #[serde(rename = "alternative")]
    pub alternative_album: Option<Box<Album>>,

    /// `API Link to the tracklist of this album`
    #[serde(rename = "tracklist")]
    tracklist_api_url: String,

    /// `Whether the album contains explicit lyrics`
    #[serde(rename = "explicit_lyrics")]
    pub has_explicit_lyrics: bool,

    /// `Return a list of contributors on the album`
    pub contributors: Vec<ContributorArtist>,

    /// `Returns an AlbumArtist object of the artist this album belongs to`
    pub artist: AlbumArtist,

    /// `list of Track objects that belong to this album`
    pub tracks: DeezerArray<AlbumTrack>,
}

impl DeezerObject for Album {
    fn get_api_url(id: u64) -> String {
        format!("album/{}", id)
    }
}

/// Subset of [`Artist`].
///
/// Use [`get_full()`] for the full [`Artist`].
///
/// [`get_full()`]: AlbumArtist::get_full
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AlbumArtist {
    /// `The artist's Deezer id`
    pub id: u64,

    /// `The artist's name`
    pub name: String,

    /// `The url of the artist picture`
    pub picture: String,

    /// `The url of the artist picture in size small`
    pub picture_small: String,

    /// `The url of the artist picture in size medium`
    pub picture_medium: String,

    /// `The url of the artist picture in size big`
    pub picture_big: String,

    /// `The url of the artist picture in size xl`
    pub picture_xl: String,
}

impl AlbumArtist {
    /// Returns the corresponding [`Artist`].
    ///
    /// # Panics
    ///
    /// Can panic when the [artist api](https://developers.deezer.com/api/artist) returns `404 - Not Found`.
    ///
    /// This should never happen as [`AlbumArtist`] references an existing [`Artist`].
    pub async fn get_full(&self) -> Result<Artist> {
        // Safety: unwrap should be okay here, as the artist is referenced by the deezer api
        let artist = Artist::get(self.id).await?.unwrap();
        Ok(artist)
    }
}

/// Subset of [`Artist`].
///
/// Use [`get_full()`] for the full [`Artist`].
///
/// [`get_full()`]: AlbumTrackArtist::get_full
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AlbumTrackArtist {
    /// `Artist's Deezer Id`
    pub id: u64,

    /// `Artist's name`
    pub name: String,

    /// `Artist's Deezer tracklist`
    pub tracklist: String,
}

impl AlbumTrackArtist {
    /// Returns the corresponding [`Artist`].
    ///
    /// # Panics
    ///
    /// Can panic when the [artist api](https://developers.deezer.com/api/artist) returns `404 - Not Found`.
    ///
    /// This should never happen as [`AlbumTrackArtist`] references an existing [`Artist`].
    pub async fn get_full(&self) -> Result<Artist> {
        // Safety: unwrap should be okay here, as the artist is referenced by the deezer api
        let artist = Artist::get(self.id).await?.unwrap();
        Ok(artist)
    }
}

/// Subset of [`Track`].
///
/// Use [`get_full()`] for the full [`Track`].
///
/// [`get_full()`]: AlbumTrack::get_full
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AlbumTrack {
    /// `The track's Deezer id`
    pub id: u64,

    /// `True if the track is readable in the player for the current user`
    pub readable: bool,

    /// `The track's full title`
    pub title: String,

    /// `The track's short title`
    pub title_short: String,

    /// `The track version`
    pub title_version: String,

    /// `The url of the track on Deezer`
    pub link: String,

    /// `The track's duration in seconds`
    #[serde(rename = "duration")]
    pub duration_in_seconds: u64,

    /// `The track's Deezer rank`
    pub rank: u64,

    /// `Whether the track contains explicit lyrics`
    pub explicit_lyrics: bool,

    /// `The url of track's preview file. This file contains the first 30 seconds of the track`
    pub preview: String,

    /// `AlbumTrackArtist object`
    pub artist: AlbumTrackArtist,
}

impl AlbumTrack {
    /// Returns the corresponding [`Track`].
    ///
    /// # Panics
    ///
    /// Can panic when the [track api](https://developers.deezer.com/api/track) returns `404 - Not Found`.
    ///
    /// This should never happen as [`AlbumTrack`] references an existing [`Track`].
    pub async fn get_full(&self) -> Result<Track> {
        // Safety: unwrap should be okay here, as the artist is referenced by the deezer api
        let track = Track::get(self.id).await?.unwrap();
        Ok(track)
    }
}

/// Subset of [`Genre`].
///
/// Use [`get_full()`] for the full [`Genre`].
///
/// [`get_full()`]: AlbumGenre::get_full
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AlbumGenre {
    /// `The Genre's id`
    pub id: u64,

    /// `The Genre's name`
    pub name: String,

    /// `The url of the genre picture`
    pub picture: String,
}

impl AlbumGenre {
    /// Returns the corresponding [`Genre`].
    ///
    /// # Panics
    ///
    /// Can panic when the [genre api](https://developers.deezer.com/api/genre) returns `404 - Not Found`.
    ///
    /// This should never happen as [`AlbumGenre`] references an existing [`Genre`].
    pub async fn get_full(&self) -> Result<Genre> {
        // Safety: unwrap should be okay here, as the artist is referenced by the deezer api
        let genre = Genre::get(self.id).await?.unwrap();
        Ok(genre)
    }
}
