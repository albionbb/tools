mod achievements;

use achievements::ACHIEVEMENTS;
use albion_packets::structs::EventFullAchievementInfo;
use albion_packets::{AlbionOperation, decode_event};
use photon_decoder::{PhotonListener, PhotonValue};
use std::collections::BTreeMap;
use std::fs;
use std::process::Command;

const OUTPUT_FILE: &str = "specs_export.json";

struct AchievementListener;

impl PhotonListener for AchievementListener {
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

fn main() {
    println!("=======================");
    println!("Albionbb Specs Exporter");
    println!("=======================");
    println!("Launch Albion Online or travel to a new zone to trigger the export.");
    println!();

    capture::start(AchievementListener);
}
