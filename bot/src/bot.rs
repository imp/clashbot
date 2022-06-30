use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet};

use clashofclans_api::Clan;
use clashofclans_api::Client;
use clashofclans_api::Player;

use indicatif::{ProgressBar, ProgressStyle};
// use tracing::{event, Level};

use super::*;

mod api;

#[derive(Debug)]
pub(crate) struct Bot {
    api: api::Api,
    store: Box<dyn Store>,
    players_queue: BTreeSet<String>,
    players_new: BTreeSet<String>,
    clans_queue: BTreeSet<String>,
    clans_new: BTreeSet<String>,
}

impl Bot {
    pub(crate) fn new<T>(token: impl ToString, store: T) -> Self
    where
        T: Store + 'static,
    {
        let api = api::Api::new(token);
        let store = Box::new(store);
        let players_queue = BTreeSet::new();
        let players_new = BTreeSet::new();
        let clans_queue = BTreeSet::new();
        let clans_new = BTreeSet::new();

        Self {
            api,
            store,
            players_queue,
            players_new,
            clans_queue,
            clans_new,
        }
    }

    pub(crate) fn seed_players(&mut self, seed: Vec<&str>) {
        for tag in seed {
            if let Some(player) = self.api.player(tag) {
                self.players_queue.insert(player.tag.clone());
                if let Some(ref clan) = player.clan {
                    self.clans_queue.insert(clan.tag.clone());
                }
            } else {
                self.players_queue.insert(tag.to_string());
            }
        }
    }

    pub(crate) fn seed_clans(&mut self, seed: Vec<&str>) {
        for tag in seed {
            if let Some(clan) = self.api.clan(tag) {
                self.clans_queue.insert(clan.tag.clone());
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
        let players = self.api.all_players();
        let players = players.values().collect::<Vec<_>>();

        if !players.is_empty() {
            // let progress = progress_bar("Saving players", players.len());
            println!("Saving {} players", players.len());
            self.store.save_players(&players);
            // progress.finish();
        }
    }

    fn save_clans(&self) {
        let clans = self.api.all_clans();
        let clans = clans.values().collect::<Vec<_>>();
        if !clans.is_empty() {
            println!("Saving {} clans", clans.len());
            self.store.save_clans(&clans);
        }
    }

    pub(crate) fn collect_new_players(&mut self) {
        let progress = progress_bar("Checking clan", self.clans_queue.len());
        let players = self
            .clans_queue
            .iter()
            // .inspect(|tag| progress_update(&progress, tag))
            .filter_map(|tag| self.players_from_clan(tag, &progress))
            .flatten()
            .collect::<BTreeSet<_>>();
        self.players_new = players.difference(&self.players_queue).cloned().collect();
        progress.finish_with_message(format!("{} new players", self.players_new.len()));
    }

    pub(crate) fn collect_new_clans(&mut self) {
        let progress = progress_bar("Checking player", self.players_queue.len());
        let clans = self
            .players_queue
            .iter()
            // .inspect(|tag| progress_update(&progress, tag))
            .filter_map(|tag| self.clans_from_player(tag, &progress))
            .flatten()
            .collect::<BTreeSet<_>>();
        self.clans_new = clans.difference(&self.clans_queue).cloned().collect();
        progress.finish_with_message(format!("{} new clans", self.clans_new.len()));
    }

    fn update_players(&mut self) {
        let progress = progress_bar(
            "Update player",
            self.players_queue.len() + self.players_new.len(),
        );
        let mut failures = 0_u64;

        for tag in self.players_queue.iter().chain(self.players_new.iter()) {
            if let Some(player) = self.api.player(tag) {
                progress_update(&progress, format!("{} ({})", player.name, player.tag));
            } else {
                progress_update(&progress, tag.clone());
                failures += 1;
            }
        }
        progress.finish();

        if failures > 0 {
            println!("Failed to load {} players", failures);
        }
    }

    fn update_clans(&mut self) {
        let progress = progress_bar("Update clan", self.clans_queue.len() + self.clans_new.len());
        let mut failures = 0_u64;

        for tag in self.clans_queue.iter().chain(self.clans_new.iter()) {
            if let Some(clan) = self.api.clan(tag) {
                progress_update(&progress, format!("{} ({})", clan.name, clan.tag));
            } else {
                progress_update(&progress, tag.clone());
                failures += 1;
            }
        }
        progress.finish();

        if failures > 0 {
            println!("Failed to load {} clans", failures);
        }
    }

    fn players_from_clan(&self, tag: &str, progress: &ProgressBar) -> Option<BTreeSet<String>> {
        let clan = self.api.clan(tag)?;
        progress_update(progress, format!("{} ({})", clan.name, clan.tag));
        let players = clan
            .member_list
            .iter()
            .map(|member| member.tag.clone())
            .collect();
        Some(players)
    }

    fn clans_from_player(&self, tag: &str, progress: &ProgressBar) -> Option<BTreeSet<String>> {
        let player = self.api.player(tag)?;
        progress_update(progress, format!("{} ({})", player.name, player.tag));
        let tag = player.clan.as_ref()?.tag.clone();
        self.api.war_opponents(&tag)
    }
}

fn progress_style() -> ProgressStyle {
    ProgressStyle::default_bar()
        .template("{prefix:15}: {msg:32} {bar:60} {pos}/{len} [{elapsed} ETA: {eta}]")
        .unwrap()
}

fn progress_bar(prefix: &str, len: usize) -> ProgressBar {
    ProgressBar::new(len as u64)
        .with_style(progress_style())
        .with_prefix(prefix.to_string())
}

fn progress_update(pb: &ProgressBar, msg: impl Into<Cow<'static, str>>) {
    pb.set_message(msg);
    pb.inc(1)
}
