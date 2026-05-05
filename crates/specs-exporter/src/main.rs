mod achievements;

use achievements::ACHIEVEMENTS;
use albion_packets::capture::{extract_payload, open_live_capture};
use albion_packets::structs::EventFullAchievementInfo;
use albion_packets::{AlbionOperation, decode_event};
use photon_decoder::{PhotonListener, PhotonParser, PhotonValue};
use std::collections::BTreeMap;
use std::fs;
use std::process::Command;

const ALBION_PORT: u16 = 5056;
const OUTPUT_FILE: &str = "specs_export.json";

struct AchievementListener;

impl PhotonListener for AchievementListener {
    fn on_request(
        &mut self,
        _operation_code: u8,
        _params: std::collections::HashMap<u8, PhotonValue>,
    ) {
    }
    fn on_response(
        &mut self,
        _operation_code: u8,
        _return_code: i16,
        _debug_message: String,
        _params: std::collections::HashMap<u8, PhotonValue>,
    ) {
    }
    fn on_encrypted(&mut self) {}

    fn on_event(&mut self, code: u8, params: std::collections::HashMap<u8, PhotonValue>) {
        let mut params = params;
        params.entry(252).or_insert(PhotonValue::Short(code as i16));

        if let Some(AlbionOperation::EventFullAchievementInfo(ev)) = decode_event(params) {
            match export_achievements(&ev) {
                Ok(count) => {
                    println!("Exported {} achievements to {}", count, OUTPUT_FILE);
                    std::process::exit(0);
                }
                Err(e) => {
                    eprintln!("Error writing {}: {}", OUTPUT_FILE, e);
                    std::process::exit(1);
                }
            }
        }
    }
}

fn export_achievements(ev: &EventFullAchievementInfo) -> Result<usize, Box<dyn std::error::Error>> {
    let mut map = BTreeMap::new();

    for &id in &ev.completed_achievement_ids {
        let idx = id as usize;
        if idx >= ACHIEVEMENTS.len() {
            eprintln!(
                "Warning: completed achievement id {} out of bounds, skipping",
                id
            );
            continue;
        }
        map.insert(ACHIEVEMENTS[idx].0.to_string(), ACHIEVEMENTS[idx].1);
    }

    for (i, &id) in ev.active_achievement_ids.iter().enumerate() {
        let idx = id as usize;
        if idx >= ACHIEVEMENTS.len() {
            eprintln!(
                "Warning: active achievement id {} out of bounds, skipping",
                id
            );
            continue;
        }
        let level = ev.active_achievement_levels.get(i).copied().unwrap_or(0);
        if level == 0 {
            continue;
        }
        map.insert(ACHIEVEMENTS[idx].0.to_string(), level);
    }

    let count = map.len();
    let json = serde_json::to_string_pretty(&map)?;
    fs::write(OUTPUT_FILE, json)?;

    if let Ok(user) = std::env::var("SUDO_USER") {
        Command::new("chown")
            .args([&user, OUTPUT_FILE])
            .status()
            .ok();
    }

    Ok(count)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=======================");
    println!("Albionbb Specs Exporter");
    println!("=======================");
    println!("Launch Albion Online or travel to a new zone to trigger the export.");
    println!();

    let mut cap = match open_live_capture(ALBION_PORT) {
        Ok(cap) => cap,
        Err(e) if e.to_string().contains("Permission denied") => {
            println!("Error: root privileges are required to capture network packets.");
            println!("Try running with sudo.");
            std::process::exit(1);
        }
        Err(e) => return Err(e),
    };
    let link_type = cap.get_datalink().0 as u16;
    let mut parser = PhotonParser::new();
    let mut listener = AchievementListener;

    loop {
        match cap.next_packet() {
            Ok(packet) => {
                let data = packet.data;
                if let Some(payload) = extract_payload(link_type, data) {
                    parser.receive_packet(&payload, &mut listener);
                }
            }
            Err(pcap::Error::TimeoutExpired) => continue,
            Err(e) => {
                eprintln!("Capture error: {}", e);
                break;
            }
        }
    }

    Ok(())
}
