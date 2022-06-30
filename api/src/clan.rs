use super::*;

mod impls;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Clan {
    pub member_list: Vec<ClanMember>,
    pub war_league: WarLeague,
    pub tag: String,
    pub clan_level: u64,
    pub clan_points: u64,
    pub chat_language: Option<Language>,
    pub war_frequency: WarFrequency,
    pub clan_versus_points: u64,
    pub required_trophies: u64,
    pub required_versus_trophies: u64,
    pub required_townhall_level: u64,
    pub is_war_log_public: bool,
    #[serde(default)]
    pub war_win_streak: u64,
    #[serde(default)]
    pub war_wins: u64,
    #[serde(default)]
    pub war_ties: u64,
    #[serde(default)]
    pub war_losses: u64,
    pub labels: Vec<Label>,
    pub name: String,
    pub location: Option<Location>,
    pub r#type: ClanType,
    pub members: u64,
    pub description: String,
    pub badge_urls: Urls,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClanMember {
    pub league: League,
    pub tag: String,
    pub name: String,
    pub role: Role,
    pub exp_level: u64,
    pub clan_rank: u64,
    pub previous_clan_rank: u64,
    pub donations: u64,
    pub donations_received: u64,
    pub trophies: u64,
    pub versus_trophies: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WarLeague {
    pub name: JsonLocalizedName,
    pub id: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Language {
    pub name: String,
    pub id: u64,
    pub language_code: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum WarFrequency {
    Unknown,
    Always,
    MoreThanOncePerWeek,
    OncePerWeek,
    LessThanOncePerWeek,
    Never,
    Any,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Location {
    pub localized_name: String,
    pub id: u64,
    pub name: String,
    pub is_country: bool,
    pub country_code: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ClanType {
    Open,
    InviteOnly,
    Closed,
}
