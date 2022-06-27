use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time;

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

    fn latest(&self) -> io::Result<PathBuf> {
        fs::read_dir(&self.base)?
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .max()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "No latest directory found"))
    }

    fn read_latest(&self, path: impl AsRef<Path>) -> io::Result<impl Iterator<Item = PathBuf>> {
        let latest = self.latest().map(|latest| latest.join(path))?;
        let names = fs::read_dir(latest)?
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path());
        Ok(names)
    }

    fn create_now(&self) -> io::Result<PathBuf> {
        let now = time::SystemTime::now()
            .duration_since(time::UNIX_EPOCH)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
            .as_secs();
        let path = self.base.join(format!("{now}"));
        fs::create_dir_all(&path)?;
        Ok(path)
    }

    fn write_players(&self, players: &[Player]) -> io::Result<()> {
        let path = self.create_now()?.join("players");
        fs::create_dir_all(&path)?;
        for player in players {
            let contents = json::to_string_pretty(player)?;
            fs::write(path.join(&player.tag), contents).unwrap();
        }
        Ok(())
    }

    fn write_clans(&self, clans: &[Clan]) -> io::Result<()> {
        let path = self.create_now()?.join("clans");
        fs::create_dir_all(&path)?;
        for clan in clans {
            let contents = json::to_string_pretty(clan)?;
            fs::write(path.join(&clan.tag), contents).unwrap();
        }
        Ok(())
    }
}

impl Store for Files {
    fn load_players(&self) -> BTreeSet<String> {
        self.read_latest("players")
            .unwrap()
            .map(|path| path.display().to_string())
            .collect()
    }

    fn load_clans(&self) -> BTreeSet<String> {
        self.read_latest("clans")
            .unwrap()
            .map(|path| path.display().to_string())
            .collect()
    }

    fn save_players(&self, players: &[Player]) {
        self.write_players(players).unwrap();
    }

    fn save_clans(&self, clans: &[Clan]) {
        self.write_clans(clans).unwrap();
    }
}
