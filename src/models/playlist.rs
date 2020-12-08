//! [Playlists API](https://developers.deezer.com/api/playlists)
#![warn(missing_docs)]
use crate::models::{Album, Artist, DeezerArray, DeezerObject, Track, User};
use crate::Result;
use serde::{Deserialize, Serialize};

/// Contains all the information provided for a Playlist.
///
/// # Examples
///
/// You can query a playlist by id via the [`DeezerObject::get()`] method:
///
/// ```rust
/// # use deezer::models::*;
/// # use deezer::DeezerError;
/// # #[tokio::main]
/// # async fn main() -> Result<(), DeezerError> {
/// let playlist = Playlist::get(908622995).await?.unwrap();
/// # assert_eq!(playlist.id, 908622995);
/// # Ok(())
/// # }
/// ```
///
/// Or you can use [`DeezerClient::playlist()`](crate::DeezerClient::playlist()):
///
/// ```rust
/// # use deezer::models::*;
/// # use deezer::{DeezerClient, DeezerError};
/// # #[tokio::main]
/// # async fn main() -> Result<(), DeezerError> {
/// let deezer = DeezerClient::new();
/// let album = deezer.playlist(908622995).await?.unwrap();
/// # assert_eq!(album.id, 908622995);
/// # Ok(())
/// # }
///
/// ```
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Playlist {
    /// The playlist's Deezer id
    pub id: u64,

    /// The playlist's title
    pub title: String,

    /// The playlist description
    pub description: String,

    /// The playlist's duration in seconds
    #[serde(rename = "duration")]
    pub duration_in_seconds: u64,

    /// If the playlist is public or not
    #[serde(rename = "public")]
    pub is_public: bool,

    /// If the playlist is the love tracks playlist
    pub is_loved_track: bool,

    /// If the playlist is collaborative or not
    #[serde(rename = "collaborative")]
    pub is_collaborative: bool,

    /// The playlist's rate
    #[serde(default)]
    pub rating: Option<u64>,

    /// Number of tracks in the playlist
    pub nb_tracks: u64,

    /// Number of tracks not seen
    #[serde(default)]
    pub unseen_track_count: Option<u64>,

    /// The number of playlist's fans
    pub fans: u64,

    /// The url of the playlist on Deezer
    pub link: String,

    /// The share link of the playlist on Deezer
    #[serde(rename = "share")]
    pub share_link: String,

    /// The url of the playlist's cover
    pub picture: String,

    /// The url of the playlist's cover in size small
    pub picture_small: String,

    /// The url of the playlist's cover in size medium
    pub picture_medium: String,

    /// The url of the playlist's cover in size big
    pub picture_big: String,

    /// The url of the playlist's cover in size xl
    pub picture_xl: String,

    /// The checksum for the track list
    pub checksum: String,

    /// User object containing : id, name
    pub creator: PlaylistUser,

    /// Vector of Track object
    pub tracks: DeezerArray<PlaylistTrack>,
}

impl DeezerObject for Playlist {
    fn get_api_url(id: u64) -> String {
        format!("playlist/{}", id)
    }
}

/// Subset of [`User`].
///
/// Use [`get_full()`] for the full [`User`].
///
/// [`get_full()`]: PlaylistUser::get_full
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PlaylistUser {
    /// The user's Deezer ID
    pub id: u64,

    /// The user's Deezer nickname
    pub name: String,
}

impl PlaylistUser {
    /// Returns the corresponding [`User`].
    pub async fn get_full(&self) -> Result<User> {
        // Safety: unwrap should be okay here, as the artist is referenced by the deezer api
        let user = User::get(self.id).await?.unwrap();
        Ok(user)
    }
}

/// Subset of [`Track`].
///
/// Use [`get_full()`] for the full [`Track`].
///
/// [`get_full()`]: PlaylistTrack::get_full
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PlaylistTrack {
    /// `The track's Deezer id`
    pub id: u64,

    /// `True if the track is readable in the player for the current user`
    pub readable: bool,

    /// `The track's fulltitle`
    pub title: String,

    /// `The track's short title`
    pub title_short: String,

    /// `The track's version`
    pub title_version: Option<String>,

    /// `The track's unseen status`
    #[serde(default)]
    pub unseen: bool,

    /// `The url of the track on Deezer`
    pub link: String,

    /// `The track's duration in seconds`
    #[serde(rename = "duration")]
    pub duration_in_seconds: u64,

    /// `The track's Deezer rank`
    pub rank: u64,

    /// `Whether the track contains explicit lyrics`
    #[serde(rename = "explicit_lyrics")]
    pub has_explicit_lyrics: bool,

    /// `The url of track's preview file. This file contains the first 30 seconds of the track`
    #[serde(default)]
    pub preview_url: String,

    /// `The time when the track has been added to the playlist`
    #[serde(rename = "time_add")]
    pub added_on: u64,

    /// `Artist Object`
    pub artist: PlaylistTrackArtist,

    /// `Album Object`
    pub album: PlaylistTrackAlbum,
}

impl PlaylistTrack {
    /// Returns the full [`Track`].
    pub async fn get_full(&self) -> Result<Track> {
        // Safety: unwrap should be okay here, as the artist is referenced by the deezer api
        let track = Track::get(self.id).await?.unwrap();
        Ok(track)
    }
}

/// Subset of [`Artist`].
///
/// Use [`get_full()`] for the full [`Artist`].
///
/// [`get_full()`]: PlaylistTrackArtist::get_full
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PlaylistTrackArtist {
    /// `The artist's Deezer id`
    pub id: u64,

    /// `The artist's name`
    pub name: String,

    /// `The url of the artist on Deezer`
    pub link: String,
}

impl PlaylistTrackArtist {
    /// Returns the full [`Artist`].
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
/// [`get_full()`]: PlaylistTrackAlbum::get_full
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PlaylistTrackAlbum {
    /// `The Deezer album id`
    pub id: u64,

    /// `The album title`
    pub title: String,

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
}

impl PlaylistTrackAlbum {
    /// Returns the full [`Album`].
    pub async fn get_full(&self) -> Result<Album> {
        // Safety: unwrap should be okay here, as the artist is referenced by the deezer api
        let album = Album::get(self.id).await?.unwrap();
        Ok(album)
    }
}
