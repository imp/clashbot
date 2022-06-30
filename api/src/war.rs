use super::*;

mod impls;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClanWarLog {
    pub items: Vec<ClanWarLogEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClanWarLogEntry {
    pub clan: WarClan,
    pub team_size: u64,
    #[serde(default)]
    pub attacks_per_member: u64,
    pub opponent: WarClan,
    pub end_time: String,
    pub result: Option<WarResult>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WarClan {
    pub destruction_percentage: f64,
    #[serde(flatten)]
    pub details: Option<WarClanDetails>,
    pub badge_urls: Urls,
    pub clan_level: u64,
    pub stars: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WarClanDetails {
    pub tag: String,
    pub name: String,
    #[serde(default)]
    pub attacks: u64,
    #[serde(default)]
    pub exp_earned: u64,
    #[serde(default)]
    pub members: Vec<ClanWarMember>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum WarResult {
    Lose,
    Win,
    Tie,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClanWarMember {
    pub tag: String,
    pub name: String,
    pub max_position: u64,
    pub townhall_level: u64,
    pub opponent_attacks: u64,
    pub best_opponent_attack: ClanWarAttack,
    pub attacks: Vec<ClanWarAttack>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClanWarAttack {
    pub order: u64,
    pub attacker_tag: String,
    pub defender_tag: String,
    pub stars: u64,
    pub destruction_percentage: f64,
    pub duration: u64,
}

#[cfg(test)]
mod tests;
