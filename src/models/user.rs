//! [User API](https://developers.deezer.com/api/user)
#![warn(missing_docs)]
use crate::models::DeezerObject;
use serde::{Deserialize, Serialize};

/// Contains all the information provided for a User.
///
/// # Examples
///
/// You can query a user by id via the [`DeezerObject::get()`] method:
///
/// ```rust
/// # use deezer::models::*;
/// # use deezer::DeezerError;
/// # #[tokio::main]
/// # async fn main() -> Result<(), DeezerError> {
/// let user = User::get(12).await?.unwrap();
/// # assert_eq!(user.id, 12);
/// # Ok(())
/// # }
/// ```
///
/// Or you can use [`DeezerClient::user()`](crate::DeezerClient::user()):
///
/// ```rust
/// # use deezer::models::*;
/// # use deezer::{DeezerClient, DeezerError};
/// # #[tokio::main]
/// # async fn main() -> Result<(), DeezerError> {
/// let deezer = DeezerClient::new();
/// let user = deezer.user(12).await?.unwrap();
/// # assert_eq!(user.id, 12);
/// # Ok(())
/// # }
///
/// ```
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User {
    /// The user's Deezer ID
    pub id: u64,

    /// The user's Deezer nickname
    pub name: String,

    /// The user's last name
    #[serde(default)]
    #[serde(rename = "lastname")]
    pub last_name: String,

    /// The user's first name
    #[serde(default)]
    #[serde(rename = "firstname")]
    pub first_name: String,

    /// The user's email
    #[serde(default)]
    pub email: String,

    /// The user's status
    #[serde(default)]
    pub status: u64,

    /// The user's birthday
    #[serde(default)]
    pub birthday: String,

    /// The user's inscription date
    #[serde(default)]
    pub inscription_date: String,

    /// The user's gender : F or M
    #[serde(default)]
    pub gender: String,

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

    /// The user's country
    pub country: String,

    /// The user's language
    #[serde(default)]
    pub lang: String,

    /// If the user is a kid or not
    #[serde(default)]
    pub is_kid: bool,

    /// API Link to the flow of this user
    #[serde(rename = "tracklist")]
    pub track_list: String,
}

impl DeezerObject for User {
    fn get_api_url(id: u64) -> String {
        format!("user/{}", id)
    }
}
