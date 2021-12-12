pub use box_color::*;
pub use box_mode::*;
pub use brightness::*;

mod box_color;
pub mod box_mode;
mod brightness;

pub enum Commands {
    SetBoxColor = 0x44,
    SetBoxMode = 0x45,
    SetBrightness = 0x74,
}

pub trait Command {
    fn into_bytes(&self) -> Vec<u8>;
}

impl Command for &[u8] {
    fn into_bytes(&self) -> Vec<u8> {
        self.to_vec()
    }
}
