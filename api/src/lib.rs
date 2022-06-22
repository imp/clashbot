#![cfg_attr(feature = "pedantic", warn(clippy::pedantic))]
// #![warn(clippy::use_self)]
#![warn(clippy::map_flatten)]
#![warn(clippy::map_unwrap_or)]
#![warn(deprecated_in_future)]
#![warn(future_incompatible)]
#![warn(noop_method_call)]
#![warn(unreachable_pub)]
#![warn(missing_debug_implementations)]
#![warn(rust_2018_compatibility)]
#![warn(rust_2021_compatibility)]
#![warn(rust_2018_idioms)]
#![warn(unused)]
#![deny(warnings)]

use serde::{Deserialize, Serialize};

pub use client::Client;

pub use clan::Clan;
pub use error::ClientError;
pub use player::League;
pub use player::Player;
pub use player::PlayerClan;
pub use player::Role;

pub mod client;

pub mod clan;
pub mod error;
pub mod player;
pub mod war;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JsonLocalizedName(Option<String>);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Label {
    pub name: JsonLocalizedName,
    pub id: u64,
    pub icon_urls: Urls,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Urls {
    pub large: Option<String>,
    pub medium: Option<String>,
    pub small: Option<String>,
    pub tiny: Option<String>,
}
