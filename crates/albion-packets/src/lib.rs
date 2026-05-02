pub mod capture;
pub mod events;
pub mod operations;
pub mod ops;
pub mod utils;

use crate::events::EventType;
use crate::operations::OperationType;
use crate::ops::*;
use photon_decoder::PhotonValue;
use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum AlbionOperation {
    RequestGetGameServerByCluster(OperationGetGameServerByCluster),
    RequestAuctionGetOffers(OperationAuctionGetOffers),
    RequestAuctionGetItemAverageStats(OperationAuctionGetItemAverageStats),
    RequestGetClusterMapInfo(OperationGetClusterMapInfo),
    RequestGoldMarketGetAverageInfo(OperationGoldMarketGetAverageInfo),
    RequestRealEstateGetAuctionData(OperationRealEstateGetAuctionData),
    RequestRealEstateBidOnAuction(OperationRealEstateBidOnAuction),

    ResponseJoin(OperationJoinResponse),
    ResponseAuctionGetOffers(OperationAuctionGetOffersResponse),
    ResponseAuctionGetRequests(OperationAuctionGetRequestsResponse),
    ResponseAuctionGetItemAverageStats(OperationAuctionGetItemAverageStatsResponse),
    ResponseGetMailInfos(OperationGetMailInfosResponse),
    ResponseReadMail(OperationReadMailResponse),
    ResponseGetClusterMapInfo(OperationGetClusterMapInfoResponse),
    ResponseGoldMarketGetAverageInfo(OperationGoldMarketGetAverageInfoResponse),
    ResponseRealEstateGetAuctionData(OperationRealEstateGetAuctionDataResponse),
    ResponseRealEstateBidOnAuction(OperationRealEstateBidOnAuctionResponse),

    EventCharacterStats(EventCharacterStats),
    EventFullAchievementInfo(EventFullAchievementInfo),
    EventRedZoneWorldMapEvent(EventRedZoneWorldMapEvent),

    UnknownRequest(u8, HashMap<u8, PhotonValue>),
    UnknownResponse(u8, i16, String, HashMap<u8, PhotonValue>),
    UnknownEvent(u8, HashMap<u8, PhotonValue>),
}

pub fn decode_request(params: HashMap<u8, PhotonValue>) -> Option<AlbionOperation> {
    let code = resolve_operation_code(&params)?;
    let op_type = OperationType(code);

    match op_type {
        OperationType::opGetGameServerByCluster => Some(
            AlbionOperation::RequestGetGameServerByCluster(OperationGetGameServerByCluster {
                cluster_id: get_param(&params, 1).unwrap_or_default(),
            }),
        ),
        OperationType::opAuctionGetOffers => Some(AlbionOperation::RequestAuctionGetOffers(
            OperationAuctionGetOffers {
                category: get_param(&params, 1).unwrap_or_default(),
                sub_category: get_param(&params, 2).unwrap_or_default(),
                quality: get_param(&params, 5).unwrap_or_default(),
                enchantment: get_param(&params, 6).unwrap_or_default(),
                enchantment_level: get_param(&params, 10).unwrap_or_default(),
                item_ids: get_param(&params, 8).unwrap_or_default(),
                max_results: get_param(&params, 12).unwrap_or_default(),
                is_ascending_order: get_param(&params, 14).unwrap_or_default(),
            },
        )),
        OperationType::opAuctionGetItemAverageStats => {
            Some(AlbionOperation::RequestAuctionGetItemAverageStats(
                OperationAuctionGetItemAverageStats {
                    item_id: get_param(&params, 1).unwrap_or_default(),
                },
            ))
        }
        OperationType::opGetClusterMapInfo => Some(AlbionOperation::RequestGetClusterMapInfo(
            OperationGetClusterMapInfo {
                cluster_id: get_param(&params, 1).unwrap_or_default(),
            },
        )),
        OperationType::opGoldMarketGetAverageInfo => {
            Some(AlbionOperation::RequestGoldMarketGetAverageInfo(
                OperationGoldMarketGetAverageInfo::default(),
            ))
        }
        OperationType::opRealEstateGetAuctionData => {
            Some(AlbionOperation::RequestRealEstateGetAuctionData(
                OperationRealEstateGetAuctionData::default(),
            ))
        }
        OperationType::opRealEstateBidOnAuction => Some(
            AlbionOperation::RequestRealEstateBidOnAuction(OperationRealEstateBidOnAuction {
                auction_id: get_param(&params, 1).unwrap_or_default(),
                bid_amount: get_param(&params, 2).unwrap_or_default(),
            }),
        ),
        _ => None,
    }
}

