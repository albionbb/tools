use albion_packets_macros::PhotonPacket;
use photon_decoder::PhotonValue;
use std::collections::HashMap;

#[derive(Debug, Clone, Default, PhotonPacket)]
pub struct OperationGetGameServerByCluster {
    #[photon(index = 1)]
    pub cluster_id: String,
}

#[derive(Debug, Clone, Default, PhotonPacket)]
pub struct OperationAuctionGetOffers {
    #[photon(index = 1)]
    pub category: String,
    #[photon(index = 2)]
    pub sub_category: String,
    #[photon(index = 5)]
    pub quality: String,
    #[photon(index = 6)]
    pub enchantment: u32,
    #[photon(index = 10)]
    pub enchantment_level: String,
    #[photon(index = 8)]
    pub item_ids: Vec<u16>,
    #[photon(index = 12)]
    pub max_results: u32,
    #[photon(index = 14)]
    pub is_ascending_order: bool,
}

#[derive(Debug, Clone, Default, PhotonPacket)]
pub struct OperationAuctionGetItemAverageStats {
    #[photon(index = 1)]
    pub item_id: u32,
}

#[derive(Debug, Clone, Default, PhotonPacket)]
pub struct OperationGetClusterMapInfo {
    #[photon(index = 1)]
    pub cluster_id: String,
}

#[derive(Debug, Clone, Default, PhotonPacket)]
pub struct OperationGoldMarketGetAverageInfo {}

#[derive(Debug, Clone, Default, PhotonPacket)]
pub struct OperationRealEstateGetAuctionData {}

#[derive(Debug, Clone, Default, PhotonPacket)]
pub struct OperationRealEstateBidOnAuction {
    #[photon(index = 1)]
    pub auction_id: u64,
    #[photon(index = 2)]
    pub bid_amount: u64,
}

#[derive(Debug, Clone, Default, PhotonPacket)]
pub struct OperationJoinResponse {
    #[photon(index = 1, decode_with = "crate::utils::decode_character_id")]
    pub character_id: String,
    #[photon(index = 2)]
    pub character_name: String,
    #[photon(index = 8)]
    pub location: String,
    #[photon(index = 56, decode_with = "crate::utils::decode_character_id")]
    pub guild_id: String,
    #[photon(index = 58)]
    pub guild_name: String,
}

#[derive(Debug, Clone, Default, PhotonPacket)]
pub struct OperationAuctionGetOffersResponse {
    #[photon(index = 0)]
    pub market_orders: Vec<String>,
}

#[derive(Debug, Clone, Default, PhotonPacket)]
pub struct OperationAuctionGetRequestsResponse {
    #[photon(index = 0)]
    pub market_orders: Vec<String>,
}

#[derive(Debug, Clone, Default, PhotonPacket)]
pub struct OperationAuctionGetItemAverageStatsResponse {
    #[photon(index = 1)]
    pub item_id: u32,
    #[photon(index = 2)]
    pub average_price: f64,
}

#[derive(Debug, Clone, Default, PhotonPacket)]
pub struct OperationGetMailInfosResponse {
    #[photon(index = 1)]
    pub mail_count: u32,
}

#[derive(Debug, Clone, Default, PhotonPacket)]
pub struct OperationReadMailResponse {
    #[photon(index = 1)]
    pub mail_id: u64,
    #[photon(index = 2)]
    pub subject: String,
    #[photon(index = 3)]
    pub body: String,
}

#[derive(Debug, Clone, Default, PhotonPacket)]
pub struct OperationGetClusterMapInfoResponse {
    #[photon(index = 1)]
    pub cluster_id: String,
    #[photon(index = 2)]
    pub cluster_name: String,
}

#[derive(Debug, Clone, Default, PhotonPacket)]
pub struct OperationGoldMarketGetAverageInfoResponse {
    #[photon(index = 1)]
    pub average_gold_price: f64,
}

#[derive(Debug, Clone, Default, PhotonPacket)]
pub struct OperationRealEstateGetAuctionDataResponse {
    #[photon(index = 1)]
    pub auction_id: u64,
    #[photon(index = 2)]
    pub starting_bid: u64,
    #[photon(index = 3)]
    pub buyout_price: u64,
}

#[derive(Debug, Clone, Default, PhotonPacket)]
pub struct OperationRealEstateBidOnAuctionResponse {
    #[photon(index = 1)]
    pub success: bool,
    #[photon(index = 2)]
    pub auction_id: u64,
}

