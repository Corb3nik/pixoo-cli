use super::{Command, Commands};

pub struct SetBrightness {
    pub value: u8,
}

impl Command for SetBrightness {
    fn into_bytes(&self) -> Vec<u8> {
        vec![Commands::SetBrightness as u8, self.value]
    }
}
