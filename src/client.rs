#![warn(missing_docs)]

use std::collections::HashMap;

use lazy_static::lazy_static;
use reqwest::StatusCode;
use serde::de::DeserializeOwned;

use crate::models::*;
use crate::Result;

const BASE_URL: &str = "https://api.deezer.com";
lazy_static! {
    static ref NO_PARAMS: HashMap<String, String> = HashMap::new();
}

/// Entrypoint to interact with all deezer apis
#[derive(Debug, Clone)]
pub struct DeezerClient {
    client: reqwest::Client,
}

impl DeezerClient {
    /// Create a new unauthenticated client instance
    pub fn new() -> Self {
        DeezerClient {
            client: reqwest::Client::new(),
        }
    }

    /// Returns the [`Album`] with the given id.
    ///
    /// [Deezer Api Documentation](https://developers.deezer.com/api/album)
    pub async fn album(&self, id: u64) -> Result<Option<Album>> {
        self.get_entity(id).await
    }

    /// Returns the [`Artist`] with the given id.
    ///
    /// [Deezer Api Documentation](https://developers.deezer.com/api/artist)
    pub async fn artist(&self, id: u64) -> Result<Option<Artist>> {
        self.get_entity(id).await
    }

    /// Returns the [`Album`] for Artist with the given id.
    ///
    /// [Deezer Api Documentation](https://developers.deezer.com/api/artist/albums)
    pub async fn artist_albums(&self, id: u64, limit: Option<u32>,
                               offset: Option<u32>) -> Result<Vec<ArtistAlbum>> {
         self.get_subresource(id, limit, offset).await
    }

    /// Returns the [`Comment`] with the given id.
    ///
    /// [Deezer Api Documentation](https://developers.deezer.com/api/comment)
    pub async fn comment(&self, id: u64) -> Result<Option<Comment>> {
        self.get_entity(id).await
    }

    /// Returns the [`Editorial`] with the given id.
    ///
    /// [Deezer Api Documentation](https://developers.deezer.com/api/editorial)
    pub async fn editorial(&self, id: u64) -> Result<Option<Editorial>> {
        self.get_entity(id).await
    }

    /// Returns a List of all [`Editorial`]s.
    ///
    /// [Deezer Api Documentation](https://developers.deezer.com/api/editorial)
    pub async fn editorials(&self) -> Result<Vec<Editorial>> {
        self.get_all().await
    }

    /// Returns the [`Genre`] with the given id.
    ///
    /// [Deezer Api Documentation](https://developers.deezer.com/api/genre)
    pub async fn genre(&self, id: u64) -> Result<Option<Genre>> {
        self.get_entity(id).await
    }

    /// Returns a List of all [`Genre`]s.
    ///
    /// [Deezer Api Documentation](https://developers.deezer.com/api/genre)
    pub async fn genres(&self) -> Result<Vec<Genre>> {
        self.get_all().await
    }

    /// Returns the [`Playlist`] with the given id.
    ///
    /// [Deezer Api Documentation](https://developers.deezer.com/api/playlist)
    pub async fn playlist(&self, id: u64) -> Result<Option<Playlist>> {
        self.get_entity(id).await
    }

    /// Returns the [`Radio`] with the given id.
    ///
    /// [Deezer Api Documentation](https://developers.deezer.com/api/radio)
    pub async fn radio(&self, id: u64) -> Result<Option<Radio>> {
        self.get_entity(id).await
    }

    /// Returns a List of all [`Radio`]s.
    ///
    /// [Deezer Api Documentation](https://developers.deezer.com/api/radio)
    pub async fn radios(&self) -> Result<Vec<Radio>> {
        self.get_all().await
    }

    /// Returns the [`Track`] with the given id.
    ///
    /// [Deezer Api Documentation](https://developers.deezer.com/api/track)
    pub async fn track(&self, id: u64) -> Result<Option<Track>> {
        self.get_entity(id).await
    }

    /// Returns the [`User`] with the given id.
    ///
    /// [Deezer Api Documentation](https://developers.deezer.com/api/user)
    pub async fn user(&self, id: u64) -> Result<Option<User>> {
        self.get_entity(id).await
    }

    /// Returns the information about the API in the current country
    ///
    /// [Deezer Api Documentation](https://developers.deezer.com/api/infos)
    pub async fn api_info(&self) -> Result<Infos> {
        let url = format!("{}/infos", BASE_URL);
        self.get(&url).await
    }

    /// Returns charts of a specified genre
    ///
    /// [Deezer Api Documentation](https://developers.deezer.com/api/chart)
    pub async fn charts(&self) -> Result<Chart> {
        let url = format!("{}/chart", BASE_URL);
        self.get(&url).await
    }

    /// Returns the user's options
    ///
    /// [Deezer Api Documentation](https://developers.deezer.com/api/options)
    pub async fn user_options(&self) -> Result<Options> {
        let url = format!("{}/options", BASE_URL);
        self.get(&url).await
    }

    pub(crate) async fn get_entity<T>(&self, id: u64) -> Result<Option<T>>
    where
        T: DeezerObject,
    {
        let url = T::get_api_url(id);
        let url = format!("{}/{}", BASE_URL, url);

        let res = self.client.get(&url).send().await?;
        if res.status() == StatusCode::NOT_FOUND {
            return Ok(None);
        }
        let body = res.error_for_status()?.json().await?;

        Ok(Some(body))
    }

    pub(crate) async fn get_all<T>(&self) -> Result<Vec<T>>
    where
        T: DeezerEnumerable,
    {
        let url = T::get_all_api_url();
        let url = format!("{}/{}", BASE_URL, url);

        let res: DeezerArray<T> = self.get(&url).await?;

        Ok(res.data)
    }

    pub(crate) async fn get_subresource<T>(&self, id: u64, limit: Option<u32>,
                                           offset: Option<u32>) -> Result<Vec<T>>
        where
            T: DeezerObject
    {
        let url = T::get_api_url(id);
        let url = format!("{}/{}", BASE_URL, url);

        let mut params: HashMap<String, String> = HashMap::new();
        if let Some(_limit) = limit {
            params.insert("limit".to_owned(), _limit.to_string());
        }
        if let Some(_offset) = offset {
            params.insert("offset".to_owned(), _offset.to_string());
        }

        let res: DeezerArray<T> = self.get_with_params(&url, &params).await?;

        Ok(res.data)
    }

    async fn get_with_params<T: DeserializeOwned>(&self, url: &str, query_params: &HashMap<String, String>) -> Result<T> {
        let res = self
            .client
            .get(url)
            .query(query_params)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(res)
    }

    async fn get<T: DeserializeOwned>(&self, url: &str) -> Result<T> {
        self.get_with_params(url, &NO_PARAMS).await
    }

}
