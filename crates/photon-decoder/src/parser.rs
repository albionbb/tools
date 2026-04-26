use super::deserializer::{PhotonValue, deserialize, deserialize_parameter_table};
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use std::collections::HashMap;
use std::io;

const PHOTON_HEADER_LENGTH: usize = 12;
const COMMAND_HEADER_LENGTH: u32 = 12;
const FRAGMENT_HEADER_LENGTH: usize = 20;

const CMD_DISCONNECT: u8 = 4;
const CMD_SEND_RELIABLE: u8 = 6;
const CMD_SEND_UNRELIABLE: u8 = 7;
const CMD_SEND_FRAGMENT: u8 = 8;

const MSG_REQUEST: u8 = 2;
const MSG_RESPONSE: u8 = 3;
const MSG_EVENT: u8 = 4;
const MSG_RESPONSE_ALT: u8 = 7;
const MSG_ENCRYPTED: u8 = 131;

struct SegmentedPackage {
    total_length: usize,
    bytes_written: usize,
    payload: Vec<u8>,
}

/// Callbacks for parsed Photon messages.
pub trait PhotonListener {
    fn on_request(&mut self, operation_code: u8, params: HashMap<u8, PhotonValue>);
    fn on_response(
        &mut self,
        operation_code: u8,
        return_code: i16,
        debug_message: String,
        params: HashMap<u8, PhotonValue>,
    );
    fn on_event(&mut self, code: u8, params: HashMap<u8, PhotonValue>);
    fn on_encrypted(&mut self);
}

/// Parses raw Photon UDP/TCP payloads and fires callbacks.
pub struct PhotonParser {
    pending_segments: HashMap<u32, SegmentedPackage>,
}

impl Default for PhotonParser {
    fn default() -> Self {
        Self::new()
    }
}

impl PhotonParser {
    pub fn new() -> Self {
        Self {
            pending_segments: HashMap::new(),
        }
    }

    pub fn receive_packet(&mut self, payload: &[u8], listener: &mut dyn PhotonListener) {
        if payload.len() < PHOTON_HEADER_LENGTH {
            return;
        }

        let mut offset = 2; // skip peerId
        let flags = payload[offset];
        offset += 1;
        let command_count = payload[offset] as usize;
        offset += 1;
        offset += 8; // skip timestamp + challenge

        if flags == 1 {
            listener.on_encrypted();
            return;
        }

        for _ in 0..command_count {
            let (new_offset, ok) = self.handle_command(payload, offset, listener);
            if !ok {
                return;
            }
            offset = new_offset;
        }
    }

    fn handle_command(
        &mut self,
        src: &[u8],
        offset: usize,
        listener: &mut dyn PhotonListener,
    ) -> (usize, bool) {
        if !available(src, offset, COMMAND_HEADER_LENGTH as usize) {
            return (offset, false);
        }

        let cmd_type = src[offset];
        let mut off = offset + 4; // skip cmdType, channelId, flags, reserved
        let cmd_len_raw = {
            let mut cursor = io::Cursor::new(&src[off..off + 4]);
            cursor.read_u32::<BigEndian>().unwrap_or(0)
        };
        off += 8; // length + seqNum

        if cmd_len_raw < COMMAND_HEADER_LENGTH {
            return (off, false);
        }
        let data_len = (cmd_len_raw - COMMAND_HEADER_LENGTH) as usize;

        if !available(src, off, data_len) {
            return (off, false);
        }

        match cmd_type {
            CMD_DISCONNECT => (off + data_len, true),
            CMD_SEND_UNRELIABLE => {
                if data_len < 4 {
                    return (off + data_len, false);
                }
                let new_off = off + 4;
                let new_len = data_len - 4;
                let (end, _) = self.handle_send_reliable(src, new_off, new_len, listener);
                (end, true)
            }
            CMD_SEND_RELIABLE => {
                let (end, _) = self.handle_send_reliable(src, off, data_len, listener);
                (end, true)
            }
            CMD_SEND_FRAGMENT => {
                let end = self.handle_send_fragment(src, off, data_len, listener);
                (end, true)
            }
            _ => (off + data_len, true),
        }
    }

