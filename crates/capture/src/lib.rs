use pcap::{Active, Capture, Device};
use photon_decoder::{PhotonListener, PhotonParser};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, mpsc};
use std::thread::JoinHandle;

const ALBION_PORT: u16 = 5056;

pub struct CaptureHandle {
    shutdown: Arc<AtomicBool>,
    handles: Vec<JoinHandle<()>>,
}

impl CaptureHandle {
    pub fn stop(self) {
        self.shutdown.store(true, Ordering::Relaxed);
        for h in self.handles {
            h.join().ok();
        }
    }
}

pub fn start<L: PhotonListener + Send + 'static>(mut listener: L) -> ! {
    let devices = match find_physical_devices() {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Error discovering network devices: {}", e);
            std::process::exit(1);
        }
    };

    if devices.is_empty() {
        eprintln!("No suitable network interface found");
        std::process::exit(1);
    }

    let (tx, rx) = mpsc::channel();
    let shutdown = Arc::new(AtomicBool::new(false));
    let mut handles = Vec::with_capacity(devices.len());

    for dev in devices {
        let tx = tx.clone();
        let shutdown = Arc::clone(&shutdown);
        let name = dev.name.clone();

        match open_capture(dev) {
            Ok(cap) => {
                let link_type = cap.get_datalink().0 as u16;
                let handle = std::thread::spawn(move || {
                    run_capture(cap, link_type, tx, shutdown);
                });
                handles.push(handle);
            }
            Err(e) => {
                eprintln!(
                    "Warning: failed to open capture on {}: {}. Skipping.",
                    name, e
                );
            }
        }
    }

    if handles.is_empty() {
        eprintln!("No capture could be started on any interface");
        std::process::exit(1);
    }

    let mut parser = PhotonParser::new();
    for payload in rx {
        parser.receive_packet(&payload, &mut listener);
    }

    // Channel closed — all capture threads have died.
    std::process::exit(1);
}

fn run_capture(
    mut cap: Capture<Active>,
    link_type: u16,
    tx: mpsc::Sender<Vec<u8>>,
    shutdown: Arc<AtomicBool>,
) {
    while !shutdown.load(Ordering::Relaxed) {
        match cap.next_packet() {
            Ok(packet) => {
                if let Some(payload) = extract_payload(link_type, packet.data)
                    && tx.send(payload).is_err()
                {
                    break;
                }
            }
            Err(pcap::Error::TimeoutExpired) => continue,
            Err(e) => {
                eprintln!("Capture error on thread: {}", e);
                break;
            }
        }
    }
}

fn find_physical_devices() -> Result<Vec<Device>, Box<dyn std::error::Error>> {
    let all = Device::list()?;
    let mut out = Vec::new();

    for dev in all {
        let name_lower = dev.name.to_lowercase();
        if name_lower == "lo" || name_lower == "lo0" {
            continue;
        }
        if dev.addresses.is_empty() {
            continue;
        }
        out.push(dev);
    }

    Ok(out)
}

fn open_capture(device: Device) -> Result<Capture<Active>, pcap::Error> {
    let mut cap = Capture::from_device(device)?
        .promisc(false)
        .snaplen(65535)
        .timeout(100)
        .open()?;

    let filter = format!("udp port {} or tcp port {}", ALBION_PORT, ALBION_PORT);
    cap.filter(&filter, true)?;
    Ok(cap)
}

pub fn extract_payload(link_type: u16, packet: &[u8]) -> Option<Vec<u8>> {
    let ip_offset = match link_type {
        1 => 14,
        228 => 16,
        101 => 0,
        _ => {
            if packet.len() > 14 && packet[12..14] == [0x08, 0x00] {
                14
            } else if packet.len() > 4 && packet[0] == 0x02 {
                16
            } else {
                0
            }
        }
    };

    if packet.len() < ip_offset + 20 {
        return None;
    }

    let ip_data = &packet[ip_offset..];
    let version_ihl = ip_data[0];
    let ihl = (version_ihl & 0x0F) as usize * 4;

    if ip_data.len() < ihl {
        return None;
    }

    let protocol = ip_data[9];
    let total_len = u16::from_be_bytes([ip_data[2], ip_data[3]]) as usize;
    let ip_payload = &ip_data[ihl..total_len.min(ip_data.len())];

    match protocol {
        17 => {
            if ip_payload.len() < 8 {
                return None;
            }
            let udp_len = u16::from_be_bytes([ip_payload[4], ip_payload[5]]) as usize;
            Some(ip_payload[8..udp_len.min(ip_payload.len())].to_vec())
        }
        6 => {
            if ip_payload.len() < 20 {
                return None;
            }
            let tcp_header_len = ((ip_payload[12] >> 4) & 0x0F) as usize * 4;
            Some(ip_payload[tcp_header_len..].to_vec())
        }
        _ => None,
    }
}
