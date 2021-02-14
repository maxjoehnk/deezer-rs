use crate::{DeezerClient, DeezerError};
use std::marker::PhantomData;
use std::collections::HashMap;
use std::fmt::{Display};

pub struct SearchClient<Model> {
    pub(crate) client: DeezerClient,
    pub(crate) query: SearchQuery,
    pub(crate) resource: SearchResource,
    _model: PhantomData<Model>,
}

impl<Model: Searchable> SearchClient<Model> {
    pub(crate) fn new<S: Into<String>>(client: DeezerClient, query: S) -> Self {
        SearchClient {
            client,
            query: SearchQuery::new(query),
            resource: Model::resource(),
            _model: Default::default(),
        }
    }

    pub fn strict(mut self) -> Self {
        self.query.strict = true;
        self
    }

    pub fn order(mut self, order: SearchOrder) -> Self {
        self.query.order = order;
        self
    }

    pub async fn send(self) -> Result<Vec<Model>, DeezerError> {
        let mut params = HashMap::new();
        params.insert("q".to_string(), self.query.query);
        if self.query.strict {
            params.insert("strict".to_string(), "on".to_string());
        }
        params.insert("order".to_string(), self.query.order.to_string());
        let url = self.resource.url();

        self.client.get_array_params(url, &params).await
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SearchResource {
    Albums,
    Artists,
    SearchHistory,
    Playlists,
    Podcasts,
    Radio,
    Tracks,
    Users,
}

impl SearchResource {
    fn url(&self) -> &'static str {
        match self {
            SearchResource::Albums => "search/album",
            SearchResource::Artists => "search/artist",
            SearchResource::SearchHistory => "search/history",
            SearchResource::Playlists => "search/playlist",
            SearchResource::Podcasts => "search/podcast",
            SearchResource::Radio => "search/radio",
            SearchResource::Tracks => "search/track",
            SearchResource::Users => "search/user",
        }
    }
}

pub trait Searchable: serde::de::DeserializeOwned {
    fn resource() -> SearchResource;
}

impl Default for SearchResource {
    fn default() -> Self {
        SearchResource::Tracks
    }
}

pub struct SearchQuery {
    pub query: String,
    pub strict: bool,
    pub order: SearchOrder,
}

impl SearchQuery {
    pub fn new<S: Into<String>>(query: S) -> Self {
        SearchQuery {
            query: query.into(),
            strict: false,
            order: Default::default(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SearchOrder {
    Ranking,
    TrackAsc,
    TrackDesc,
    ArtistAsc,
    ArtistDesc,
    AlbumAsc,
    AlbumDesc,
    RatingAsc,
    RatingDesc,
    DurationAsc,
    DurationDesc,
}

impl Display for SearchOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SearchOrder::Ranking => write!(f, "RANKING"),
            SearchOrder::TrackAsc => write!(f, "TRACK_ASC"),
            SearchOrder::TrackDesc => write!(f, "TRACK_DESC"),
            SearchOrder::ArtistAsc => write!(f, "ARTIST_ASC"),
            SearchOrder::ArtistDesc => write!(f, "ARTIST_DESC"),
            SearchOrder::AlbumAsc => write!(f, "ALBUM_ASC"),
            SearchOrder::AlbumDesc => write!(f, "ALBUM_DESC"),
            SearchOrder::RatingAsc => write!(f, "RATING_ASC"),
            SearchOrder::RatingDesc => write!(f, "RATING_DESC"),
            SearchOrder::DurationAsc => write!(f, "DURATION_ASC"),
            SearchOrder::DurationDesc => write!(f, "DURATION_DESC"),
        }
    }
}

impl Default for SearchOrder {
    fn default() -> Self {
        SearchOrder::Ranking
    }
}


