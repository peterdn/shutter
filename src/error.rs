use reqwest;
use serde_json;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "A network error caused the request to fail")]
    NetworkError,

    #[fail(display = "HTTP request failed ({})", request_error)]
    HttpRequestError {
        #[cause]
        request_error: reqwest::Error,
    },

    #[fail(display = "User `{}` was not found", username)]
    UserNotFound { username: String },

    #[fail(display = "Error retrieving response body")]
    ResponseBodyError,

    #[fail(display = "Failed to find profile data")]
    ProfileDataNotFound,

    #[fail(display = "Failed to decode profile data")]
    ProfileDataDecodeFailed,

    #[fail(display = "Failed to parse profile json")]
    ProfileJsonParseError,

    #[fail(display = "Profile json data is invalid")]
    ProfileJsonInvalid,
}

pub type Result<T> = ::std::result::Result<T, Error>;

impl From<serde_json::Error> for Error {
    fn from(_err: serde_json::Error) -> Error {
        Error::ProfileJsonParseError
    }
}

impl From<reqwest::Error> for Error {
    fn from(_err: reqwest::Error) -> Error {
        Error::NetworkError
    }
}