pub fn decode_response(
    params: HashMap<u8, PhotonValue>,
    _return_code: i16,
    _debug_message: String,
) -> Option<AlbionOperation> {
    let code = resolve_operation_code(&params)?;
    let op_type = OperationType(code);

    match op_type {
        OperationType::opJoin => {
            let character_id = get_param::<Vec<u8>>(&params, 1)
                .map(|bytes| utils::decode_character_id(&bytes))
                .unwrap_or_default();
            let guild_id = get_param::<Vec<u8>>(&params, 56)
                .map(|bytes| utils::decode_character_id(&bytes))
                .unwrap_or_default();
            Some(AlbionOperation::ResponseJoin(OperationJoinResponse {
                character_id,
                character_name: get_param(&params, 2).unwrap_or_default(),
                location: get_param(&params, 8).unwrap_or_default(),
                guild_id,
                guild_name: get_param(&params, 58).unwrap_or_default(),
            }))
        }
        OperationType::opAuctionGetOffers => Some(AlbionOperation::ResponseAuctionGetOffers(
            OperationAuctionGetOffersResponse {
                market_orders: get_string_array(&params, 0).unwrap_or_default(),
            },
        )),
        OperationType::opAuctionGetRequests => Some(AlbionOperation::ResponseAuctionGetRequests(
            OperationAuctionGetRequestsResponse {
                market_orders: get_string_array(&params, 0).unwrap_or_default(),
            },
        )),
        OperationType::opAuctionGetItemAverageStats => {
            Some(AlbionOperation::ResponseAuctionGetItemAverageStats(
                OperationAuctionGetItemAverageStatsResponse {
                    item_id: get_param(&params, 1).unwrap_or_default(),
                    average_price: get_param(&params, 2).unwrap_or_default(),
                },
            ))
        }
        OperationType::opGetMailInfos => Some(AlbionOperation::ResponseGetMailInfos(
            OperationGetMailInfosResponse {
                mail_count: get_param(&params, 1).unwrap_or_default(),
            },
        )),
        OperationType::opReadMail => Some(AlbionOperation::ResponseReadMail(
            OperationReadMailResponse {
                mail_id: get_param(&params, 1).unwrap_or_default(),
                subject: get_param(&params, 2).unwrap_or_default(),
                body: get_param(&params, 3).unwrap_or_default(),
            },
        )),
        OperationType::opGetClusterMapInfo => Some(AlbionOperation::ResponseGetClusterMapInfo(
            OperationGetClusterMapInfoResponse {
                cluster_id: get_param(&params, 1).unwrap_or_default(),
                cluster_name: get_param(&params, 2).unwrap_or_default(),
            },
        )),
        OperationType::opGoldMarketGetAverageInfo => {
            Some(AlbionOperation::ResponseGoldMarketGetAverageInfo(
                OperationGoldMarketGetAverageInfoResponse {
                    average_gold_price: get_param(&params, 1).unwrap_or_default(),
                },
            ))
        }
        OperationType::opRealEstateGetAuctionData => {
            Some(AlbionOperation::ResponseRealEstateGetAuctionData(
                OperationRealEstateGetAuctionDataResponse {
                    auction_id: get_param(&params, 1).unwrap_or_default(),
                    starting_bid: get_param(&params, 2).unwrap_or_default(),
                    buyout_price: get_param(&params, 3).unwrap_or_default(),
                },
            ))
        }
        OperationType::opRealEstateBidOnAuction => {
            Some(AlbionOperation::ResponseRealEstateBidOnAuction(
                OperationRealEstateBidOnAuctionResponse {
                    success: get_param(&params, 1).unwrap_or_default(),
                    auction_id: get_param(&params, 2).unwrap_or_default(),
                },
            ))
        }
        _ => None,
    }
}

