use byteorder::{LittleEndian, WriteBytesExt};

pub struct Serialize {}

pub trait SerializeExt<T> {
    fn p16(value: T) -> Vec<u8>;
}

impl SerializeExt<u16> for Serialize {
    fn p16(value: u16) -> Vec<u8> {
        let mut result = vec![];
        result
            .write_u16::<LittleEndian>(value)
            .expect("Failed to pack value");
        result
    }
}

impl SerializeExt<u8> for Serialize {
    fn p16(value: u8) -> Vec<u8> {
        vec![value, 0]
    }
}

impl SerializeExt<usize> for Serialize {
    fn p16(value: usize) -> Vec<u8> {
        let mut result = vec![];
        result
            .write_u16::<LittleEndian>(value as u16)
            .expect("Failed to pack value");
        result
    }
}
