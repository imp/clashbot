use super::*;

impl ClanWarLog {
    pub fn opponents(&self) -> impl Iterator<Item = &WarClanDetails> {
        self.items
            .iter()
            .filter_map(|entry| entry.opponent.details.as_ref())
    }
}