pub fn decode_event(params: HashMap<u8, PhotonValue>) -> Option<AlbionOperation> {
    let code = resolve_event_code(&params)?;
    let ev_type = EventType(code);

    match ev_type {
        EventType::evCharacterStats => {
            let standings = match params.get(&9) {
                Some(PhotonValue::Dictionary(dict)) => {
                    let get = |k: i32| -> u64 {
                        match dict.get(&PhotonValue::Int(k)) {
                            Some(PhotonValue::Long(v)) => *v as u64,
                            Some(PhotonValue::Int(v)) => *v as u64,
                            Some(PhotonValue::Short(v)) => *v as u64,
                            _ => 0,
                        }
                    };
                    (get(0), get(1), get(2))
                }
                _ => (0, 0, 0),
            };
            Some(AlbionOperation::EventCharacterStats(EventCharacterStats {
                player_name: get_param(&params, 1).unwrap_or_default(),
                guild_name: get_param(&params, 2).unwrap_or_default(),
                alliance_name: get_param(&params, 4).unwrap_or_default(),
                profile_description: get_param(&params, 5).unwrap_or_default(),
                reputation: get_param(&params, 8).unwrap_or_default(),
                total_fame: get_param(&params, 7).unwrap_or_default(),
                fame_pvp: get_param(&params, 11).unwrap_or_default(),
                fame_pve: get_param(&params, 13).unwrap_or_default(),
                fame_gathering: get_param(&params, 14).unwrap_or_default(),
                fame_crafting: get_param(&params, 16).unwrap_or_default(),
                total_kills: get_param(&params, 10).unwrap_or_default(),
                resources_invested: get_param(&params, 15).unwrap_or_default(),
                current_rank: get_param(&params, 60).unwrap_or_default(),
                current_rank_points: get_param(&params, 61).unwrap_or_default(),
                highest_rank_points: get_param(&params, 62).unwrap_or_default(),
                standing_brecilien: standings.0,
                standing_smugglers: standings.1,
                standing_antiquarian: standings.2,
                arena_battles_played: get_param(&params, 29).unwrap_or_default(),
                arena_battles_won: get_param(&params, 30).unwrap_or_default(),
                crystal_arena_matches_played: get_param(&params, 31).unwrap_or_default(),
                crystal_arena_matches_won: get_param(&params, 32).unwrap_or_default(),
                crystal_league_5v5_battles: get_param(&params, 59).unwrap_or_default(),
                crystal_league_5v5_wins: get_param(&params, 45).unwrap_or_default(),
                crystal_league_5v5_lethal_battles: get_param(&params, 42).unwrap_or_default(),
                crystal_league_5v5_lethal_wins: get_param(&params, 43).unwrap_or_default(),
                crystal_league_20v20_battles: get_param(&params, 46).unwrap_or_default(),
                crystal_league_20v20_wins: get_param(&params, 47).unwrap_or_default(),
                crystal_league_kills: get_param(&params, 48).unwrap_or_default(),
                crystal_league_kill_fame: get_param(&params, 49).unwrap_or_default(),
                crystal_realm_battles: get_param(&params, 36).unwrap_or_default(),
                crystal_realm_kills: get_param(&params, 38).unwrap_or_default(),
                crystal_realm_kill_fame: get_param(&params, 39).unwrap_or_default(),
                infamy_corrupted: get_param(&params, 52).unwrap_or_default(),
                infamy_corrupted_highest: get_param(&params, 51).unwrap_or_default(),
                infamy_2v2_hellgates: get_param(&params, 54).unwrap_or_default(),
                infamy_2v2_hellgates_highest: get_param(&params, 53).unwrap_or_default(),
                infamy_5v5_hellgates: get_param(&params, 56).unwrap_or_default(),
                infamy_5v5_hellgates_highest: get_param(&params, 55).unwrap_or_default(),
                infamy_10v10_hellgates: get_param(&params, 58).unwrap_or_default(),
                gvg_kills: get_param(&params, 18).unwrap_or_default(),
                gvg_fights_participated: get_param(&params, 17).unwrap_or_default(),
                gvg_fame: get_param(&params, 19).unwrap_or_default(),
            }))
        }
        EventType::evRedZoneWorldMapEvent => Some(AlbionOperation::EventRedZoneWorldMapEvent(
            EventRedZoneWorldMapEvent {
                event_time: get_param(&params, 0).unwrap_or_default(),
                phase: get_param(&params, 1).unwrap_or_default(),
            },
        )),
        EventType::evFullAchievementInfo => Some(AlbionOperation::EventFullAchievementInfo(
            EventFullAchievementInfo {
                completed_achievement_ids: get_param(&params, 1).unwrap_or_default(),
                active_achievement_ids: get_param(&params, 2).unwrap_or_default(),
                active_achievement_levels: get_param(&params, 3).unwrap_or_default(),
            },
        )),
        _ => None,
    }
}

fn resolve_operation_code(params: &HashMap<u8, PhotonValue>) -> Option<u16> {
    let v = params.get(&253)?;
    let code = to_u16(v)?;
    Some(utils::normalize_operation_code(code))
}

fn resolve_event_code(params: &HashMap<u8, PhotonValue>) -> Option<u16> {
    let v = params.get(&252)?;
    let code = to_u16(v)?;
    Some(utils::normalize_event_code(code))
}

fn to_u16(v: &PhotonValue) -> Option<u16> {
    match v {
        PhotonValue::Short(v) => Some(*v as u16),
        PhotonValue::Int(v) => Some(*v as u16),
        PhotonValue::Long(v) => Some(*v as u16),
        PhotonValue::Byte(v) => Some(*v as u16),
        _ => None,
    }
}
