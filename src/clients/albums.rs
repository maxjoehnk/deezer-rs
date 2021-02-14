use crate::{Result, DeezerClient};
use crate::search::SearchClient;
use crate::models::{Album, AlbumCommentConnection, AlbumFanConnection, AlbumTrackConnection, SearchedAlbum};

pub struct AlbumsClient {
    client: DeezerClient,
}

impl AlbumsClient {
    pub(crate) fn new(client: DeezerClient) -> Self {
        Self {
            client
        }
    }

    /// Query resources for a single album by id
    pub fn id(self, id: u64) -> AlbumClient {
        AlbumClient {
            identifier: id.into(),
            client: self.client,
        }
    }

    /// Query resources for a single album by id
    ///
    /// # Example
    /// ```rust
    /// # use deezer::{DeezerError, DeezerClient};
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), DeezerError> {
    /// # let client = DeezerClient::new();
    /// let album = client
    ///   .albums()
    ///   .upc("724384960650")
    ///   .get()
    ///   .await?;
    /// # Ok(())
    /// # }
    pub fn upc<S: Into<String>>(self, upc: S) -> AlbumClient {
        AlbumClient {
            identifier: AlbumIdentifier::Upc(upc.into()),
            client: self.client,
        }
    }

    /// Search for albums
    ///
    /// # Example
    /// ```rust
    /// # use deezer::{DeezerError, DeezerClient};
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), DeezerError> {
    /// # let client = DeezerClient::new();
    /// let albums = client
    ///   .albums()
    ///   .search("eminem")
    ///   .send()
    ///   .await?;
    /// # assert!(albums.len() > 0);
    /// # Ok(())
    /// # }
    /// ```
    pub fn search<S: Into<String>>(self, query: S) -> SearchClient<SearchedAlbum> {
        SearchClient::new(self.client, query)
    }
}

pub struct AlbumClient {
    client: DeezerClient,
    identifier: AlbumIdentifier,
}

impl AlbumClient {
    /// Fetch album
    ///
    /// # Example
    /// ```rust
    /// # use deezer::{DeezerError, DeezerClient};
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), DeezerError> {
    /// # let client = DeezerClient::new();
    /// let album = client
    ///   .albums()
    ///   .id(302127)
    ///   .get()
    ///   .await?;
    /// # Ok(())
    /// # }
    pub async fn get(&self) -> Result<Option<Album>> {
        let url = format!("album/{}", self.identifier.serialize());
        self.client.get_entity_from_url(url).await
    }

    /// Query comments for album
    ///
    /// # Example
    /// ```rust
    /// # use deezer::{DeezerError, DeezerClient};
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), DeezerError> {
    /// # let client = DeezerClient::new();
    /// let comments = client
    ///   .albums()
    ///   .id(302127)
    ///   .comments()
    ///   .await?;
    /// # assert!(comments.len() > 0);
    /// # Ok(())
    /// # }
    pub async fn comments(&self) -> Result<Vec<AlbumCommentConnection>> {
        self.client.get_connection::<Album, _>(&self.identifier.serialize()).await
    }

    /// Query fans for album
    ///
    /// # Example
    /// ```rust
    /// # use deezer::{DeezerError, DeezerClient};
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), DeezerError> {
    /// # let client = DeezerClient::new();
    /// let users = client
    ///   .albums()
    ///   .id(302127)
    ///   .fans()
    ///   .await?;
    /// # assert!(users.len() > 0);
    /// # Ok(())
    /// # }
    pub async fn fans(&self) -> Result<Vec<AlbumFanConnection>> {
        self.client.get_connection::<Album, _>(&self.identifier.serialize()).await
    }

    /// Query tracks for album
    ///
    /// # Example
    /// ```rust
    /// # use deezer::{DeezerError, DeezerClient};
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), DeezerError> {
    /// # let client = DeezerClient::new();
    /// let tracks = client
    ///   .albums()
    ///   .id(302127)
    ///   .tracks()
    ///   .await?;
    /// # assert!(tracks.len() > 0);
    /// # Ok(())
    /// # }
    pub async fn tracks(&self) -> Result<Vec<AlbumTrackConnection>> {
        self.client.get_connection::<Album, _>(&self.identifier.serialize()).await
    }
}

#[derive(Debug, Clone)]
pub enum AlbumIdentifier {
    Id(u64),
    Upc(String),
}

impl AlbumIdentifier {
    fn serialize(&self) -> String {
        match self {
            AlbumIdentifier::Id(id) => id.to_string(),
            AlbumIdentifier::Upc(upc) => format!("upc:{}", upc),
        }
    }
}

impl From<u64> for AlbumIdentifier {
    fn from(id: u64) -> Self {
        AlbumIdentifier::Id(id)
    }
}
