use std::collections::BTreeSet;
use std::fmt;

use clashofclans_api::Clan;
use clashofclans_api::Player;

use super::*;

pub(crate) use files::Files;
pub(crate) use mongo::Mongo;

mod files;
mod mongo;

pub(crate) trait Store: fmt::Debug {
    fn load_players(&self) -> BTreeSet<String>;

    fn load_clans(&self) -> BTreeSet<String>;

    fn save_players(&self, players: &[&Timestamped<Player>]);

    fn save_clans(&self, clans: &[&Timestamped<Clan>]);
}
