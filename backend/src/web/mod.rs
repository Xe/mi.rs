pub mod discord_webhook;
pub mod mastodon;
pub mod pluralkit;
pub mod switchcounter;
pub mod twitter;

pub type Result<T = ()> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("serde error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("ureq error: {0}")]
    UReq(String),

    #[error("http unsuccessful: {0}")]
    HttpStatus(u16),

    #[error("futures io error: {0}")]
    FuturesIO(#[from] futures_io::Error),

    #[error("systemmate mapping not found")]
    SystemmateMappingNotFound(String),
}
