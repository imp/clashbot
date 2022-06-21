#![cfg_attr(feature = "pedantic", warn(clippy::pedantic))]
#![warn(clippy::use_self)]
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

use std::env;

use clashofclans_api::Client;

fn main() -> anyhow::Result<()> {
    let token = env::var("COC_TOKEN")?;
    let client = Client::new(token);
    // let player = client.player("%23LVGV0CJC")?;
    let player = client.player("#LVGV0CJC")?;
    println!("{player:#?}");
    Ok(())
}
