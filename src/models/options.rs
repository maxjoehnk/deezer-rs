//! [Options API](https://developers.deezer.com/api/options)
#[warn(missing_docs)]
use serde::{Deserialize, Serialize};

/// Contains all the information provided for a user's Options.
///
/// # Examples
///
/// ```rust
/// # use deezer::models::*;
/// # use deezer::{DeezerClient, DeezerError};
/// # #[tokio::main]
/// # async fn main() -> Result<(), DeezerError> {
/// let deezer = DeezerClient::new();
/// let options = deezer.user_options().await?;
/// # println!("{:?}", options);
/// # Ok(())
/// # }
///
/// ```
#[derive(Deserialize, Serialize, Debug)]
pub struct Options {
    /// If the user can stream on the platform
    pub streaming: bool,

    /// the streaming duration of the user (in seconds?)
    pub streaming_duration: u64,

    /// The user can listen to the music in offline mode
    pub offline: bool,

    /// The HQ can be activated
    pub hq: bool,

    /// Displays ads
    pub ads_display: bool,

    /// Activates audio ads
    pub ads_audio: bool,

    /// If the user reached the limit of linked devices
    #[serde(rename = "too_many_devices")]
    pub has_too_many_devices: bool,

    /// If the user can subscribe to the service
    pub can_subscribe: bool,

    /// The limit of radio skips. 0 = no limit
    pub radio_skips: u64,

    /// Lossless is available
    pub lossless: bool,

    /// Allows to display the preview of the tracks
    pub preview: bool,

    /// Allows to stream the radio
    pub radio: bool,
}
