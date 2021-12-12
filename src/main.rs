use anyhow::Result;
use pixoo_cli::device::Device;
use pixoo_cli::utils::{Color, Image};

fn main() -> Result<()> {
    let mut device = Device::connect("/dev/tty.Pixoo-SerialPort1")?;
    let image1 = Image::from(
        [[Color {
            r: 0xff,
            g: 44,
            b: 51,
        }; 16]; 16],
    );

    let image2 = Image::from(
        [[Color {
            r: 0xff,
            g: 0xff,
            b: 51,
        }; 16]; 16],
    );

    let image3 = Image::from(
        [[Color {
            r: 0xff,
            g: 0x00,
            b: 0xff,
        }; 16]; 16],
    );

    let animation = vec![image1, image2, image3];
    device.set_animation(animation)?;

    Ok(())
}
