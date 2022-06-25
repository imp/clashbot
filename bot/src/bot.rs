use std::collections::BTreeSet;

use clashofclans_api::Clan;
use clashofclans_api::Client;
use clashofclans_api::Player;

// use mongodb::bson::doc;
use mongodb::options;
use mongodb::sync;
// use serde::{Deserialize, Serialize};

// use tracing::{event, Level};

const DB_NAME: &str = "v0";

#[derive(Debug)]
pub(crate) struct Bot {
    client: Client,
    mongo: sync::Client,
    players_queue: BTreeSet<String>,
    players_new: BTreeSet<String>,
    clans_queue: BTreeSet<String>,
    clans_new: BTreeSet<String>,
    players: Vec<Player>,
    clans: Vec<Clan>,
}

impl Bot {
    pub(crate) fn new(token: impl ToString, mongodb: impl AsRef<str>) -> Self {
        let client = Client::new(token);
        let mongo = sync::Client::with_uri_str(mongodb).expect("Failed to connect to MongoDB");
        let players_queue = BTreeSet::new();
        let players_new = BTreeSet::new();
        let clans_queue = BTreeSet::new();
        let clans_new = BTreeSet::new();
        let players = Vec::new();
        let clans = Vec::new();
        Self {
            client,
            mongo,
            players_queue,
            players_new,
            clans_queue,
            clans_new,
            players,
            clans,
        }
    }

    pub(crate) fn seed(&mut self, seed: &str) {
        if let Ok(player) = self.client.player(seed) {
            self.clans_queue.insert(player.clan.tag);
            self.players_queue.insert(player.tag);
        } else {
            self.players_queue.insert(seed.to_string());
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

    fn mongodb(&self) -> sync::Database {
        self.mongo.database(DB_NAME)
    }

    fn players_collection(&self) -> sync::Collection<Player> {
        self.mongodb().collection("players")
    }

    fn clans_collection(&self) -> sync::Collection<Clan> {
        self.mongodb().collection("clans")
    }

    fn load_players(&mut self) {
        let players = self.players_collection();
        self.players_queue = players
            .distinct("tag", None, None)
            .unwrap()
            .into_iter()
            // .inspect(|tag| println!("{tag}"))
            .filter_map(|tag| tag.as_str().map(ToString::to_string))
            .collect();
    }

    fn load_clans(&mut self) {
        let clans = self.clans_collection();
        self.clans_queue = clans
            .distinct("tag", None, None)
            .unwrap()
            .into_iter()
            // .inspect(|tag| println!("{tag}"))
            .filter_map(|tag| tag.as_str().map(ToString::to_string))
            .collect();
    }

    fn save_players(&self) {
        if !self.players.is_empty() {
            println!("Saving {} players", self.players.len());
            let players = self.players_collection();
            let options = options::InsertManyOptions::builder().ordered(false).build();
            players.insert_many(&self.players, options).unwrap();
        }
    }

    fn save_clans(&self) {
        if !self.clans.is_empty() {
            println!("Saving {} clans", self.clans.len());
            let clans = self.clans_collection();
            let options = options::InsertManyOptions::builder().ordered(false).build();
            clans.insert_many(&self.clans, options).unwrap();
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
        self.client
            .clan(tag)
            .map(|clan| clan.member_list)
            .unwrap_or_default()
            .into_iter()
            .map(|member| member.tag)
            .collect()
    }

    fn clans_from_player(&self, tag: &str) -> BTreeSet<String> {
        self.client
            .player(tag)
            .and_then(|player| self.client.clan(player.clan.tag))
            .and_then(|clan| self.client.warlog(clan.tag))
            .map(|warlog| warlog.items)
            .unwrap_or_default()
            .into_iter()
            .filter_map(|war| war.opponent.details)
            .map(|details| details.tag)
            .collect()
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
}
