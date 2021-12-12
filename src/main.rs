use anyhow::Result;
use pixoo_cli::device::Device;
use pixoo_cli::utils::{Color, Image};

fn main() -> Result<()> {
    let mut device = Device::connect("/dev/tty.Pixoo-SerialPort1")?;

    let image = Image::from(
        [[Color {
            r: 0xff,
            g: 0xff,
            b: 0xff,
        }; 16]; 16],
    );
    device.set_image(image)?;
    Ok(())
}
