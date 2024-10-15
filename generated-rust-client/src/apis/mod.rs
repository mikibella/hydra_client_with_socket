use std::fmt;
use std::fmt::Debug;

use hyper;
use hyper::http;
use hyper_util::client::legacy::connect::Connect;
use serde_json;

#[derive(Debug)]
pub enum Error {
    Api(ApiError),
    Header(http::header::InvalidHeaderValue),
    Http(http::Error),
    Hyper(hyper::Error),
    HyperClient(hyper_util::client::legacy::Error),
    Serde(serde_json::Error),
    UriError(http::uri::InvalidUri),
}

pub struct ApiError {
    pub code: hyper::StatusCode,
    pub body: hyper::body::Incoming,
}

impl Debug for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ApiError")
         .field("code", &self.code)
         .field("body", &"hyper::body::Incoming")
         .finish()
    }
}

impl From<(hyper::StatusCode, hyper::body::Incoming)> for Error {
    fn from(e: (hyper::StatusCode, hyper::body::Incoming)) -> Self {
        Error::Api(ApiError {
            code: e.0,
            body: e.1,
        })
    }
}

impl From<http::Error> for Error {
    fn from(e: http::Error) -> Self {
        Error::Http(e)
    }
}

impl From<hyper_util::client::legacy::Error> for Error {
    fn from(e: hyper_util::client::legacy::Error) -> Self {
        Error::HyperClient(e)
    }
}

impl From<hyper::Error> for Error {
    fn from(e: hyper::Error) -> Self {
        Error::Hyper(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

mod request;

mod jwk_api;
pub use self::jwk_api::{ JwkApi, JwkApiClient };
mod metadata_api;
pub use self::metadata_api::{ MetadataApi, MetadataApiClient };
mod o_auth2_api;
pub use self::o_auth2_api::{ OAuth2Api, OAuth2ApiClient };
mod oidc_api;
pub use self::oidc_api::{ OidcApi, OidcApiClient };
mod wellknown_api;
pub use self::wellknown_api::{ WellknownApi, WellknownApiClient };

pub mod configuration;
pub mod client;
