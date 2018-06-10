use reqwest;

#[derive(Debug, Fail, PartialEq)]
pub enum ScrapeError {
    #[fail(display = "A network error caused the request to fail")]
    NetworkError,

    #[fail(display = "HTTP request failed (status code: {})", status_code)]
    HttpRequestError { status_code: reqwest::StatusCode },

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

pub type Result<T> = ::std::result::Result<T, ScrapeError>;
