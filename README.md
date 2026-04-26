# albionbb/tools

Rust tools for capturing and decoding Albion Online network traffic.

## Crates

| Crate | Description |
|-------|-------------|
| `achievement-exporter` | Captures achievement data and exports it as JSON |
| `albion-packets` | Albion Online packet decoding + live packet capture |
| `inspector` | CLI that prints decoded operations and events to stdout |
| `photon-decoder` | Photon Protocol 18 parser and deserializer |

## Requirements

- [Rust](https://rustup.rs/) toolchain (edition 2024)
- `libpcap` system library:
  - **macOS**: pre-installed
  - **Linux**: `sudo apt install libpcap-dev`
  - **Windows**: [Npcap](https://npcap.com/) with SDK
- Root/Administrator privileges for live packet capture

## Usage

```bash
# Run the achievement exporter
sudo cargo run -p achievement-exporter

# Run the packet inspector
sudo cargo run -p inspector
```

## Credits

- https://github.com/ao-data/albiondata-client
