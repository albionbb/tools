use albion_packets::capture::{extract_payload, open_live_capture};
use albion_packets::operations::OperationType;
use albion_packets::{decode_event, decode_request, decode_response};
use photon_decoder::{PhotonListener, PhotonParser, PhotonValue};
use std::collections::HashMap;

const ALBION_PORT: u16 = 5056;

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
            params.insert(
                253,
                PhotonValue::Short(OperationType::opAuctionGetOffers.0 as i16),
            );
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=========================");
    println!("Albionbb Packet Inspector");
    println!("=========================");
    println!();

    let mut cap = match open_live_capture(ALBION_PORT) {
        Ok(cap) => cap,
        Err(e) if e.to_string().contains("Permission denied") => {
            eprintln!("Error: root privileges are required to capture network packets.");
            eprintln!("Try running with sudo.");
            std::process::exit(1);
        }
        Err(e) => return Err(e),
    };
    let link_type = cap.get_datalink().0 as u16;
    let mut parser = PhotonParser::new();
    let mut listener = AlbionListener;

    loop {
        match cap.next_packet() {
            Ok(packet) => {
                let data = packet.data;
                if let Some(payload) = extract_payload(link_type, data) {
                    parser.receive_packet(&payload, &mut listener);
                }
            }
            Err(pcap::Error::TimeoutExpired) => {
                continue;
            }
            Err(e) => {
                eprintln!("Capture error: {}", e);
                break;
            }
        }
    }

    Ok(())
}
