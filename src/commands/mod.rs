pub use set_box_color::*;
pub use set_box_mode::*;
pub use set_brightness::*;

mod set_box_color;
pub mod set_box_mode;
mod set_brightness;

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
