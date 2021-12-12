use super::{Command, Commands};
use crate::utils::{Image, Serialize, SerializeExt};

const FRAME_START: u8 = 0xaa;

pub struct SetBoxColor {
    pub image: Image,
}

impl Command for SetBoxColor {
    fn into_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![Commands::SetBoxColor as u8];

        // Fixed data
        bytes.extend_from_slice(&[0x00, 0x0a, 0x0a, 0x04, FRAME_START]);

        // Image data
        let image_data = self.image.into_vec();

        // Image data length
        let image_data_len = Serialize::p16(7 + image_data.len());
        bytes.extend_from_slice(&image_data_len);

        // Fixed data
        bytes.extend_from_slice(&[0x00, 0x00, 0x00]);

        // Number of colors
        bytes.push(self.image.num_colors as u8);

        // Image data
        bytes.extend_from_slice(&image_data);

        bytes
    }
}
