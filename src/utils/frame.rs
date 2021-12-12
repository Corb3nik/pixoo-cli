use std::num::Wrapping;

use crate::commands::Command;
use crate::utils::Serialize;

const HEADER: u8 = 0x1;
const FOOTER: u8 = 0x2;
const CRC_LENGTH: usize = 0x2;

pub struct Frame<T: Command> {
    inner: T,
}

impl<T: Command> Frame<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }

    pub fn into_bytes(&self) -> Vec<u8> {
        let mut frame = vec![];

        // Header
        frame.push(HEADER);

        let payload = self.inner.into_bytes();

        // Length
        let length_i = payload.len() as u8 + CRC_LENGTH as u8;
        let length = Serialize::<u8>::p16(length_i);
        frame.extend_from_slice(&length);

        // Payload
        frame.extend_from_slice(&payload);

        // CRC
        let crc_i = crc(&frame[1..]);
        let crc = Serialize::<u16>::p16(crc_i);
        frame.extend_from_slice(&crc);

        // Footer
        frame.push(FOOTER);

        frame
    }
}

fn crc(data: &[u8]) -> u16 {
    let mut sum = Wrapping(0u16);
    for byte in data {
        sum += Wrapping(*byte as u16)
    }

    sum.0
}
