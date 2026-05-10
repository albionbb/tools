use albion_packets::codes::OpCode;
use albion_packets::{decode_event, decode_request, decode_response};
use photon_decoder::{PhotonListener, PhotonValue};
use std::collections::HashMap;

struct AlbionListener;

impl PhotonListener for AlbionListener {
    fn on_request(&mut self, operation_code: u8, params: HashMap<u8, PhotonValue>) {
        let mut params = params;
        params
            .entry(253)
            .or_insert(PhotonValue::Short(operation_code as i16));

        if let Some(op) = decode_request(params) {
            println!("[REQUEST] {:?}", op);
        }
    }

    fn on_response(
        &mut self,
        operation_code: u8,
        return_code: i16,
        debug_message: String,
        params: HashMap<u8, PhotonValue>,
    ) {
        let mut params = params;
        params
            .entry(253)
            .or_insert(PhotonValue::Short(operation_code as i16));

        if let Some(PhotonValue::Array(arr)) = params.get(&0)
            && arr.iter().all(|v| matches!(v, PhotonValue::String(_)))
        {
            params.insert(253, PhotonValue::Short(OpCode::AuctionGetOffers.0 as i16));
        }

        if let Some(op) = decode_response(params, return_code, debug_message.clone()) {
            println!("[RESPONSE] {:?}", op);
        }
    }

    fn on_event(&mut self, operation_code: u8, params: HashMap<u8, PhotonValue>) {
        let mut params = params;
        params
            .entry(252)
            .or_insert(PhotonValue::Short(operation_code as i16));

        if let Some(op) = decode_event(params) {
            println!("[EVENT] {:?}", op);
        }
    }

    fn on_encrypted(&mut self) {
        println!("[ENCRYPTED] Packet encrypted, skipping...");
    }
}

fn main() {
    println!("=========================");
    println!("Albionbb Packet Inspector");
    println!("=========================");
    println!();

    capture::start(AlbionListener);
}
