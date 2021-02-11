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

pub use mi_web::{Error, Result};
pub use ponepoints::{self, Client as PonePoints};
