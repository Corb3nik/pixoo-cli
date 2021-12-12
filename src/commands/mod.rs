pub use box_mode::*;
pub use brightness::*;

pub mod box_mode;
mod brightness;

pub enum Commands {
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
