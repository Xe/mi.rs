pub const APPLICATION_NAME: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " +https://mi.within.website/.within/botinfo"
);

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
