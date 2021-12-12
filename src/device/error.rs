use thiserror::Error;

#[derive(Error, Debug)]
pub enum DeviceError {
    #[error("Could not connect to device")]
    ConnectionError(#[from] serialport::Error),

    #[error("Connection interrupted")]
    InterruptedError(#[from] std::io::Error),
}
