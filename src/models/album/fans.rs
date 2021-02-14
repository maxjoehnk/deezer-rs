use serde::{Deserialize, Serialize};
use crate::models::{DeezerConnection, Album};

impl DeezerConnection<AlbumFanConnection> for Album {
    fn get_connection_url(identifier: &str) -> String {
        format!("album/{}/fans", identifier)
    }

    fn get_url(&self) -> String {
        format!("album/{}/fans", self.id)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AlbumFanConnection {
    /// The user's Deezer ID
    pub id: u64,

    /// The user's Deezer nickname
    pub name: String,

    /// The url of the profil for the user on Deezer
    pub link: String,

    /// The url of the user's profile picture.
    pub picture: String,

    /// The url of the user's profile picture in size small.
    pub picture_small: String,

    /// The url of the user's profile picture in size medium.
    pub picture_medium: String,

    /// The url of the user's profile picture in size big.
    pub picture_big: String,

    /// The url of the user's profile picture in size xl.
    pub picture_xl: String,
}
