use crate::prelude::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum OpCode {
    Continuation = 0x0,
    Text = 0x1,
    Binary = 0x2,
    Close = 0x8,
    Ping = 0x9,
    Pong = 0xA,
}

impl From<u8> for OpCode {
    fn from(byte: u8) -> Self {
        match byte {
            0x0 => OpCode::Continuation,
            0x1 => OpCode::Text,
            0x2 => OpCode::Binary,
            0x8 => OpCode::Close,
            0x9 => OpCode::Ping,
            0xA => OpCode::Pong,
            _ => panic!("Unknown opcode: {}", byte),
        }
    }
}

pub struct SizedBuffer<'a>(pub &'a [u8], pub usize);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Frame {
    fin: bool,
    rsv1: bool,
    rsv2: bool,
    rsv3: bool,
    opcode: OpCode,
    mask: bool,
    payload_length: u64,
    masking_key: Option<[u8; 4]>,
    payload: Vec<u8>,
}

impl TryFrom<SizedBuffer<'_>> for Frame {
    type Error = anyhow::Error;

    fn try_from(SizedBuffer(buffer, bytes_read): SizedBuffer) -> Result<Self> {
        let (fin, rsv1, rsv2, rsv3, opcode) = parse_first_byte(buffer);
        let (mask, payload_length, payload_len_end_index) = mask_and_length(buffer)?;
        let masking_key = if mask {
            Some(parse_masking_key(buffer, payload_len_end_index))
        } else {
            None
        };
        let payload = parse_payload(buffer, payload_len_end_index, payload_length, masking_key);

        Ok(Frame {
            fin,
            rsv1,
            rsv2,
            rsv3,
            opcode,
            mask,
            payload_length,
            masking_key,
            payload,
        })
    }
}

impl TryInto<String> for Frame {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<String> {
        if self.opcode != OpCode::Text {
            return Err(anyhow!("Cannot convert non-text frame to String"));
        }
        String::from_utf8(self.payload).map_err(|e| anyhow!("Invalid UTF-8 sequence: {}", e))
    }
}

impl Frame {
    pub fn payload_as_text(self) -> Result<String> {
        self.try_into()
    }
}

fn parse_first_byte(buffer: &[u8]) -> (bool, bool, bool, bool, OpCode) {
    let first_byte = buffer[0];
    let fin = (first_byte & 0x80) != 0;
    let rsv1 = (first_byte & 0x40) != 0;
    let rsv2 = (first_byte & 0x20) != 0;
    let rsv3 = (first_byte & 0x10) != 0;
    let opcode = OpCode::from(first_byte & 0x0F);
    (fin, rsv1, rsv2, rsv3, opcode)
}

fn mask_and_length(buffer: &[u8]) -> Result<(bool, u64, usize)> {
    let second_byte = buffer[1];
    let mask = (second_byte & 0x80) != 0;
    let payload_length = (second_byte & 0x7F) as u64;
    match payload_length {
        0..126 => Ok((mask, payload_length, 2)),
        126 => {
            let length_bytes = &buffer[2..4];
            let payload_length = u16::from_be_bytes(length_bytes.try_into()?) as u64;
            Ok((mask, payload_length, 4))
        }
        127 => {
            let length_bytes = &buffer[2..10];
            let payload_length = u64::from_be_bytes(length_bytes.try_into()?);
            Ok((mask, payload_length, 10))
        }
        _ => Err(anyhow!("Payload length exceeds maximum size")),
    }
}

fn parse_masking_key(buffer: &[u8], start_at: usize) -> [u8; 4] {
    buffer[start_at..start_at + 4]
        .try_into()
        .expect("Masking key should be exactly 4 bytes")
}

fn parse_payload(
    buffer: &[u8],
    start_at: usize,
    payload_length: u64,
    masking_key: Option<[u8; 4]>,
) -> Vec<u8> {
    let payload_start = start_at + if masking_key.is_some() { 4 } else { 0 };
    let payload_end = payload_start + payload_length as usize;
    let payload = &buffer[payload_start..payload_end];

    if let Some(masking_key) = masking_key {
        payload
            .iter()
            .zip(masking_key.iter().cycle())
            .map(|(p, m)| p ^ m)
            .collect()
    } else {
        payload.to_vec()
    }
}
