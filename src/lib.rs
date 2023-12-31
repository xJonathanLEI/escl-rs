use std::{fmt::Display, time::Duration};

use futures_util::{pin_mut, stream::StreamExt};
use mdns::RecordKind;
use reqwest::{Client, StatusCode};
use serde::de::DeserializeOwned;
pub use url::Url;

pub mod capabilities;
use capabilities::ScannerCapabilities;

pub mod status;
use status::ScannerStatus;

pub mod settings;
use settings::ScanSettings;

const SERVICE_NAME: &str = "_uscan._tcp.local";

#[derive(Debug)]
pub struct Scanner {
    base_url: Url,
    http_client: Client,
}

#[derive(Debug)]
pub enum Error {
    Http(reqwest::Error),
    Xml(serde_xml_rs::Error),
    UnexpectedStatusCode(StatusCode),
    LocationHeader,
}

#[derive(Debug)]
pub struct ScanJob {
    job_url: Url,
    http_client: Client,
}

#[derive(Debug)]
pub struct ScannerService {
    base_url: Url,
    name: String,
}

#[derive(Debug)]
pub enum DiscoverError {
    Mdns(mdns::Error),
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

    pub async fn scan(&self, settings: &ScanSettings) -> Result<ScanJob, Error> {
        let url = self.extended_url(&["ScanJobs"]);

        let request_body = serde_xml_rs::to_string(settings).map_err(Error::Xml)?;

        let response = self
            .http_client
            .post(url)
            .header("Content-Type", "text/xml")
            .body(request_body)
            .send()
            .await
            .map_err(Error::Http)?;

        let status_code = response.status();
        if status_code != StatusCode::CREATED {
            return Err(Error::UnexpectedStatusCode(status_code));
        }

        let location: Url = response
            .headers()
            .get("location")
            .ok_or(Error::LocationHeader)?
            .to_str()
            .map_err(|_| Error::LocationHeader)?
            .parse()
            .map_err(|_| Error::LocationHeader)?;

        Ok(ScanJob {
            job_url: location,
            http_client: self.http_client.clone(),
        })
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
            Error::UnexpectedStatusCode(code) => write!(f, "unexpected http status code {}", code),
            Error::LocationHeader => write!(f, "missing or invalid `Location` header in response"),
        }
    }
}

impl std::error::Error for Error {}

impl ScanJob {
    pub async fn next_document(&self) -> Result<Option<Vec<u8>>, Error> {
        let url = self.extended_url(&["NextDocument"]);

        let response = self
            .http_client
            .get(url)
            .send()
            .await
            .map_err(Error::Http)?;

        let status_code = response.status();
        if status_code == StatusCode::NOT_FOUND {
            return Ok(None);
        } else if status_code != StatusCode::OK {
            return Err(Error::UnexpectedStatusCode(status_code));
        }

        let bytes = response.bytes().await.map_err(Error::Http)?;
        Ok(Some(bytes.to_vec()))
    }

    fn extended_url(&self, segments: &[&'static str]) -> Url {
        let mut url = self.job_url.clone();
        url.path_segments_mut()
            .expect("Invalid base URL")
            .extend(segments);

        url
    }

    pub fn job_url(&self) -> &Url {
        &self.job_url
    }
}

impl ScannerService {
    /// Base URL that can be used to initialize a [Scanner] instance
    pub fn url(&self) -> &Url {
        &self.base_url
    }

    /// Human readable scanner make and model
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl From<&ScannerService> for Scanner {
    fn from(value: &ScannerService) -> Self {
        Self {
            base_url: value.base_url.clone(),
            http_client: Client::new(),
        }
    }
}

impl From<ScannerService> for Scanner {
    fn from(value: ScannerService) -> Self {
        Self {
            base_url: value.base_url,
            http_client: Client::new(),
        }
    }
}

impl Display for DiscoverError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mdns(err) => write!(f, "mDNS error: {}", err),
        }
    }
}

impl std::error::Error for DiscoverError {}

/// Looks for eSCL-enabled scanner devices in LAN. Up to a set timeout.
pub async fn discover(timeout: Duration) -> Result<Vec<ScannerService>, DiscoverError> {
    let mdns_stream = mdns::discover::all(SERVICE_NAME, timeout)
        .map_err(DiscoverError::Mdns)?
        .listen();
    pin_mut!(mdns_stream);

    let services = match mdns_stream.next().await {
        Some(Ok(response)) => {
            response
                .records()
                .filter_map(|record| {
                    if record.name == SERVICE_NAME {
                        match &record.kind {
                            RecordKind::PTR(ptr_record) => {
                                // Each PTR record on the service name represents one scanner

                                // There must be one TXT record with metadata
                                let txt_record = response.records().find_map(|record| {
                                    if &record.name == ptr_record {
                                        match &record.kind {
                                            RecordKind::TXT(txt) => Some(txt),
                                            _ => None,
                                        }
                                    } else {
                                        None
                                    }
                                })?;

                                // Extracts URL prefix
                                let rs = txt_record.iter().find_map(|item| {
                                    let (key, value) = item.split_once('=')?;

                                    if key == "rs" {
                                        Some(value)
                                    } else {
                                        None
                                    }
                                })?;

                                // Extracts human readable name
                                let ty = txt_record.iter().find_map(|item| {
                                    let (key, value) = item.split_once('=')?;

                                    if key == "ty" {
                                        Some(value)
                                    } else {
                                        None
                                    }
                                })?;

                                // There must be one SRV record pointing to the address
                                let (srv_record, port) = response.records().find_map(|record| {
                                    if &record.name == ptr_record {
                                        match &record.kind {
                                            RecordKind::SRV { target, port, .. } => {
                                                Some((target, port))
                                            }
                                            _ => None,
                                        }
                                    } else {
                                        None
                                    }
                                })?;

                                // There should be one A record with IP address
                                let ip_addr = response.records().find_map(|record| {
                                    if &record.name == srv_record {
                                        match &record.kind {
                                            RecordKind::A(ip_addr) => Some(ip_addr),
                                            _ => None,
                                        }
                                    } else {
                                        None
                                    }
                                })?;

                                let url =
                                    Url::parse(&format!("http://{}:{}/{}", ip_addr, port, rs))
                                        .ok()?;

                                Some(ScannerService {
                                    base_url: url,
                                    name: ty.to_owned(),
                                })
                            }
                            _ => None,
                        }
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        }
        Some(Err(err)) => return Err(DiscoverError::Mdns(err)),
        _ => {
            vec![]
        }
    };

    Ok(services)
}
