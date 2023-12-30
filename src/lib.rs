use std::fmt::Display;

use reqwest::Client;

use serde::de::DeserializeOwned;
pub use url::Url;

pub mod capabilities;
use capabilities::ScannerCapabilities;

pub mod status;
use status::ScannerStatus;

#[derive(Debug)]
pub struct Scanner {
    base_url: Url,
    http_client: Client,
}

#[derive(Debug)]
pub enum Error {
    Http(reqwest::Error),
    Xml(serde_xml_rs::Error),
}

impl Scanner {
    /// Creates a new [Scanner] instance by supplying a base URL. Note that the base URL must
    /// include the `eSCL` segment if it exists.
    ///
    /// For example, if the full scanner status URL is `http://192.168.1.1/eSCL/ScannerStatus`,
    /// then the `base_url` value should be `http://192.168.1.1/eSCL`.
    ///
    /// ```
    /// use escl::{Scanner, Url};
    ///
    /// let scanner = Scanner::new(Url::parse("http://192.168.1.1/eSCL").unwrap());
    /// ```
    pub fn new(base_url: Url) -> Self {
        Self {
            base_url,
            http_client: Client::new(),
        }
    }

    pub async fn capabilities(&self) -> Result<ScannerCapabilities, Error> {
        self.send_get_request(self.extended_url(&["ScannerCapabilities"]))
            .await
    }

    pub async fn status(&self) -> Result<ScannerStatus, Error> {
        self.send_get_request(self.extended_url(&["ScannerStatus"]))
            .await
    }

    fn extended_url(&self, segments: &[&'static str]) -> Url {
        let mut url = self.base_url.clone();
        url.path_segments_mut()
            .expect("Invalid base URL")
            .extend(segments);

        url
    }

    async fn send_get_request<T>(&self, url: Url) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        let response = self
            .http_client
            .get(url)
            .send()
            .await
            .map_err(Error::Http)?;

        let response_body = response.text().await.map_err(Error::Http)?;

        serde_xml_rs::from_str(&response_body).map_err(Error::Xml)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Http(err) => write!(f, "http error: {}", err),
            Error::Xml(err) => write!(f, "xml error: {}", err),
        }
    }
}

impl std::error::Error for Error {}