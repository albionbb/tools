/// Decodes a mixed-endian UUID from bytes (Albion's CharacterID format).
/// First 4 bytes LE, next 2 bytes LE, next 2 bytes LE, remaining 8 bytes BE.
pub fn decode_character_id(bytes: &[u8]) -> String {
    if bytes.len() != 16 {
        return format!("{:x?}", bytes);
    }
    let mut b = [0u8; 16];
    b.copy_from_slice(bytes);

    // Swap first 4 bytes (little-endian → big-endian for UUID display)
    b.swap(0, 3);
    b.swap(1, 2);
    // Swap next 2 bytes
    b.swap(4, 5);
    // Swap next 2 bytes
    b.swap(6, 7);

    format!(
        "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        b[0],
        b[1],
        b[2],
        b[3],
        b[4],
        b[5],
        b[6],
        b[7],
        b[8],
        b[9],
        b[10],
        b[11],
        b[12],
        b[13],
        b[14],
        b[15]
    )
}

const MAX_OPCODE: u16 = 539;
const MAX_EVCODE: u16 = 680;

pub fn normalize_operation_code(code: u16) -> u16 {
    if code <= MAX_OPCODE {
        return code;
    }
    let swapped = code.rotate_left(8);
    if swapped <= MAX_OPCODE {
        return swapped;
    }
    if code > 0x00FF && code & 0x00FF == 0 {
        let shifted = code >> 8;
        if shifted <= MAX_OPCODE {
            return shifted;
        }
    }
    code
}

pub fn normalize_event_code(code: u16) -> u16 {
    if code <= MAX_EVCODE {
        return code;
    }
    let swapped = code.rotate_left(8);
    if swapped <= MAX_EVCODE {
        return swapped;
    }
    if code > 0x00FF && code & 0x00FF == 0 {
        let shifted = code >> 8;
        if shifted <= MAX_EVCODE {
            return shifted;
        }
    }
    code
}
