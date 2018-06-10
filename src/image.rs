use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug, PartialEq)]
pub struct PostImage {
    pub url: String,
    pub timestamp_uploaded: SystemTime,
}

impl PostImage {
    pub fn new(url: &str, timestamp: i32) -> PostImage {
        PostImage {
            url: url.to_string(),
            timestamp_uploaded: UNIX_EPOCH + Duration::from_secs(timestamp as u64),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProfileImage {
    pub url: String,
}
