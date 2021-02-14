use serde::{Deserialize, Serialize};
use crate::models::{DeezerConnection, Album, CommentAuthor};

impl DeezerConnection<AlbumCommentConnection> for Album {
    fn get_connection_url(identifier: &str) -> String {
        format!("album/{}/comments", identifier)
    }

    fn get_url(&self) -> String {
        format!("album/{}/comments", self.id)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AlbumCommentConnection {
    /// The comment's Deezer id
    pub id: u64,

    /// The content of the comment
    pub text: String,

    /// The date the comment was posted
    pub date: u64,

    /// User this comment belongs to
    pub author: CommentAuthor,
}
