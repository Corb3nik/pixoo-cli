pub use error::DeviceError;
pub use serialport::SerialPort;

use crate::commands::box_mode::Mode;
use crate::commands::{Command, SetBoxMode, SetBrightness};
use crate::utils::Frame;

mod error;

type Result<T> = std::result::Result<T, DeviceError>;

pub struct Device {
    inner: Box<dyn SerialPort>,
}

const BAUD_RATE: u32 = 115_200;

impl Device {
    pub fn connect(path: &str) -> Result<Self> {
        let inner = serialport::new(path, BAUD_RATE).open()?;

        Ok(Self { inner })
    }
    pub fn set_brightness(&mut self, value: u8) {
        let command = SetBrightness { value };
        self.send_command(command);
    }

    pub fn show_time(&mut self) {
        let mode = Mode::Time;
        let command = SetBoxMode { mode };
        self.send_command(command);
    }

    fn send_command<T: Command>(&mut self, command: T) {
        let frame = Frame::new(command);
        let data = frame.into_bytes();
        self.inner.write(&data);
    }
}
