use std::cell::{Ref, RefCell};

use clashofclans_api::ClanWarLog;
// use clashofclans_api::WarClanDetails;

use super::*;

#[derive(Debug)]
pub(super) struct Api {
    client: Client,
    players: RefCell<BTreeMap<String, Timestamped<Player>>>,
    clans: RefCell<BTreeMap<String, Timestamped<Clan>>>,
    warlogs: RefCell<BTreeMap<String, Timestamped<ClanWarLog>>>,
}

impl Api {
    pub(super) fn new(token: impl ToString) -> Self {
        let client = Client::new(token);
        let players = RefCell::new(BTreeMap::new());
        let clans = RefCell::new(BTreeMap::new());
        let warlogs = RefCell::new(BTreeMap::new());

        Self {
            client,
            players,
            clans,
            warlogs,
        }
    }
    pub(super) fn all_players(&self) -> Ref<'_, BTreeMap<String, Timestamped<Player>>> {
        self.players.borrow()
    }

    pub(super) fn all_clans(&self) -> Ref<'_, BTreeMap<String, Timestamped<Clan>>> {
        self.clans.borrow()
    }

    pub(super) fn player(&self, tag: &str) -> Option<Ref<'_, Timestamped<Player>>> {
        if !self.players.borrow().contains_key(tag) {
            if let Some(player) = self.get_player(tag) {
                self.players
                    .borrow_mut()
                    .insert(player.tag.clone(), player.into());
            }
        }
        let players = self.players.borrow();
        Ref::filter_map(players, |players| players.get(tag)).ok()
    }

    pub(super) fn clan(&self, tag: &str) -> Option<Ref<'_, Timestamped<Clan>>> {
        if !self.clans.borrow().contains_key(tag) {
            if let Some(clan) = self.get_clan(tag) {
                self.clans
                    .borrow_mut()
                    .insert(clan.tag.clone(), clan.into());
            }
        }
        let clans = self.clans.borrow();
        Ref::filter_map(clans, |clans| clans.get(tag)).ok()
    }

    pub(super) fn war_opponents(&self, tag: &str) -> Option<BTreeSet<String>> {
        if !self.clan(tag).map_or(true, |clan| clan.is_war_log_public) {
            return None;
        }

        if !self.warlogs.borrow().contains_key(tag) {
            if let Some(warlog) = self.get_warlog(tag) {
                self.warlogs
                    .borrow_mut()
                    .insert(tag.to_string(), warlog.into());
            }
        }

        let opponents = self
            .warlogs
            .borrow()
            .get(tag)?
            .opponents()
            .map(|details| details.tag.clone())
            .collect();
        Some(opponents)
    }

    fn get_player(&self, tag: &str) -> Option<Player> {
        self.client
            .player(tag)
            .map_err(|e| println!("Failed to load player {tag}: {e}"))
            .ok()
    }

    fn get_clan(&self, tag: &str) -> Option<Clan> {
        self.client
            .clan(tag)
            .map_err(|e| println!("Failed to load clan {tag}: {e}"))
            .ok()
    }

    fn get_warlog(&self, tag: &str) -> Option<ClanWarLog> {
        self.client
            .clan_warlog(tag)
            .map_err(|e| {
                if !e.is_access_denied() {
                    println!("Failed to load warlog {tag}: {e}")
                }
            })
            .ok()
    }
}
