//! Contains all api objects
#![warn(missing_docs)]
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::DeezerClient;
use crate::Result;

#[doc(inline)]
pub use self::album::*;
#[doc(inline)]
pub use self::artist::*;
#[doc(inline)]
pub use self::chart::*;
#[doc(inline)]
pub use self::comment::*;
#[doc(inline)]
pub use self::editorial::*;
#[doc(inline)]
pub use self::genre::*;
#[doc(inline)]
pub use self::infos::*;
#[doc(inline)]
pub use self::options::*;
#[doc(inline)]
pub use self::playlist::*;
#[doc(inline)]
pub use self::radio::*;
#[doc(inline)]
pub use self::track::*;
#[doc(inline)]
pub use self::user::*;
use std::ops::Deref;

pub mod album;
pub mod artist;
pub mod chart;
pub mod comment;
pub mod editorial;
pub mod genre;
pub mod infos;
pub mod options;
pub mod playlist;
pub mod radio;
pub mod track;
pub mod user;

/// Wrapper around deezer array types
///
/// Some deezer models return an object with a `data` property containing the actual array.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeezerArray<T> {
    pub data: Vec<T>,
}

impl<T> DeezerArray<T> {
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.data.iter()
    }
}

impl<T> Deref for DeezerArray<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        self.data.deref()
    }
}

impl<T> AsRef<[T]> for DeezerArray<T> {
    fn as_ref(&self) -> &[T] {
        &self.data
    }
}

impl<T> IntoIterator for DeezerArray<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

/// A by id queryable api object of the deezer api
#[async_trait]
pub trait DeezerObject: serde::de::DeserializeOwned {
    /// Get a relative api url for the given `id`
    fn get_api_url(id: u64) -> String;

    /// Fetch an api object with the given `id`
    async fn get(id: u64) -> Result<Option<Self>> {
        let client = DeezerClient::new();

        client.get_entity(id).await
    }
}

// Represents an api object which has a list method
#[async_trait]
pub trait DeezerEnumerable: DeezerObject {
    fn get_all_api_url() -> String;

    async fn get_all() -> Result<Vec<Self>> {
        let client = DeezerClient::new();

        client.get_all().await
    }
}
