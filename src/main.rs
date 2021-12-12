use anyhow::Result;
use pixoo_cli::device::Device;
use pixoo_cli::utils::{Color, Image};

fn main() -> Result<()> {
    let mut device = Device::connect("/dev/tty.Pixoo-SerialPort1")?;

    let animation = vec![
        Image::from(
            [[Color {
                r: 255,
                g: 255,
                b: 255,
            }; 16]; 16],
        ),
        Image::from(
            [[Color {
                r: 0,
                g: 255,
                b: 255,
            }; 16]; 16],
        ),
        Image::from([[Color { r: 0, g: 255, b: 0 }; 16]; 16]),
        Image::from([[Color { r: 0, g: 0, b: 255 }; 16]; 16]),
        Image::from([[Color { r: 255, g: 0, b: 0 }; 16]; 16]),
        Image::from(
            [[Color {
                r: 255,
                g: 0,
                b: 255,
            }; 16]; 16],
        ),
        Image::from(
            [[Color {
                r: 50,
                g: 100,
                b: 50,
            }; 16]; 16],
        ),
        Image::from(
            [[Color {
                r: 105,
                g: 130,
                b: 58,
            }; 16]; 16],
        ),
        Image::from(
            [[Color {
                r: 205,
                g: 120,
                b: 98,
            }; 16]; 16],
        ),
        Image::from(
            [[Color {
                r: 205,
                g: 120,
                b: 00,
            }; 16]; 16],
        ),
    ];
    device.set_animation(animation)?;

    Ok(())
}