#[derive(Debug, Clone, Default, PhotonPacket)]
pub struct EventRedZoneWorldMapEvent {
    #[photon(index = 0)]
    pub event_time: i64,
    #[photon(index = 1)]
    pub phase: i32,
}

#[derive(Debug, Clone, Default, PhotonPacket)]
pub struct EventFullAchievementInfo {
    #[photon(index = 1)]
    pub completed_achievement_ids: Vec<i16>,
    #[photon(index = 2)]
    pub active_achievement_ids: Vec<i16>,
    #[photon(index = 3)]
    pub active_achievement_levels: Vec<u8>,
}

#[derive(Debug, Clone, Default, PhotonPacket)]
pub struct EventCharacterStats {
    #[photon(index = 1)]
    pub player_name: String,
    #[photon(index = 2)]
    pub guild_name: String,
    #[photon(index = 4)]
    pub alliance_name: String,
    #[photon(index = 5)]
    pub profile_description: String,
    #[photon(index = 8)]
    pub reputation: f32,
    #[photon(index = 6)]
    pub silver: u64,
    #[photon(index = 7)]
    pub total_fame: u64,
    #[photon(index = 11)]
    pub fame_pvp: u64,
    #[photon(index = 13)]
    pub fame_pve: u64,
    #[photon(index = 14)]
    pub fame_gathering: u64,
    #[photon(index = 16)]
    pub fame_crafting: u64,
    #[photon(index = 10)]
    pub total_kills: u32,
    #[photon(index = 15)]
    pub resources_invested: u64,
    #[photon(index = 60)]
    pub current_rank: u32,
    #[photon(index = 61)]
    pub current_rank_points: u32,
    #[photon(index = 62)]
    pub highest_rank_points: u32,
    #[photon(index = 9, dict_key = 2)]
    pub standing_antiquarian: u64,
    #[photon(index = 9, dict_key = 0)]
    pub standing_brecilien: u64,
    #[photon(index = 9, dict_key = 1)]
    pub standing_smugglers: u64,
    #[photon(index = 29)]
    pub arena_battles_played: u32,
    #[photon(index = 30)]
    pub arena_battles_won: u32,
    #[photon(index = 31)]
    pub crystal_arena_matches_played: u32,
    #[photon(index = 32)]
    pub crystal_arena_matches_won: u32,
    #[photon(index = 59)]
    pub crystal_league_5v5_battles: u32,
    #[photon(index = 45)]
    pub crystal_league_5v5_wins: u32,
    #[photon(index = 42)]
    pub crystal_league_5v5_lethal_battles: u32,
    #[photon(index = 43)]
    pub crystal_league_5v5_lethal_wins: u32,
    #[photon(index = 46)]
    pub crystal_league_20v20_battles: u32,
    #[photon(index = 47)]
    pub crystal_league_20v20_wins: u32,
    #[photon(index = 48)]
    pub crystal_league_kills: u32,
    #[photon(index = 49)]
    pub crystal_league_kill_fame: u64,
    #[photon(index = 36)]
    pub crystal_realm_battles: u32,
    #[photon(index = 38)]
    pub crystal_realm_kills: u32,
    #[photon(index = 39)]
    pub crystal_realm_kill_fame: u64,
    #[photon(index = 52)]
    pub infamy_corrupted: u32,
    #[photon(index = 54)]
    pub infamy_2v2_hellgates: u32,
    #[photon(index = 56)]
    pub infamy_5v5_hellgates: u32,
    #[photon(index = 58)]
    pub infamy_10v10_hellgates: u32,
    #[photon(index = 51)]
    pub infamy_corrupted_highest: u32,
    #[photon(index = 53)]
    pub infamy_2v2_hellgates_highest: u32,
    #[photon(index = 55)]
    pub infamy_5v5_hellgates_highest: u32,
    #[photon(index = 18)]
    pub gvg_kills: u32,
    #[photon(index = 17)]
    pub gvg_fights_participated: u32,
    #[photon(index = 19)]
    pub gvg_fame: u64,
}

// ── conversion helpers ───────────────────────────────────────────────────────

pub trait FromPhotonValue: Sized {
    fn from_photon(value: &PhotonValue) -> Option<Self>;
}

impl FromPhotonValue for bool {
    fn from_photon(value: &PhotonValue) -> Option<Self> {
        match value {
            PhotonValue::Bool(b) => Some(*b),
            _ => None,
        }
    }
}

impl FromPhotonValue for u8 {
    fn from_photon(value: &PhotonValue) -> Option<Self> {
        match value {
            PhotonValue::Byte(b) => Some(*b),
            _ => None,
        }
    }
}

