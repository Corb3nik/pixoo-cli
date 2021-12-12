use anyhow::Result;
use pixoo_cli::device::Device;

fn main() -> Result<()> {
    let mut device = Device::connect("/dev/tty.Pixoo-SerialPort1")?;
    device.set_brightness(100)?;
    device.show_time()?;
    Ok(())
}
