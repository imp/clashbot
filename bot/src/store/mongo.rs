// use mongodb::bson::doc;
use mongodb::options;
use mongodb::sync;

use super::*;

const DB_NAME: &str = "v0";

#[derive(Debug)]
pub(crate) struct Mongo {
    mongo: sync::Client,
}

impl Mongo {
    pub(crate) fn new(mongodb: impl AsRef<str>) -> Self {
        let mongo = sync::Client::with_uri_str(mongodb).expect("Failed to connect to MongoDB");
        Self { mongo }
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
}

impl Store for Mongo {
    fn load_players(&self) -> BTreeSet<String> {
        self.players_collection()
            .distinct("tag", None, None)
            .unwrap()
            .into_iter()
            // .inspect(|tag| println!("{tag}"))
            .filter_map(|tag| tag.as_str().map(ToString::to_string))
            .collect()
    }

    fn load_clans(&self) -> BTreeSet<String> {
        self.clans_collection()
            .distinct("tag", None, None)
            .unwrap()
            .into_iter()
            // .inspect(|tag| println!("{tag}"))
            .filter_map(|tag| tag.as_str().map(ToString::to_string))
            .collect()
    }

    fn save_players(&self, players: &[Player]) {
        if !players.is_empty() {
            println!("Saving {} players", players.len());
            let collection = self.players_collection();
            let options = options::InsertManyOptions::builder().ordered(false).build();
            for chunk in players.chunks(50) {
                match collection.insert_many(chunk, options.clone()) {
                    Ok(_) => {}
                    Err(err) => println!("Failed to save players chunk: {err}"),
                }
            }
        }
    }

    fn save_clans(&self, clans: &[Clan]) {
        if !clans.is_empty() {
            println!("Saving {} clans", clans.len());
            let collection = self.clans_collection();
            let options = options::InsertManyOptions::builder().ordered(false).build();
            for chunk in clans.chunks(50) {
                match collection.insert_many(chunk, options.clone()) {
                    Ok(_) => {}
                    Err(err) => println!("Failed to save clans chunk: {err}"),
                }
            }
        }
    }
}
