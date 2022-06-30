use clashofclans_api::ClanWarLog;

use super::*;

#[derive(Debug)]
pub(super) struct Api {
    client: Client,
    players: BTreeMap<String, Player>,
    clans: BTreeMap<String, Clan>,
    warlogs: BTreeMap<String, ClanWarLog>,
}

impl Api {
    pub(super) fn new(token: impl ToString) -> Self {
        let client = Client::new(token);
        let players = BTreeMap::new();
        let clans = BTreeMap::new();
        let warlogs = BTreeMap::new();

        Self {
            client,
            players,
            clans,
            warlogs,
        }
    }
    pub(super) fn all_players(&self) -> &BTreeMap<String, Player> {
        &self.players
    }

    pub(super) fn all_clans(&self) -> &BTreeMap<String, Clan> {
        &self.clans
    }

    pub(super) fn players_from_clan(&mut self, tag: &str) -> Option<BTreeSet<String>> {
        let players = self
            .clan(tag)?
            .member_list
            .iter()
            .map(|member| member.tag.clone())
            .collect();
        Some(players)
    }

    pub(super) fn clans_from_player(&mut self, tag: &str) -> Option<BTreeSet<String>> {
        let tag = self.player(tag)?.clan.as_ref()?.tag.clone();
        self.war_opponents(&tag)
    }

    pub(super) fn player(&mut self, tag: &str) -> Option<&Player> {
        if !self.players.contains_key(tag) {
            if let Some(player) = self.get_player(tag) {
                self.players.insert(player.tag.clone(), player);
            }
        }
        self.players.get(tag)
    }

    pub(super) fn clan(&mut self, tag: &str) -> Option<&Clan> {
        if !self.clans.contains_key(tag) {
            if let Some(clan) = self.get_clan(tag) {
                self.clans.insert(clan.tag.clone(), clan);
            }
        }
        self.clans.get(tag)
    }

    fn war_opponents(&mut self, tag: &str) -> Option<BTreeSet<String>> {
        if !self.warlogs.contains_key(tag) {
            if let Some(warlog) = self.get_warlog(tag) {
                self.warlogs.insert(tag.to_string(), warlog);
            }
        }

        let opponents = self
            .warlogs
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
