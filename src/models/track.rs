//! [Track API](https://developers.deezer.com/api/track)
#![warn(missing_docs)]
use serde::{Deserialize, Serialize};

use crate::models::{Album, Artist, ContributorArtist, DeezerObject};
use crate::Result;

/// Contains all the information provided for a Track.
///
/// # Examples
///
/// You can query a track by id via the [`DeezerObject::get()`] method:
///
/// ```rust
/// # use deezer::models::*;
/// # use deezer::DeezerError;
/// # #[tokio::main]
/// # async fn main() -> Result<(), DeezerError> {
/// let track = Track::get(912486).await?.unwrap();
/// # assert_eq!(track.id, 912486);
/// # Ok(())
/// # }
/// ```
///
/// Or you can use [`DeezerClient::track()`](crate::DeezerClient::track()):
///
/// ```rust
/// # use deezer::models::*;
/// # use deezer::{DeezerClient, DeezerError};
/// # #[tokio::main]
/// # async fn main() -> Result<(), DeezerError> {
/// let deezer = DeezerClient::new();
/// let track = deezer.track(912486).await?.unwrap();
/// # assert_eq!(track.id, 912486);
/// # Ok(())
/// # }
/// ```
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Track {
    /// `The track's Deezer id`
    pub id: u64,

    /// `True if the track is readable in the player for the current user`
    pub readable: bool,

    /// `The track's full title`
    pub title: String,

    /// `The track's short title`
    pub title_short: String,

    /// `The track's version`
    pub title_version: String,

    /// `The track's unseen status`
    #[serde(default)]
    pub unseen: Option<bool>,

    /// `The track's isrc`
    pub isrc: String,

    /// `The url of the track on Deezer`
    pub link: String,

    /// `The share link of the track on Deezer`
    #[serde(rename = "share")]
    pub share_link: String,

    /// `The track's duration in seconds`
    #[serde(rename = "duration")]
    pub duration_in_seconds: u64,

    /// `The position of the track in its album`
    #[serde(rename = "track_position")]
    pub track_position_in_album: u64,

    /// `The track's album's disk number`
    #[serde(rename = "disk_number")]
    pub album_disk_number: u64,

    /// `The track's Deezer rank`
    pub rank: u64,

    /// `The track's release date`
    pub release_date: String,

    /// `Whether the track contains explicit lyrics`
    #[serde(rename = "explicit_lyrics")]
    pub has_explicit_lyrics: bool,

    /// `The url of track's preview file. This file contains the first 30 seconds of the track`
    #[serde(default)]
    pub preview_url: Option<String>,

    /// `Beats per minute`
    pub bpm: f32,

    /// `Signal strength`
    pub gain: f32,

    /// `List of countries where the track is available`
    pub available_countries: Vec<String>,

    /// `Return an alternative readable track if the current track is not readable`
    #[serde(rename = "alternative")]
    #[serde(default)]
    pub alternative_track_id: Option<u64>,

    /// `Return a list of contributors on the track`
    pub contributors: Vec<ContributorArtist>,

    /// `Artist Object`
    pub artist: TrackArtist,

    /// `Album Object`
    pub album: TrackAlbum,
}

impl DeezerObject for Track {
    fn get_api_url(id: u64) -> String {
        format!("track/{}", id)
    }
}

/// Subset of [`Artist`].
///
/// Use [`get_full()`] for the full [`Artist`].
///
/// [`get_full()`]: TrackArtist::get_full
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TrackArtist {
    /// `The artist's Deezer id`
    pub id: u64,

    /// `The artist's name`
    pub name: String,

    /// `The url of the artist on Deezer`
    pub link: String,

    /// `The share link of the artist on Deezer`
    #[serde(rename = "share")]
    pub share_link: String,

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

    /// `The number of artist's albums`
    #[serde(default)]
    pub nb_album: Option<u64>,

    /// `The number of artist's fans`
    #[serde(default)]
    pub nb_fan: Option<u64>,

    /// `True if the artist has a smartradio`
    #[serde(rename = "radio")]
    pub has_radio: bool,

    /// `API Link to the top of this artist`
    pub tracklist: String,
}

impl TrackArtist {
    /// Returns the corresponding [`Artist`].
    pub async fn get_full(&self) -> Result<Artist> {
        // Safety: unwrap should be okay here, as the artist is referenced by the deezer api
        let artist = Artist::get(self.id).await?.unwrap();
        Ok(artist)
    }
}

/// Subset of [`Album`].
///
/// Use [`get_full()`] for the full [`Album`].
///
/// [`get_full()`]: TrackAlbum::get_full
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TrackAlbum {
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

    /// `The album's release date`
    pub release_date: String,
}

impl TrackAlbum {
    /// Returns the full [`Album`].
    pub async fn get_full(&self) -> Result<Album> {
        // Safety: unwrap should be okay here, as the artist is referenced by the deezer api
        let album = Album::get(self.id).await?.unwrap();
        Ok(album)
    }
}
