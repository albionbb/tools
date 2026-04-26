use pcap::{Active, Capture, Device};

pub fn open_live_capture(port: u16) -> Result<Capture<Active>, Box<dyn std::error::Error>> {
    let device = find_active_device()?;

    let mut cap = Capture::from_device(device)?
        .promisc(false)
        .snaplen(65535)
        .timeout(100)
        .open()?;

    let filter = format!("udp port {} or tcp port {}", port, port);
    cap.filter(&filter, true)?;

    Ok(cap)
}

fn find_active_device() -> Result<Device, Box<dyn std::error::Error>> {
    let devices = Device::list()?;
    for dev in &devices {
        if dev.name == "lo0" || dev.name == "lo" {
            continue;
        }
        if !dev.addresses.is_empty() {
            return Ok(dev.clone());
        }
    }

    if let Ok(Some(dev)) = Device::lookup() {
        return Ok(dev);
    }

    for dev in devices {
        if dev.name != "lo0" && dev.name != "lo" {
            return Ok(dev);
        }
    }

    Err("No suitable network interface found".into())
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
