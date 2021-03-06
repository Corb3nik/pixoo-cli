use bitvec::prelude::*;
use std::collections::HashMap;
use std::fmt;

use crate::utils::Color;

#[derive(Debug)]
pub struct Image {
    pub num_colors: usize,

    pixels: [[Color; 16]; 16],
    color_data: Vec<u8>,
    pixel_data: Vec<u8>,
}

impl Image {
    pub fn into_bytes(&self) -> Vec<u8> {
        let mut data = vec![];
        data.extend_from_slice(&self.color_data);
        data.extend_from_slice(&self.pixel_data);
        data
    }
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut map = HashMap::new();

        for row in self.pixels {
            for pixel in row {
                let new_index = map.len();
                let index = map.entry(pixel).or_insert(new_index);

                write!(f, "{} ", index);
            }
            write!(f, "\n");
        }

        Ok(())
    }
}

impl From<[[Color; 16]; 16]> for Image {
    fn from(pixels: [[Color; 16]; 16]) -> Image {
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
            let start_index = index * 3;
            let end_index = start_index + 3;
            color_data[start_index..end_index].copy_from_slice(&color.into_bytes());
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

        Image {
            pixels,
            color_data,
            pixel_data,
            num_colors: num_colors,
        }
    }
}
