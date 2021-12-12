pub use error::DeviceError;
pub use serialport::SerialPort;

use crate::commands::set_box_mode::Mode;
use crate::commands::{
    Command, MultiCommand, SetBoxColor, SetBoxMode, SetBrightness, SetMulBoxColor,
};
use crate::utils::{Frame, Image};

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

    // Change brightness setting
    // Value should be between 0 and 100
    pub fn set_brightness(&mut self, value: u8) -> Result<()> {
        let command = SetBrightness { value };
        self.send_command(command)?;
        Ok(())
    }

    // Show the time
    pub fn show_time(&mut self) -> Result<()> {
        let mode = Mode::Time;
        let command = SetBoxMode { mode };
        self.send_command(command)?;
        Ok(())
    }

    // Send a raw payload
    // Payload argument shouldn't contain any header/footer/CRC
    pub fn send_raw(&mut self, raw: &[u8]) -> Result<()> {
        self.send_command(raw)?;
        Ok(())
    }

    // Set an image
    pub fn set_image(&mut self, image: Image) -> Result<()> {
        let command = SetBoxColor { image };
        self.send_command(command)?;
        Ok(())
    }

    // Set an animation
    pub fn set_animation(&mut self, images: Vec<Image>) -> Result<()> {
        let generator = SetMulBoxColor { images };
        for command in generator.commands() {
            self.send_command(command)?;
        }
        Ok(())
    }

    fn send_command<T: Command>(&mut self, command: T) -> Result<()> {
        let frame = Frame::new(command);
        let data = frame.into_bytes();
        println!("data: {:?}", data);
        self.inner.write(&data)?;
        Ok(())
    }
}
