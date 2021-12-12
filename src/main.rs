use anyhow::Result;
use pixoo_cli::commands::Color;
use pixoo_cli::device::Device;

fn main() -> Result<()> {
    let mut device = Device::connect("/dev/tty.Pixoo-SerialPort1")?;

    let image = [[Color {
        r: 0xff,
        g: 44,
        b: 51,
    }; 16]; 16];
    device.set_image(image)?;
    Ok(())
}
