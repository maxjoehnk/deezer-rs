use serde::{Deserialize, Serialize};
use crate::models::{DeezerConnection, Album};

impl DeezerConnection<AlbumTrackConnection> for Album {
    fn get_connection_url(identifier: &str) -> String {
        format!("album/{}/tracks", identifier)
    }

    fn get_url(&self) -> String {
        format!("album/{}/tracks", self.id)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AlbumTrackConnection {
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

    /// `The track's isrc`
    pub isrc: String,

    /// `The url of the track on Deezer`
    pub link: String,

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

    /// `Whether the track contains explicit lyrics`
    #[serde(rename = "explicit_lyrics")]
    pub has_explicit_lyrics: bool,

    /// `The url of track's preview file. This file contains the first 30 seconds of the track`
    #[serde(default, rename = "preview")]
    pub preview_url: Option<String>,

    /// `Return an alternative readable track if the current track is not readable`
    #[serde(rename = "alternative")]
    #[serde(default)]
    pub alternative_track_id: Option<u64>,

    /// `Artist Object`
    pub artist: AlbumTrackConnectionArtist,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AlbumTrackConnectionArtist {
    /// `The artist's Deezer id`
    pub id: u64,

    /// `The artist's name`
    pub name: String,
}
