use std::convert::TryInto;
use std::io;

use diesel::result::Error as SQLError;
use hex::FromHexError as HexError;
use io::Error as IOError;
use json::Error as JSONError;
use semver::{SemVerError as SemverError, Version};
use thiserror::Error;
use toml::de::Error as TOMLError;

use tide::response::IntoResponse;
use tide::Response;

use crate::db::models::Author;
use crate::utils;
use http::StatusCode;

/// The Error type for the registry.
///
/// It can represent any kind of error the registry might encounter.
#[derive(Error, Debug)]
pub enum Error {
    /// An IO error (file not found, access forbidden, etc...).
    #[error("IO error: {0}")]
    IOError(#[source] IOError),
    /// JSON (de)serialization error (invalid JSON parsed, etc...).
    #[error("JSON error: {0}")]
    JSONError(#[source] JSONError),
    /// TOML (de)serialization error (invalid TOML parsed, etc...).
    #[error("TOML error: {0}")]
    TOMLError(#[source] TOMLError),
    /// SQL error (invalid queries, database disconnections, etc...).
    #[error("SQL error: {0}")]
    SQLError(#[source] SQLError),
    /// Version parsing errors (invalid version format parsed, etc...).
    #[error("Semver error: {0}")]
    SemverError(#[source] SemverError),
    /// Hexadecimal decoding errors (odd length, etc...).
    #[error("Hex error: {0}")]
    HexError(#[source] HexError),
    /// Alexandrie's custom errors (crate not found, invalid token, etc...).
    #[error("Alexandrie error: {0}")]
    AlexError(#[source] AlexError),
}

/// The Error type for Alexandrie's own errors.
#[derive(Error, Debug)]
pub enum AlexError {
    /// The requested crate cannot be found.
    #[error("no crate named '{name}' found")]
    CrateNotFound {
        /// The requested crate's name.
        name: String,
    },
    /// The crate is not owned by the user.
    #[error("you are not an owner of '{name}'")]
    CrateNotOwned {
        /// The involved crate's name.
        name: String,
        /// The involved author.
        author: Author,
    },
    /// The published crate version is lower than the current hosted version.
    #[error("the published version is too low (hosted version is {hosted}, and thus {published} <= {hosted})")]
    VersionTooLow {
        /// The krate's name.
        krate: String,
        /// The available hosted version.
        hosted: Version,
        /// The proposed version to be published.
        published: Version,
    },
    /// The token used to access the registry is invalid.
    #[error("invalid token")]
    InvalidToken,
    /// The request is invalid because of a required query parameter.
    #[error("missing query parameters: {missing_params:?}")]
    MissingQueryParams {
        /// The list of missing query parameters.
        missing_params: &'static [&'static str],
    },
}

impl AlexError {
    /// Function to map `AlexError` to an appropriate HTTP error code
    pub fn get_http_status_code(&self) -> StatusCode {
        match self {
            AlexError::CrateNotFound { .. } => http::StatusCode::BAD_REQUEST,
            AlexError::CrateNotOwned { .. } => http::StatusCode::BAD_REQUEST,
            AlexError::VersionTooLow { .. } => http::StatusCode::BAD_REQUEST,
            AlexError::InvalidToken => http::StatusCode::UNAUTHORIZED,
            AlexError::MissingQueryParams { .. } => http::StatusCode::BAD_REQUEST,
        }
    }
}
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        error!("constructing error response: {0}", self);
        let message = match &self {
            Error::IOError(_) => "internal server error".to_string(),
            Error::JSONError(_) => "internal server error".to_string(),
            Error::TOMLError(_) => "internal server error".to_string(),
            Error::SQLError(_) => "internal server error".to_string(),
            Error::SemverError(_) => "internal server error".to_string(),
            Error::HexError(_) => "internal server error".to_string(),
            Error::AlexError(err) => err.to_string(),
        };
        let status_code = match self {
            Error::AlexError(err) => err.get_http_status_code(),
            _ => http::StatusCode::INTERNAL_SERVER_ERROR
        };


        utils::response::error(status_code, message)
    }
}

impl From<IOError> for Error {
    fn from(err: IOError) -> Error {
        Error::IOError(err)
    }
}

impl From<JSONError> for Error {
    fn from(err: JSONError) -> Error {
        Error::JSONError(err)
    }
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Error {
        Error::TOMLError(err)
    }
}

impl From<SQLError> for Error {
    fn from(err: SQLError) -> Error {
        Error::SQLError(err)
    }
}

impl From<SemverError> for Error {
    fn from(err: SemverError) -> Error {
        Error::SemverError(err)
    }
}

impl From<HexError> for Error {
    fn from(err: HexError) -> Error {
        Error::HexError(err)
    }
}

impl From<AlexError> for Error {
    fn from(err: AlexError) -> Error {
        Error::AlexError(err)
    }
}

impl TryInto<IOError> for Error {
    type Error = ();

    fn try_into(self) -> Result<IOError, Self::Error> {
        match self {
            Error::IOError(err) => Ok(err),
            _ => Err(()),
        }
    }
}

impl TryInto<JSONError> for Error {
    type Error = ();

    fn try_into(self) -> Result<JSONError, Self::Error> {
        match self {
            Error::JSONError(err) => Ok(err),
            _ => Err(()),
        }
    }
}

impl TryInto<TOMLError> for Error {
    type Error = ();

    fn try_into(self) -> Result<TOMLError, Self::Error> {
        match self {
            Error::TOMLError(err) => Ok(err),
            _ => Err(()),
        }
    }
}

impl TryInto<SQLError> for Error {
    type Error = ();

    fn try_into(self) -> Result<SQLError, Self::Error> {
        match self {
            Error::SQLError(err) => Ok(err),
            _ => Err(()),
        }
    }
}

impl TryInto<SemverError> for Error {
    type Error = ();

    fn try_into(self) -> Result<SemverError, Self::Error> {
        match self {
            Error::SemverError(err) => Ok(err),
            _ => Err(()),
        }
    }
}

impl TryInto<AlexError> for Error {
    type Error = ();

    fn try_into(self) -> Result<AlexError, Self::Error> {
        match self {
            Error::AlexError(err) => Ok(err),
            _ => Err(()),
        }
    }
}
