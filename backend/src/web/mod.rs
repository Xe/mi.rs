pub mod bridgy;
pub mod canada_weather;
pub mod discord_webhook;
pub mod lemmy;
pub mod mastodon;
pub mod orange_connex;
pub mod pluralkit;
pub mod switchcounter;
pub mod twitter;

pub use discord_webhook::Client as DiscordWebhook;
pub use lemmy::Client as Lemmy;
pub use mastodon::Client as Mastodon;
pub use pluralkit::Client as PluralKit;
pub use switchcounter::Client as SwitchCounter;
pub use twitter::Client as Twitter;

pub type Result<T = ()> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("serde error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("ureq error: {0}")]
    UReq(#[from] ureq::Error),

    #[error("old ureq error: {0}")]
    OldUReq(String),

    #[error("http unsuccessful: {0}")]
    HttpStatus(u16),

    #[error("futures io error: {0}")]
    FuturesIO(#[from] futures_io::Error),

    #[error("systemmate mapping not found")]
    SystemmateMappingNotFound(String),
}
