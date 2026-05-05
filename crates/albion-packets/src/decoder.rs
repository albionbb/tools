use crate::structs::*;
use crate::types::{EventType, OperationType};
use crate::utils;
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
        OperationType::opGetGameServerByCluster => OperationGetGameServerByCluster::decode(&params)
            .map(AlbionOperation::RequestGetGameServerByCluster),
        OperationType::opAuctionGetOffers => {
            OperationAuctionGetOffers::decode(&params).map(AlbionOperation::RequestAuctionGetOffers)
        }
        OperationType::opAuctionGetItemAverageStats => {
            OperationAuctionGetItemAverageStats::decode(&params)
                .map(AlbionOperation::RequestAuctionGetItemAverageStats)
        }
        OperationType::opGetClusterMapInfo => OperationGetClusterMapInfo::decode(&params)
            .map(AlbionOperation::RequestGetClusterMapInfo),
        OperationType::opGoldMarketGetAverageInfo => {
            OperationGoldMarketGetAverageInfo::decode(&params)
                .map(AlbionOperation::RequestGoldMarketGetAverageInfo)
        }
        OperationType::opRealEstateGetAuctionData => {
            OperationRealEstateGetAuctionData::decode(&params)
                .map(AlbionOperation::RequestRealEstateGetAuctionData)
        }
        OperationType::opRealEstateBidOnAuction => OperationRealEstateBidOnAuction::decode(&params)
            .map(AlbionOperation::RequestRealEstateBidOnAuction),
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
            OperationJoinResponse::decode(&params).map(AlbionOperation::ResponseJoin)
        }
        OperationType::opAuctionGetOffers => OperationAuctionGetOffersResponse::decode(&params)
            .map(AlbionOperation::ResponseAuctionGetOffers),
        OperationType::opAuctionGetRequests => OperationAuctionGetRequestsResponse::decode(&params)
            .map(AlbionOperation::ResponseAuctionGetRequests),
        OperationType::opAuctionGetItemAverageStats => {
            OperationAuctionGetItemAverageStatsResponse::decode(&params)
                .map(AlbionOperation::ResponseAuctionGetItemAverageStats)
        }
        OperationType::opGetMailInfos => OperationGetMailInfosResponse::decode(&params)
            .map(AlbionOperation::ResponseGetMailInfos),
        OperationType::opReadMail => {
            OperationReadMailResponse::decode(&params).map(AlbionOperation::ResponseReadMail)
        }
        OperationType::opGetClusterMapInfo => OperationGetClusterMapInfoResponse::decode(&params)
            .map(AlbionOperation::ResponseGetClusterMapInfo),
        OperationType::opGoldMarketGetAverageInfo => {
            OperationGoldMarketGetAverageInfoResponse::decode(&params)
                .map(AlbionOperation::ResponseGoldMarketGetAverageInfo)
        }
        OperationType::opRealEstateGetAuctionData => {
            OperationRealEstateGetAuctionDataResponse::decode(&params)
                .map(AlbionOperation::ResponseRealEstateGetAuctionData)
        }
        OperationType::opRealEstateBidOnAuction => {
            OperationRealEstateBidOnAuctionResponse::decode(&params)
                .map(AlbionOperation::ResponseRealEstateBidOnAuction)
        }
        _ => None,
    }
}

pub fn decode_event(params: HashMap<u8, PhotonValue>) -> Option<AlbionOperation> {
    let code = resolve_event_code(&params)?;
    let ev_type = EventType(code);

    match ev_type {
        EventType::evCharacterStats => {
            EventCharacterStats::decode(&params).map(AlbionOperation::EventCharacterStats)
        }
        EventType::evRedZoneWorldMapEvent => EventRedZoneWorldMapEvent::decode(&params)
            .map(AlbionOperation::EventRedZoneWorldMapEvent),
        EventType::evFullAchievementInfo => {
            EventFullAchievementInfo::decode(&params).map(AlbionOperation::EventFullAchievementInfo)
        }
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
