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
pub use error::ClientError;
pub use player::Player;
pub use player::PlayerClan;

pub mod client;
pub mod error;
pub mod player;
