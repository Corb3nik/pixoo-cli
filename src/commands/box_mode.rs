use super::{Command, Commands};

pub enum Mode {
    Time,
}

pub struct SetBoxMode {
    pub mode: Mode,
}

impl Command for SetBoxMode {
    fn into_bytes(&self) -> Vec<u8> {
        match self.mode {
            Mode::Time => {
                vec![
                    Commands::SetBoxMode as u8,
                    0x00,
                    0x01,
                    0x01,
                    0x00,
                    0x00,
                    0x00,
                    0x00,
                    0xf5,
                    0x7f,
                    0x75,
                ]
            }
            _ => panic!("Mode is not implemented"),
        }
    }
}
