use crate::structs::*;
use crate::types::{EvCode, OpCode};
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
    let op_code = OpCode(code);

    match op_code {
        OpCode::GetGameServerByCluster => OperationGetGameServerByCluster::decode(&params)
            .map(AlbionOperation::RequestGetGameServerByCluster),
        OpCode::AuctionGetOffers => {
            OperationAuctionGetOffers::decode(&params).map(AlbionOperation::RequestAuctionGetOffers)
        }
        OpCode::AuctionGetItemAverageStats => OperationAuctionGetItemAverageStats::decode(&params)
            .map(AlbionOperation::RequestAuctionGetItemAverageStats),
        OpCode::GetClusterMapInfo => OperationGetClusterMapInfo::decode(&params)
            .map(AlbionOperation::RequestGetClusterMapInfo),
        OpCode::GoldMarketGetAverageInfo => OperationGoldMarketGetAverageInfo::decode(&params)
            .map(AlbionOperation::RequestGoldMarketGetAverageInfo),
        OpCode::RealEstateGetAuctionData => OperationRealEstateGetAuctionData::decode(&params)
            .map(AlbionOperation::RequestRealEstateGetAuctionData),
        OpCode::RealEstateBidOnAuction => OperationRealEstateBidOnAuction::decode(&params)
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
    let op_code = OpCode(code);

    match op_code {
        OpCode::Join => OperationJoinResponse::decode(&params).map(AlbionOperation::ResponseJoin),
        OpCode::AuctionGetOffers => OperationAuctionGetOffersResponse::decode(&params)
            .map(AlbionOperation::ResponseAuctionGetOffers),
        OpCode::AuctionGetRequests => OperationAuctionGetRequestsResponse::decode(&params)
            .map(AlbionOperation::ResponseAuctionGetRequests),
        OpCode::AuctionGetItemAverageStats => {
            OperationAuctionGetItemAverageStatsResponse::decode(&params)
                .map(AlbionOperation::ResponseAuctionGetItemAverageStats)
        }
        OpCode::GetMailInfos => OperationGetMailInfosResponse::decode(&params)
            .map(AlbionOperation::ResponseGetMailInfos),
        OpCode::ReadMail => {
            OperationReadMailResponse::decode(&params).map(AlbionOperation::ResponseReadMail)
        }
        OpCode::GetClusterMapInfo => OperationGetClusterMapInfoResponse::decode(&params)
            .map(AlbionOperation::ResponseGetClusterMapInfo),
        OpCode::GoldMarketGetAverageInfo => {
            OperationGoldMarketGetAverageInfoResponse::decode(&params)
                .map(AlbionOperation::ResponseGoldMarketGetAverageInfo)
        }
        OpCode::RealEstateGetAuctionData => {
            OperationRealEstateGetAuctionDataResponse::decode(&params)
                .map(AlbionOperation::ResponseRealEstateGetAuctionData)
        }
        OpCode::RealEstateBidOnAuction => OperationRealEstateBidOnAuctionResponse::decode(&params)
            .map(AlbionOperation::ResponseRealEstateBidOnAuction),
        _ => None,
    }
}

pub fn decode_event(params: HashMap<u8, PhotonValue>) -> Option<AlbionOperation> {
    let code = resolve_event_code(&params)?;
    let ev_code = EvCode(code);

    match ev_code {
        EvCode::CharacterStats => {
            EventCharacterStats::decode(&params).map(AlbionOperation::EventCharacterStats)
        }
        EvCode::RedZoneWorldMapEvent => EventRedZoneWorldMapEvent::decode(&params)
            .map(AlbionOperation::EventRedZoneWorldMapEvent),
        EvCode::FullAchievementInfo => {
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