impl FromPhotonValue for i16 {
    fn from_photon(value: &PhotonValue) -> Option<Self> {
        match value {
            PhotonValue::Short(v) => Some(*v),
            _ => None,
        }
    }
}

impl FromPhotonValue for i32 {
    fn from_photon(value: &PhotonValue) -> Option<Self> {
        match value {
            PhotonValue::Int(v) => Some(*v),
            PhotonValue::Short(v) => Some(*v as i32),
            PhotonValue::Byte(v) => Some(*v as i32),
            _ => None,
        }
    }
}

impl FromPhotonValue for u32 {
    fn from_photon(value: &PhotonValue) -> Option<Self> {
        match value {
            PhotonValue::Long(v) => Some(*v as u32),
            PhotonValue::Int(v) => Some(*v as u32),
            PhotonValue::Short(v) => Some(*v as u32),
            PhotonValue::Byte(v) => Some(*v as u32),
            _ => None,
        }
    }
}

impl FromPhotonValue for i64 {
    fn from_photon(value: &PhotonValue) -> Option<Self> {
        match value {
            PhotonValue::Long(v) => Some(*v),
            PhotonValue::Int(v) => Some(*v as i64),
            PhotonValue::Short(v) => Some(*v as i64),
            PhotonValue::Byte(v) => Some(*v as i64),
            _ => None,
        }
    }
}

impl FromPhotonValue for u64 {
    fn from_photon(value: &PhotonValue) -> Option<Self> {
        match value {
            PhotonValue::Long(v) => Some(*v as u64),
            PhotonValue::Int(v) => Some(*v as u64),
            PhotonValue::Short(v) => Some(*v as u64),
            PhotonValue::Byte(v) => Some(*v as u64),
            _ => None,
        }
    }
}

impl FromPhotonValue for f32 {
    fn from_photon(value: &PhotonValue) -> Option<Self> {
        match value {
            PhotonValue::Float(v) => Some(*v),
            PhotonValue::Double(v) => Some(*v as f32),
            _ => None,
        }
    }
}

impl FromPhotonValue for f64 {
    fn from_photon(value: &PhotonValue) -> Option<Self> {
        match value {
            PhotonValue::Double(v) => Some(*v),
            PhotonValue::Float(v) => Some(*v as f64),
            _ => None,
        }
    }
}

impl FromPhotonValue for String {
    fn from_photon(value: &PhotonValue) -> Option<Self> {
        match value {
            PhotonValue::String(s) => Some(s.clone()),
            _ => None,
        }
    }
}

impl FromPhotonValue for Vec<String> {
    fn from_photon(value: &PhotonValue) -> Option<Self> {
        match value {
            PhotonValue::Array(arr) | PhotonValue::ObjectArray(arr) => arr
                .iter()
                .map(|v| match v {
                    PhotonValue::String(s) => Some(s.clone()),
                    _ => None,
                })
                .collect(),
            _ => None,
        }
    }
}

impl FromPhotonValue for Vec<u8> {
    fn from_photon(value: &PhotonValue) -> Option<Self> {
        match value {
            PhotonValue::Array(arr) => arr
                .iter()
                .map(|v| match v {
                    PhotonValue::Byte(b) => Some(*b),
                    _ => None,
                })
                .collect(),
            _ => None,
        }
    }
}

impl FromPhotonValue for Vec<u16> {
    fn from_photon(value: &PhotonValue) -> Option<Self> {
        match value {
            PhotonValue::Array(arr) => arr
                .iter()
                .map(|v| match v {
                    PhotonValue::Short(s) => Some(*s as u16),
                    PhotonValue::Int(i) => Some(*i as u16),
                    PhotonValue::Byte(b) => Some(*b as u16),
                    _ => None,
                })
                .collect(),
            _ => None,
        }
    }
}

impl FromPhotonValue for Vec<i16> {
    fn from_photon(value: &PhotonValue) -> Option<Self> {
        match value {
            PhotonValue::Array(arr) => arr
                .iter()
                .map(|v| match v {
                    PhotonValue::Short(s) => Some(*s),
                    PhotonValue::Int(i) => Some(*i as i16),
                    PhotonValue::Byte(b) => Some(*b as i16),
                    _ => None,
                })
                .collect(),
            _ => None,
        }
    }
}

pub fn get_param<T: FromPhotonValue>(params: &HashMap<u8, PhotonValue>, key: u8) -> Option<T> {
    params.get(&key).and_then(T::from_photon)
}
