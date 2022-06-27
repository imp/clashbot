use std::collections::BTreeSet;

use clashofclans_api::Clan;
use clashofclans_api::Client;
use clashofclans_api::Player;

// use tracing::{event, Level};

use super::*;

#[derive(Debug)]
pub(crate) struct Bot {
    client: Client,
    store: Box<dyn Store>,
    players_queue: BTreeSet<String>,
    players_new: BTreeSet<String>,
    clans_queue: BTreeSet<String>,
    clans_new: BTreeSet<String>,
    players: Vec<Player>,
    clans: Vec<Clan>,
}

impl Bot {
    pub(crate) fn new<T>(token: impl ToString, store: T) -> Self
    where
        T: Store + 'static,
    {
        let store = Box::new(store);
        let client = Client::new(token);
        let players_queue = BTreeSet::new();
        let players_new = BTreeSet::new();
        let clans_queue = BTreeSet::new();
        let clans_new = BTreeSet::new();
        let players = Vec::new();
        let clans = Vec::new();
        Self {
            client,
            store,
            players_queue,
            players_new,
            clans_queue,
            clans_new,
            players,
            clans,
        }
    }

    pub(crate) fn seed_players(&mut self, seed: Vec<&str>) {
        for tag in seed {
            if let Some(player) = self.player(tag) {
                self.players_queue.insert(player.tag);
                if let Some(clan) = player.clan {
                    self.clans_queue.insert(clan.tag);
                }
            } else {
                self.players_queue.insert(tag.to_string());
            }
        }
    }

    pub(crate) fn seed_clans(&mut self, seed: Vec<&str>) {
        for tag in seed {
            if let Some(clan) = self.clan(tag) {
                self.clans_queue.insert(clan.tag);
            } else {
                self.clans_queue.insert(tag.to_string());
            }
        }
    }

    pub(crate) fn load(&mut self) {
        self.load_players();
        println!("Loaded {} unique players", self.players_queue.len());
        self.load_clans();
        println!("Loaded {} unique clans", self.clans_queue.len());
    }

    pub(crate) fn collect_new(&mut self) {
        self.collect_new_players();
        self.collect_new_clans();
    }

    pub(crate) fn update(&mut self) {
        self.update_players();
        self.update_clans();
    }

    pub(crate) fn checkpoint(&self) {
        self.save_players();
        self.save_clans();
    }

    fn load_players(&mut self) {
        self.players_queue = self.store.load_players();
    }

    fn load_clans(&mut self) {
        self.clans_queue = self.store.load_clans();
    }

    fn save_players(&self) {
        if !self.players.is_empty() {
            println!("Saving {} players", self.players.len());
            self.store.save_players(&self.players);
        }
    }

    fn save_clans(&self) {
        if !self.clans.is_empty() {
            println!("Saving {} clans", self.clans.len());
            self.store.save_clans(&self.clans);
        }
    }

    pub(crate) fn collect_new_players(&mut self) {
        let players = self
            .clans_queue
            .iter()
            .flat_map(|tag| self.players_from_clan(tag))
            .collect::<BTreeSet<_>>();
        self.players_new = players.difference(&self.players_queue).cloned().collect();
        println!("Discovered {} new players", self.players_new.len());
    }

    pub(crate) fn collect_new_clans(&mut self) {
        let clans = self
            .players_queue
            .iter()
            .flat_map(|tag| self.clans_from_player(tag))
            .collect::<BTreeSet<_>>();
        self.clans_new = clans.difference(&self.clans_queue).cloned().collect();
        println!("Discovered {} new clans", self.clans_new.len());
    }

    fn players_from_clan(&self, tag: &str) -> BTreeSet<String> {
        self.clan(tag)
            .map(|clan| clan.member_list)
            .unwrap_or_default()
            .into_iter()
            .map(|member| member.tag)
            .collect()
    }

    fn clans_from_player(&self, tag: &str) -> BTreeSet<String> {
        self.player(tag)
            .and_then(|player| player.clan)
            .map(|clan| self.warlog(&clan.tag))
            .unwrap_or_default()
    }

    fn update_players(&mut self) {
        let mut players = Vec::new();
        let mut failures = 0_u64;

        for tag in self.players_queue.iter().chain(self.players_new.iter()) {
            match self.client.player(tag) {
                Ok(player) => players.push(player),
                Err(e) => {
                    println!("Failed to load player {tag}: {e}");
                    failures += 1;
                }
            }
        }

        if failures > 0 {
            println!("Failed to load {} players", failures);
        }
        self.players = players;
        println!("Updated {} players", self.players.len());
    }

    fn update_clans(&mut self) {
        let mut clans = Vec::new();
        let mut failures = 0_u64;

        for tag in self.clans_queue.iter().chain(self.clans_new.iter()) {
            match self.client.clan(tag) {
                Ok(clan) => clans.push(clan),
                Err(e) => {
                    println!("Failed to load clan {tag}: {e}");
                    failures += 1;
                }
            }
        }

        if failures > 0 {
            println!("Failed to load {} clans", failures);
        }
        self.clans = clans;
        println!("Updated {} clans", self.clans.len());
    }

    fn player(&self, tag: &str) -> Option<Player> {
        self.client
            .player(tag)
            .map_err(|e| println!("Failed to load player {tag}: {e}"))
            .ok()
    }

    fn clan(&self, tag: &str) -> Option<Clan> {
        self.client
            .clan(tag)
            .map_err(|e| println!("Failed to load clan {tag}: {e}"))
            .ok()
    }

    fn warlog(&self, tag: &str) -> BTreeSet<String> {
        self.client
            .warlog(tag)
            .map_err(|e| println!("Failed to load warlog {tag}: {e}"))
            .map(|warlog| warlog.items)
            .unwrap_or_default()
            .into_iter()
            .filter_map(|war| war.opponent.details)
            .map(|details| details.tag)
            .collect()
    }
}
