use serde::{ Serialize, Deserialize };
use crate::models::badge_urls::BadgeUrls;

#[derive(Debug, Serialize, Deserialize)]
pub struct War {
    #[serde(rename = "state")]
    state: String,

    #[serde(rename = "teamSize")]
    team_size: Option<i32>,

    #[serde(rename = "attacksPerMember")]
    attacks_per_member: Option<i8>,

    #[serde(rename = "preparationStartTime")]
    preparation_start_time: Option<String>,

    #[serde(rename = "startTime")]
    start_time: Option<String>,

    #[serde(rename = "endTime")]
    end_time: Option<String>,

    #[serde(rename = "clan")]
    clan: Option<WarClan>,

    #[serde(rename = "opponent")]
    opponent: Option<WarClan>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WarClan {
    #[serde(rename = "tag")]
    tag: Option<String>,

    #[serde(rename = "name")]
    name: Option<String>,

    #[serde(rename = "badgeUrls")]
    badge_urls: BadgeUrls,

    #[serde(rename = "clanLevel")]
    clan_level: Option<i8>,

    #[serde(rename = "attacks")]
    attacks: Option<i32>,

    #[serde(rename = "stars")]
    stars: Option<i32>,

    #[serde(rename = "destructionPercentage")]
    destruction_percentage: Option<f64>,

    #[serde(rename = "members")]
    members: Option<Vec<Member>>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Member {
    #[serde(rename = "tag")]
    tag: String,

    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "townhallLevel")]
    townhall_level: i8,

    #[serde(rename = "mapPosition")]
    map_position: i32,

    #[serde(rename = "attacks")]
    attacks: Option<Vec<Attack>>,

    #[serde(rename = "opponentAttacks")]
    opponent_attacks: i32,

    #[serde(rename = "bestOpponentAttack")]
    best_opponent_attack: Attack,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attack {
    #[serde(rename = "attackerTag")]
    attacker_tag: String,

    #[serde(rename = "defenderTag")]
    defender_tag: String,

    #[serde(rename = "stars")]
    stars: i32,

    #[serde(rename = "destructionPercentage")]
    destruction_percentage: f32,

    #[serde(rename = "order")]
    order: i32,

    #[serde(rename = "duration")]
    duration: i32,
}

impl War {
    pub fn state(&self) -> &str {
        &self.state
    }
    pub fn team_size(&self) -> Option<i32> {
        self.team_size
    }
    pub fn attacks_per_member(&self) -> Option<i8> {
        self.attacks_per_member
    }
    pub fn preparation_start_time(&self) -> &Option<String> {
        &self.preparation_start_time
    }
    pub fn start_time(&self) -> &Option<String> {
        &self.start_time
    }
    pub fn end_time(&self) -> &Option<String> {
        &self.end_time
    }
    pub fn clan(&self) -> &Option<WarClan> {
        &self.clan
    }
    pub fn opponent(&self) -> &Option<WarClan> {
        &self.opponent
    }
}

impl WarClan {
    pub fn tag(&self) -> &Option<String> {
        &self.tag
    }
    pub fn name(&self) -> &Option<String> {
        &self.name
    }
    pub fn badge_urls(&self) -> &BadgeUrls {
        &self.badge_urls
    }
    pub fn clan_level(&self) -> Option<i8> {
        self.clan_level
    }
    pub fn attacks(&self) -> Option<i32> {
        self.attacks
    }
    pub fn stars(&self) -> Option<i32> {
        self.stars
    }
    pub fn destruction_percentage(&self) -> Option<f64> {
        self.destruction_percentage
    }
    pub fn members(&self) -> &Option<Vec<Member>> {
        &self.members
    }
}

impl Member {
    pub fn tag(&self) -> &str {
        &self.tag
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn townhall_level(&self) -> i8 {
        self.townhall_level
    }
    pub fn map_position(&self) -> i32 {
        self.map_position
    }
    pub fn attacks(&self) -> &Option<Vec<Attack>> {
        &self.attacks
    }
    pub fn opponent_attacks(&self) -> i32 {
        self.opponent_attacks
    }
    pub fn best_opponent_attack(&self) -> &Attack {
        &self.best_opponent_attack
    }
}

impl Attack {
    pub fn attacker_tag(&self) -> &str {
        &self.attacker_tag
    }
    pub fn defender_tag(&self) -> &str {
        &self.defender_tag
    }
    pub fn stars(&self) -> i32 {
        self.stars
    }
    pub fn destruction_percentage(&self) -> f32 {
        self.destruction_percentage
    }
    pub fn order(&self) -> i32 {
        self.order
    }
    pub fn duration(&self) -> i32 {
        self.duration
    }
}
