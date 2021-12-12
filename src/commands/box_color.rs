use bitvec::prelude::*;
use std::collections::HashMap;

use super::{Command, Commands};
use crate::utils::{Color, Serialize, SerializeExt};

pub struct SetBoxColor {
    pub pixels: [[Color; 16]; 16],
}

impl Command for SetBoxColor {
    fn into_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![Commands::SetBoxColor as u8];

        // Fixed data
        bytes.extend_from_slice(&[0x00, 0x0a, 0x0a, 0x04, 0xaa]);

        // Image data
        let image_data = ImageData::from(self);
        let image_data_vec = image_data.into_vec();

        // Image data length
        let image_data_len = Serialize::p16(7 + image_data_vec.len());
        bytes.extend_from_slice(&image_data_len);

        // Fixed data
        bytes.extend_from_slice(&[0x00, 0x00, 0x00]);

        // Number of colors
        bytes.push(image_data.num_colors as u8);

        // Image data
        bytes.extend_from_slice(&image_data_vec);

        bytes
    }
}

#[derive(Debug)]
struct ImageData {
    pub num_colors: usize,

    color_data: Vec<u8>,
    pixel_data: Vec<u8>,
}

impl ImageData {
    fn into_vec(&self) -> Vec<u8> {
        let mut data = vec![];
        data.extend_from_slice(&self.color_data);
        data.extend_from_slice(&self.pixel_data);
        data
    }
}

impl From<&SetBoxColor> for ImageData {
    fn from(val: &SetBoxColor) -> ImageData {
        let pixels = &val.pixels;

        let mut color_map = HashMap::new();
        let mut pixel_array = vec![];

        // Collect colors and indexes
        for row in pixels {
            for color in row {
                let next_index = color_map.len();
                let index = color_map.entry(color).or_insert(next_index);
                pixel_array.push(*index);
            }
        }

        // Serialize colors
        let num_colors = color_map.len();
        let color_data_len = num_colors * 3;
        let mut color_data = vec![];
        color_data.resize(color_data_len, 0u8);
        for (color, index) in color_map.iter() {
            color_data[*index..*index + 3].copy_from_slice(&color.into_bytes());
        }

        // Serialize pixels
        let bit_length = (num_colors as f64 + 1.0).log(2.0).ceil() as usize;
        let capacity = bit_length * pixel_array.len();
        let mut bitvec = BitVec::<Lsb0, u8>::with_capacity(capacity);
        bitvec.resize(capacity, false);

        for (i, color_index) in pixel_array.iter().enumerate() {
            bitvec[i..(i + bit_length)].store(*color_index)
        }

        let pixel_data = bitvec.into_vec();

        ImageData {
            color_data,
            pixel_data,
            num_colors: num_colors,
        }
    }
}
