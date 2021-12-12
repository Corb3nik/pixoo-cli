use byteorder::{LittleEndian, WriteBytesExt};
use std::marker::PhantomData;

pub struct Serialize<T> {
    _marker: PhantomData<T>,
}

impl Serialize<u16> {
    pub fn p16(value: u16) -> Vec<u8> {
        let mut result = vec![];
        result
            .write_u16::<LittleEndian>(value)
            .expect("Failed to pack value");
        result
    }
}

impl Serialize<u8> {
    pub fn p16(value: u8) -> Vec<u8> {
        vec![value, 0]
    }
}
