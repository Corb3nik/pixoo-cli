use anyhow::Result;
use pixoo_cli::device::Device;
use pixoo_cli::utils::Color;

fn main() -> Result<()> {
    let mut device = Device::connect("/dev/tty.Pixoo-SerialPort1")?;

    let image = [[Color {
        r: 0xff,
        g: 0xff,
        b: 51,
    }; 16]; 16];
    device.set_image(image)?;
    Ok(())
}
