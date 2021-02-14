use serde::{Deserialize, Serialize};
use crate::search::{Searchable, SearchResource};
use crate::models::AlbumArtist;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SearchedAlbum {
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

    /// `Number of tracks in the album`
    pub nb_tracks: u64,

    /// `The record type of the album (EP / ALBUM / etc..)`
    pub record_type: String,

    /// `Whether the album contains explicit lyrics`
    #[serde(rename = "explicit_lyrics")]
    pub has_explicit_lyrics: bool,

    /// `Returns an AlbumArtist object of the artist this album belongs to`
    pub artist: AlbumArtist,
}

impl Searchable for SearchedAlbum {
    fn resource() -> SearchResource {
        SearchResource::Albums
    }
}
