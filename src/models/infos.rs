//! [Infos API](https://developers.deezer.com/api/infos)
#[warn(missing_docs)]
use serde::{Deserialize, Serialize};

/// Contains all the information about the API in the current country.
///
/// # Examples
///
/// ```rust
/// # use deezer::models::*;
/// # use deezer::{DeezerClient, DeezerError};
/// # #[tokio::main]
/// # async fn main() -> Result<(), DeezerError> {
/// let deezer = DeezerClient::new();
/// let info = deezer.api_info().await?;
/// # println!("{:?}", info);
/// # Ok(())
/// # }
///
/// ```
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Infos {
    /// The current country ISO code
    pub country_iso: String,

    /// The current country name
    pub country: String,

    /// Indicates if Deezer is open in the current country or not
    pub open: bool,

    /// An array of available offers in the current country
    pub offers: Vec<Offer>,
}

/// Contains all the information provided for an Offer.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Offer {
    pub id: u64,
    pub name: String,
    pub amount: String,
    pub currency: String,
    pub displayed_amount: String,
    pub tc: String,
    pub tc_html: String,
    pub tc_txt: String,
    pub try_and_buy: u64,
}
