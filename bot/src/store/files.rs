use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use time::macros::format_description;

use serde_json as json;

use super::*;

#[derive(Debug)]
pub(crate) struct Files {
    base: PathBuf,
}

impl Files {
    pub(crate) fn new(base: impl AsRef<str>) -> Self {
        let base = Path::new(&base.as_ref()).to_path_buf();
        std::fs::create_dir_all(&base).unwrap();
        Self { base }
    }

    fn load_all(&self, path: impl AsRef<Path>) -> io::Result<BTreeSet<String>> {
        let path = self.base.join(path);
        let items = fs::read_dir(path)?
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .filter_map(filename)
            .collect();

        Ok(items)
    }

    fn write_players(&self, players: &[&Timestamped<Player>]) -> io::Result<()> {
        let format = format_description!("[year][month][day][hour][minute][second]");

        for player in players {
            let path = self.base.join("players").join(&player.tag);
            fs::create_dir_all(&path)?;
            let contents = json::to_string_pretty(player.data())?;
            let timestamp = player.timestamp().format(&format).unwrap();
            fs::write(path.join(&timestamp), contents)?;
        }
        Ok(())
    }

    fn write_clans(&self, clans: &[&Timestamped<Clan>]) -> io::Result<()> {
        let format = format_description!("[year][month][day][hour][minute][second]");
        let now = time::OffsetDateTime::now_utc().format(&format).unwrap();

        for clan in clans {
            let path = self.base.join("clans").join(&clan.tag);
            fs::create_dir_all(&path)?;
            let contents = json::to_string_pretty(clan.data())?;
            fs::write(path.join(&now), contents)?;
        }
        Ok(())
    }
}

impl Store for Files {
    fn load_players(&self) -> BTreeSet<String> {
        self.load_all("players").unwrap()
    }

    fn load_clans(&self) -> BTreeSet<String> {
        self.load_all("clans").unwrap()
    }

    fn save_players(&self, players: &[&Timestamped<Player>]) {
        self.write_players(players).unwrap();
    }

    fn save_clans(&self, clans: &[&Timestamped<Clan>]) {
        self.write_clans(clans).unwrap();
    }
}

fn filename(path: PathBuf) -> Option<String> {
    path.file_name()
        .map(|name| name.to_string_lossy().to_string())
}
