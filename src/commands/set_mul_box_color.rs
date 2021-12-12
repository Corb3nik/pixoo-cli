use super::{Command, Commands, MultiCommand};
use crate::utils::{Image, Serialize, SerializeExt};

const FRAME_START: u8 = 0xaa;
const RESET_PALETTE: u8 = 0x0;
const PACKET_SIZE: usize = 200;
const ANIMATION_DELAY: usize = 1000;

pub struct SetMulBoxColor {
    pub images: Vec<Image>,
}

impl MultiCommand<SetMulBoxColorFrame> for SetMulBoxColor {
    fn commands(&self) -> Vec<SetMulBoxColorFrame> {
        let frames: Vec<AnimationFrame> = self.images.iter().map(AnimationFrame::new).collect();

        let mut all_frame_data = vec![];
        for frame_data in frames {
            all_frame_data.extend_from_slice(&frame_data.into_bytes());
        }

        // All data length
        let total_len = all_frame_data.len();

        // Generate commands
        let mut commands = vec![];
        for (packet_number, chunk) in all_frame_data.chunks(PACKET_SIZE).enumerate() {
            let command = SetMulBoxColorFrame {
                packet_number,
                total_len,
                data: chunk.to_vec(),
            };

            commands.push(command);
        }

        commands
    }
}

pub struct SetMulBoxColorFrame {
    packet_number: usize,
    total_len: usize,
    data: Vec<u8>,
}

impl Command for SetMulBoxColorFrame {
    fn into_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.push(Commands::SetMulBoxColor as u8);
        bytes.extend_from_slice(&Serialize::p16(self.total_len));
        bytes.push(self.packet_number as u8);
        bytes.extend_from_slice(&self.data);
        bytes
    }
}

struct AnimationFrame {
    timestamp_ms: usize,
    num_colors: usize,
    data: Vec<u8>,
}

impl AnimationFrame {
    fn new(image: &Image) -> Self {
        let data = image.into_bytes();
        let num_colors = image.num_colors;

        Self {
            timestamp_ms: ANIMATION_DELAY,
            data,
            num_colors,
        }
    }

    fn into_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![FRAME_START];
        let image_data = &self.data;

        // length
        // - 1 byte     (framestart)
        // - 2 bytes    (len)
        // - 2 bytes    (timestamp)
        // - 1 byte     (reset palette)
        // - 1 byte     (num colors)
        // - N bytes    (image data)
        let length = 7 + image_data.len();
        bytes.extend_from_slice(&Serialize::p16(length));

        // Timestamp
        bytes.extend_from_slice(&Serialize::p16(self.timestamp_ms));

        // Reset palette
        bytes.push(RESET_PALETTE);

        // Num colors
        bytes.push(self.num_colors as u8);

        // Image data
        bytes.extend_from_slice(&image_data);

        bytes
    }
}
