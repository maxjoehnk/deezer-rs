//! [Comment API](https://developers.deezer.com/api/comment)
#![warn(missing_docs)]
use crate::models::{DeezerObject, User};
use crate::Result;
use serde::{Deserialize, Serialize};

/// Contains all the information provided for a Comment.
///
/// # Examples
///
/// You can query a comment by id via the [`DeezerObject::get()`] method:
///
/// ```rust
/// # use deezer::models::*;
/// # use deezer::DeezerError;
/// # #[tokio::main]
/// # async fn main() -> Result<(), DeezerError> {
/// let comment = Comment::get(4179157801).await?.unwrap();
/// # assert_eq!(comment.id, 4179157801);
/// # Ok(())
/// # }
/// ```
///
/// Or you can use [`DeezerClient::comment()`](crate::DeezerClient::comment()):
///
/// ```rust
/// # use deezer::models::*;
/// # use deezer::{DeezerClient, DeezerError};
/// # #[tokio::main]
/// # async fn main() -> Result<(), DeezerError> {
/// let deezer = DeezerClient::new();
/// let comment = deezer.comment(4179157801).await?.unwrap();
/// # assert_eq!(comment.id, 4179157801);
/// # Ok(())
/// # }
///
/// ```
#[derive(Deserialize, Serialize, Debug)]
pub struct Comment {
    /// The comment's Deezer id
    pub id: u64,

    /// The content of the comment
    pub text: String,

    /// The date the comment was posted
    pub date: u64,

    /// Object the comment belongs to, containing: id, type.
    /// Type can be "artist", "album" or "playlist".
    object: CommentParent,

    /// User this comment belongs to
    pub author: CommentAuthor,
}

impl DeezerObject for Comment {
    fn get_api_url(id: u64) -> String {
        format!("comment/{}", id)
    }
}

/// Subset of [`User`].
///
/// Use [`get_full()`] for the corresponding [`User`] struct.
///
/// [`get_full()`]: CommentAuthor::get_full
#[derive(Deserialize, Serialize, Debug)]
pub struct CommentAuthor {
    /// The comment's Deezer id
    pub id: u64,

    /// The user's Deezer nickname
    pub name: String,

    /// The url of the profil for the user on Deezer
    pub link: String,

    /// The url of the user's profile picture
    pub picture: String,

    /// The url of the user's profile picture in size small
    pub picture_small: String,

    /// The url of the user's profile picture in size medium
    pub picture_medium: String,

    /// The url of the user's profile picture in size big
    pub picture_big: String,

    /// The url of the user's profile picture in size xl
    pub picture_xl: String,
}

impl CommentAuthor {
    /// Returns the full [`User`].
    pub async fn get_full(&self) -> Result<User> {
        // Safety: unwrap should be okay here, as the artist is referenced by the deezer api
        let user = User::get(self.id).await?.unwrap();
        Ok(user)
    }
}

#[derive(Deserialize, Serialize, Debug)]
struct CommentParent {
    id: String,

    #[serde(rename = "type")]
    object_type: String,
}
