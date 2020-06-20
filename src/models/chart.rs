//! [Chart API](https://developers.deezer.com/api/chart)
#[warn(missing_docs)]
use crate::models::{Album, Artist, DeezerArray, DeezerObject, Playlist, PlaylistUser, Track};
use crate::Result;
use serde::{Deserialize, Serialize};

/// Charts of a specified genre
///
/// # Examples
/// ```rust
/// # use deezer::models::*;
/// # use deezer::{DeezerClient, DeezerError};
/// # #[tokio::main]
/// # async fn main() -> Result<(), DeezerError> {
/// let deezer = DeezerClient::new();
/// let charts = deezer.charts().await?;
/// # println!("{:?}", charts);
/// # Ok(())
/// # }
///
/// ```
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Chart {
    /// Vector of ChartTrack objects in the Chart
    pub tracks: DeezerArray<ChartTrack>,

    /// Vector of ChartAlbum objects in the Chart
    pub albums: DeezerArray<ChartAlbum>,

    /// Vector of ChartArtist objects in the Chart
    pub artists: DeezerArray<ChartArtist>,

    /// Vector of Playlist objects in the Chart
    pub playlists: DeezerArray<ChartPlaylist>,
}

/// Subset of [`Track`].
///
/// Use [`get_full()`] for the full [`Track`].
///
/// [`get_full()`]: ChartTrack::get_full
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ChartTrack {
    /// `The track's Deezer id`
    pub id: u64,

    /// `The track's full title`
    pub title: String,

    /// `The track's short title`
    pub title_short: String,

    /// `The track's version`
    pub title_version: String,

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
    pub preview_url: Option<String>,

    /// `The position of the track in the charts`
    pub position: u64,

    /// `Returns an ChartTrackArtist object of the artist this track belongs to`
    pub artist: ChartTrackArtist,

    /// `Returns an ChartTrackAlbum object of the album this track belongs to`
    pub album: ChartTrackAlbum,
}

impl ChartTrack {
    /// Returns the corresponding [`Track`].
    ///
    /// # Panics
    ///
    /// Can panic when the [track api](https://developers.deezer.com/api/track) returns `404 - Not Found`.
    ///
    /// This should never happen as [`ChartTrack`] references an existing [`Track`].
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
/// [`get_full()`]: ChartTrackArtist::get_full
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ChartTrackArtist {
    /// `The artist's Deezer id`
    pub id: u64,

    /// `The artist's name`
    pub name: String,

    /// `The url of the artist on Deezer`
    pub link: String,

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

    /// `True if the artist has a smartradio`
    #[serde(rename = "radio")]
    pub has_radio: bool,
}

impl ChartTrackArtist {
    /// Returns the corresponding [`Artist`].
    ///
    /// # Panics
    ///
    /// Can panic when the [artist api](https://developers.deezer.com/api/artist) returns `404 - Not Found`.
    ///
    /// This should never happen as [`ChartTrackArtist`] references an existing [`Artist`].
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
/// [`get_full()`]: ChartTrackAlbum::get_full
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ChartTrackAlbum {
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

impl ChartTrackAlbum {
    /// Returns the corresponding [`Album`].
    ///
    /// # Panics
    ///
    /// Can panic when the [album api](https://developers.deezer.com/api/album) returns `404 - Not Found`.
    ///
    /// This should never happen as [`ChartTrackAlbum`] references an existing [`Album`].
    pub async fn get_full(&self) -> Result<Album> {
        // Safety: unwrap should be okay here, as the artist is referenced by the deezer api
        let album = Album::get(self.id).await?.unwrap();
        Ok(album)
    }
}

/// Subset of [`Album`].
///
/// Use [`get_full()`] for the full [`Album`].
///
/// [`get_full()`]: ChartAlbum::get_full
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ChartAlbum {
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

    /// `The record type of the album (EP / ALBUM / etc..)`
    pub record_type: String,

    /// `Whether the album contains explicit lyrics`
    #[serde(rename = "explicit_lyrics")]
    pub has_explicit_lyrics: bool,

    /// `The position of the album in the charts`
    pub position: u64,

    /// `Returns an ChartAlbumArtist object of the artist this album belongs to`
    pub artist: ChartAlbumArtist,
}

impl ChartAlbum {
    /// Returns the corresponding [`Album`].
    ///
    /// # Panics
    ///
    /// Can panic when the [album api](https://developers.deezer.com/api/album) returns `404 - Not Found`.
    ///
    /// This should never happen as [`ChartAlbum`] references an existing [`Album`].
    pub async fn get_full(&self) -> Result<Album> {
        // Safety: unwrap should be okay here, as the artist is referenced by the deezer api
        let album = Album::get(self.id).await?.unwrap();
        Ok(album)
    }
}

/// Subset of [`Artist`].
///
/// Use [`get_full()`] for the full [`Artist`].
///
/// [`get_full()`]: ChartAlbumArtist::get_full
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ChartAlbumArtist {
    /// `The artist's Deezer id`
    pub id: u64,

    /// `The artist's name`
    pub name: String,

    /// `The url of the artist on Deezer`
    pub link: String,

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

    /// `True if the artist has a smartradio`
    #[serde(rename = "radio")]
    pub has_radio: bool,
}

impl ChartAlbumArtist {
    /// Returns the full [`Artist`].
    ///
    /// # Panics
    ///
    /// Can panic when the [artist api](https://developers.deezer.com/api/artist) returns `404 - Not Found`.
    ///
    /// This should never happen as [`ChartAlbumArtist`] references an existing [`Artist`].
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
/// [`get_full()`]: ChartArtist::get_full
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ChartArtist {
    /// `The artist's Deezer id`
    pub id: u64,

    /// `The artist's name`
    pub name: String,

    /// `The url of the artist on Deezer`
    pub link: String,

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

    /// `True if the artist has a smartradio`
    #[serde(rename = "radio")]
    pub has_radio: bool,

    /// `The position of the artist in the charts`
    pub position: u64,
}

impl ChartArtist {
    /// Returns the full [`Artist`].
    ///
    /// # Panics
    ///
    /// Can panic when the [artist api](https://developers.deezer.com/api/artist) returns `404 - Not Found`.
    ///
    /// This should never happen as [`ChartArtist`] references an existing [`Artist`].
    pub async fn get_full(&self) -> Result<Artist> {
        let artist = Artist::get(self.id).await?.unwrap();
        Ok(artist)
    }
}

/// Subset of [`Playlist`].
///
/// Use [`get_full()`] for the full [`Playlist`].
///
/// [`get_full()`]: ChartPlaylist::get_full
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ChartPlaylist {
    /// The playlist's Deezer id
    pub id: u64,

    /// The playlist's title
    pub title: String,

    /// If the playlist is public or not
    #[serde(rename = "public")]
    pub is_public: bool,

    /// The url of the playlist on Deezer
    pub link: String,

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

    /// The position of the playlist in the charts
    #[serde(default)]
    pub position: u64,

    /// User object
    pub user: PlaylistUser,
}

impl ChartPlaylist {
    /// Returns the full [`Playlist`].
    ///
    /// # Panics
    ///
    /// Can panic when the [playlist api](https://developers.deezer.com/api/playlist) returns `404 - Not Found`.
    ///
    /// This should never happen as [`ChartPlaylist`] references an existing [`Playlist`].
    pub async fn get_full(&self) -> Result<Playlist> {
        // Safety: unwrap should be okay here, as the artist is referenced by the deezer api
        let playlist = Playlist::get(self.id).await?.unwrap();
        Ok(playlist)
    }
}
