#![warn(missing_docs)]

use std::collections::HashMap;

use reqwest::StatusCode;
use serde::de::DeserializeOwned;

use crate::models::*;
use crate::clients::*;
use crate::Result;

const BASE_URL: &str = "https://api.deezer.com";

/// Entrypoint to interact with all deezer apis
#[derive(Default, Debug, Clone)]
pub struct DeezerClient {
    client: reqwest::Client,
}

impl DeezerClient {
    /// Create a new unauthenticated client instance
    pub fn new() -> Self {
        Default::default()
    }

    /// Returns a client to explore the albums api
    ///
    /// [Deezer Api Documentation](https://developers.deezer.com/api/album)
    pub fn albums(&self) -> AlbumsClient {
        AlbumsClient::new(self.clone())
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
        self.get("infos").await
    }

    /// Returns charts of a specified genre
    ///
    /// [Deezer Api Documentation](https://developers.deezer.com/api/chart)
    pub async fn charts(&self) -> Result<Chart> {
        self.get("chart").await
    }

    /// Returns the user's options
    ///
    /// [Deezer Api Documentation](https://developers.deezer.com/api/options)
    pub async fn user_options(&self) -> Result<Options> {
        self.get("options").await
    }

    pub(crate) async fn get_entity<T>(&self, id: u64) -> Result<Option<T>>
    where
        T: DeezerObject,
    {
        let url = T::get_by_id(id);

        self.get_entity_from_url(url).await
    }

    pub(crate) async fn get_entity_from_url<T>(&self, url: String) -> Result<Option<T>>
        where
            T: DeserializeOwned,
    {
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

        let res: DeezerArray<T> = self.get(&url).await?;

        Ok(res.data)
    }

    pub(crate) async fn get_array_params<T>(&self, url: &str, query_params: &HashMap<String, String>) -> Result<Vec<T>>
        where
            T: DeserializeOwned,
    {
        let res: DeezerArray<T> = self.get_with_params(url, query_params).await?;

        Ok(res.data)
    }

    pub(crate) async fn get_connection<T, C>(&self, identifier: &str) -> Result<Vec<C>>
        where
            T: DeezerConnection<C>,
            C: DeserializeOwned,
    {
        let url = T::get_connection_url(identifier);

        let res: DeezerArray<C> = self.get(&url).await?;

        Ok(res.data)
    }

    pub(crate) async fn _get_connection<T, C, L: Into<Option<u32>>, O: Into<Option<u32>>>(&self, identifier: &str, limit: L,
                                                                                         offset: O) -> Result<Vec<C>>
        where
            T: DeezerConnection<C>,
            C: DeezerObject {
        let url = T::get_connection_url(identifier);

        let mut params: HashMap<String, String> = HashMap::new();
        if let Some(limit) = limit.into() {
            params.insert("limit".to_owned(), limit.to_string());
        }
        if let Some(offset) = offset.into() {
            params.insert("offset".to_owned(), offset.to_string());
        }

        let res: DeezerArray<C> = self.get_with_params(&url, &params).await?;

        Ok(res.data)
    }

    async fn get_with_params<T: DeserializeOwned>(&self, url: &str, query_params: &HashMap<String, String>) -> Result<T> {
        self.get_with_optional_params(url, Some(query_params)).await
    }

    async fn get<T: DeserializeOwned>(&self, url: &str) -> Result<T> {
        self.get_with_optional_params(url, None).await
    }

    async fn get_with_optional_params<T: DeserializeOwned>(&self, url: &str, query_params: Option<&HashMap<String, String>>) -> Result<T> {
        let url = format!("{}/{}", BASE_URL, url);
        let mut request_builder = self
            .client
            .get(&url);
        if let Some(params) = query_params {
            request_builder = request_builder.query(params);
        }
        let res =
            request_builder
                .send()
                .await?
                .error_for_status()?
                .json()
                .await?;

        Ok(res)
    }
}
