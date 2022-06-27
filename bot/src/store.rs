use std::collections::BTreeSet;

use clashofclans_api::Clan;
use clashofclans_api::Player;

pub(crate) use mongo::Mongo;

mod files;
mod mongo;

pub(crate) trait Store {
    fn load_players(&self) -> BTreeSet<String>;

    fn load_clans(&self) -> BTreeSet<String>;

    fn save_players(&self, players: &[Player]);

    fn save_clans(&self, clans: &[Clan]);
}
