use photon_decoder::PhotonValue;
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct OperationGetGameServerByCluster {
    pub cluster_id: String,
}

#[derive(Debug, Clone, Default)]
pub struct OperationAuctionGetOffers {
    pub category: String,
    pub sub_category: String,
    pub quality: String,
    pub enchantment: u32,
    pub enchantment_level: String,
    pub item_ids: Vec<u16>,
    pub max_results: u32,
    pub is_ascending_order: bool,
}

#[derive(Debug, Clone, Default)]
pub struct OperationAuctionGetItemAverageStats {
    pub item_id: u32,
}

#[derive(Debug, Clone, Default)]
pub struct OperationGetClusterMapInfo {
    pub cluster_id: String,
}

#[derive(Debug, Clone, Default)]
pub struct OperationGoldMarketGetAverageInfo {}

#[derive(Debug, Clone, Default)]
pub struct OperationRealEstateGetAuctionData {}

#[derive(Debug, Clone, Default)]
pub struct OperationRealEstateBidOnAuction {
    pub auction_id: u64,
    pub bid_amount: u64,
}

#[derive(Debug, Clone, Default)]
pub struct OperationJoinResponse {
    pub character_id: String,
    pub character_name: String,
    pub location: String,
    pub guild_id: String,
    pub guild_name: String,
}

#[derive(Debug, Clone, Default)]
pub struct OperationAuctionGetOffersResponse {
    pub market_orders: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct OperationAuctionGetRequestsResponse {
    pub market_orders: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct OperationAuctionGetItemAverageStatsResponse {
    pub item_id: u32,
    pub average_price: f64,
}

#[derive(Debug, Clone, Default)]
pub struct OperationGetMailInfosResponse {
    pub mail_count: u32,
}

#[derive(Debug, Clone, Default)]
pub struct OperationReadMailResponse {
    pub mail_id: u64,
    pub subject: String,
    pub body: String,
}

#[derive(Debug, Clone, Default)]
pub struct OperationGetClusterMapInfoResponse {
    pub cluster_id: String,
    pub cluster_name: String,
}

#[derive(Debug, Clone, Default)]
pub struct OperationGoldMarketGetAverageInfoResponse {
    pub average_gold_price: f64,
}

#[derive(Debug, Clone, Default)]
pub struct OperationRealEstateGetAuctionDataResponse {
    pub auction_id: u64,
    pub starting_bid: u64,
    pub buyout_price: u64,
}

#[derive(Debug, Clone, Default)]
pub struct OperationRealEstateBidOnAuctionResponse {
    pub success: bool,
    pub auction_id: u64,
}

#[derive(Debug, Clone, Default)]
pub struct EventRedZoneWorldMapEvent {
    pub event_time: i64,
    pub phase: i32,
}

#[derive(Debug, Clone, Default)]
pub struct EventFullAchievementInfo {
    pub completed_achievement_ids: Vec<i16>,
    pub active_achievement_ids: Vec<i16>,
    pub active_achievement_levels: Vec<u8>,
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

pub fn get_string_array(params: &HashMap<u8, PhotonValue>, key: u8) -> Option<Vec<String>> {
    match params.get(&key)? {
        PhotonValue::Array(arr) => arr
            .iter()
            .map(|v| match v {
                PhotonValue::String(s) => Some(s.clone()),
                _ => None,
            })
            .collect(),
        _ => None,
    }
}
