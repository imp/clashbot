use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub clan: Option<PlayerClan>,
    pub league: Option<League>,
    pub attack_wins: u64,
    #[serde(default)]
    pub town_hall_weapon_level: u64,
    pub versus_battle_wins: u64,
    pub legend_statistics: Option<PlayerLegendStatistics>,
    pub heroes: Vec<PlayerItemLevel>,
    pub spells: Vec<PlayerItemLevel>,
    pub role: Option<Role>,
    pub war_preference: Option<WarPreference>,
    pub town_hall_level: u64,
    pub troops: Vec<PlayerItemLevel>,
    pub defense_wins: u64,
    pub labels: Vec<Label>,
    pub tag: String,
    pub name: String,
    pub exp_level: u64,
    pub trophies: u64,
    pub best_trophies: u64,
    pub donations: u64,
    pub donations_received: u64,
    #[serde(default)]
    pub builder_hall_level: u64,
    pub versus_trophies: u64,
    pub best_versus_trophies: u64,
    pub war_stars: u64,
    pub achievements: Vec<PlayerAchievementProgress>,
    pub versus_battle_win_count: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerClan {
    pub tag: String,
    pub clan_level: u64,
    pub name: String,
    pub badge_urls: Urls,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct League {
    pub name: JsonLocalizedName,
    pub id: u64,
    pub icon_urls: Urls,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct PlayerLegendStatistics {
    pub best_season: LegendLeagueTournamentSeasonResult,
    pub legend_trophies: u64,
    pub current_season: LegendLeagueTournamentSeasonResult,
    pub previous_season: LegendLeagueTournamentSeasonResult,
    pub best_versus_season: LegendLeagueTournamentSeasonResult,
    pub previous_versus_season: LegendLeagueTournamentSeasonResult,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct LegendLeagueTournamentSeasonResult {
    pub trophies: u64,
    pub id: String,
    pub rank: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerItemLevel {
    pub level: u64,
    pub name: JsonLocalizedName,
    pub max_level: u64,
    pub village: Village,
    #[serde(default)]
    pub super_troop_is_active: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Village {
    Home,
    BuilderBase,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Role {
    NotMember,
    Member,
    Leader,
    Admin,
    CoLeader,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum WarPreference {
    Out,
    In,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerAchievementProgress {
    pub stars: u64,
    pub value: u64,
    pub name: JsonLocalizedName,
    pub target: u64,
    pub info: JsonLocalizedName,
    pub completion_info: JsonLocalizedName,
    pub village: Village,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyTokenRequest {
    pub token: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyTokenResponse {
    pub tag: String,
    pub token: String,
    pub status: String,
}

#[cfg(test)]
mod tests;