    fn handle_send_reliable(
        &mut self,
        src: &[u8],
        offset: usize,
        cmd_len: usize,
        listener: &mut dyn PhotonListener,
    ) -> (usize, bool) {
        if cmd_len < 2 || !available(src, offset, cmd_len) {
            return (offset + cmd_len, false);
        }

        let mut off = offset;
        // signalByte := src[off];
        off += 1;
        let msg_type = src[off];
        off += 1;
        let data_len = cmd_len - 2;

        if !available(src, off, data_len) {
            return (off + data_len, false);
        }

        if msg_type == MSG_ENCRYPTED {
            listener.on_encrypted();
            return (off + data_len, true);
        }

        let data = &src[off..(off + data_len)];
        off += data_len;

        match msg_type {
            MSG_REQUEST => self.dispatch_request(data, listener),
            MSG_RESPONSE | MSG_RESPONSE_ALT => self.dispatch_response(data, listener),
            MSG_EVENT => self.dispatch_event(data, listener),
            _ => {}
        }

        (off, true)
    }

    fn dispatch_request(&self, data: &[u8], listener: &mut dyn PhotonListener) {
        if data.is_empty() {
            return;
        }
        let op_code = data[0];
        let params = deserialize_parameter_table(&data[1..]);
        listener.on_request(op_code, params);
    }

    fn dispatch_response(&self, data: &[u8], listener: &mut dyn PhotonListener) {
        if data.len() < 3 {
            return;
        }
        let op_code = data[0];
        let return_code = {
            let mut cursor = io::Cursor::new(&data[1..3]);
            cursor.read_i16::<LittleEndian>().unwrap_or(0)
        };

        let mut cursor = io::Cursor::new(&data[3..]);
        let mut debug_message = String::new();

        if cursor.position() < cursor.get_ref().len() as u64 {
            let tc = cursor.read_u8().unwrap_or(0);
            let val = deserialize(&mut cursor, tc);
            match val {
                PhotonValue::String(s) => debug_message = s,
                PhotonValue::Array(arr) => {
                    // Check if it's a typed string array (market data special case)
                    let all_strings = arr.iter().all(|v| matches!(v, PhotonValue::String(_)));
                    if all_strings {
                        let mut params = HashMap::new();
                        params.insert(0, PhotonValue::Array(arr));
                        listener.on_response(op_code, return_code, String::new(), params);
                        return;
                    }
                }
                _ => {}
            }
        }

        let params = super::deserializer::read_parameter_table(&mut cursor);
        listener.on_response(op_code, return_code, debug_message, params);
    }

    fn dispatch_event(&self, data: &[u8], listener: &mut dyn PhotonListener) {
        if data.is_empty() {
            return;
        }
        let code = data[0];
        let params = deserialize_parameter_table(&data[1..]);
        listener.on_event(code, params);
    }

    fn handle_send_fragment(
        &mut self,
        src: &[u8],
        offset: usize,
        cmd_len: usize,
        listener: &mut dyn PhotonListener,
    ) -> usize {
        if cmd_len < FRAGMENT_HEADER_LENGTH || !available(src, offset, FRAGMENT_HEADER_LENGTH) {
            return offset + cmd_len;
        }

        let mut off = offset;
        let start_seq = {
            let mut cursor = io::Cursor::new(&src[off..off + 4]);
            cursor.read_u32::<BigEndian>().unwrap_or(0)
        };
        off += 4;
        // fragmentCount
        off += 4;
        // fragmentNumber
        off += 4;
        let total_len = {
            let mut cursor = io::Cursor::new(&src[off..off + 4]);
            cursor.read_u32::<BigEndian>().unwrap_or(0) as usize
        };
        off += 4;
        let frag_offset = {
            let mut cursor = io::Cursor::new(&src[off..off + 4]);
            cursor.read_u32::<BigEndian>().unwrap_or(0) as usize
        };
        off += 4;

        let frag_len = cmd_len - FRAGMENT_HEADER_LENGTH;
        if !available(src, off, frag_len) {
            return off + frag_len;
        }

        let seg = self
            .pending_segments
            .entry(start_seq)
            .or_insert_with(|| SegmentedPackage {
                total_length: total_len,
                bytes_written: 0,
                payload: vec![0u8; total_len],
            });

        let end = frag_offset + frag_len;
        if end <= seg.payload.len() {
            seg.payload[frag_offset..end].copy_from_slice(&src[off..off + frag_len]);
        }
        seg.bytes_written += frag_len;

        if seg.bytes_written >= seg.total_length {
            let payload = self.pending_segments.remove(&start_seq).unwrap().payload;
            let _ = self.handle_send_reliable(&payload, 0, payload.len(), listener);
        }

        off + frag_len
    }
}

fn available(src: &[u8], offset: usize, count: usize) -> bool {
    count == 0 || (offset <= src.len() && src.len() - offset >= count)
}
