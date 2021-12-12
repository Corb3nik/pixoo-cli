use byteorder::{LittleEndian, WriteBytesExt};
use std::fs::File;
use std::io::{Read, Write};
use std::thread::sleep_ms;

const HEADER: u8 = 0x01;
const FOOTER: u8 = 0x02;

fn get_crc(data: &[u8]) -> Vec<u8> {
    let mut sum: u16 = 0;
    for byte in data {
        sum += *byte as u16;
    }

    let mut res = vec![];
    res.write_u16::<LittleEndian>(sum);
    res
}

fn create_frame(payload: &[u8]) -> Vec<u8> {
    let crc_length = 2;
    let length: u16 = payload.len() as u16 + crc_length;
    let mut length_arr = vec![];
    length_arr.write_u16::<LittleEndian>(length);

    let mut crc_data = vec![length as u8];
    crc_data.extend_from_slice(payload);
    let crc = get_crc(&crc_data);

    let mut frame = vec![HEADER];
    frame.extend(&length_arr);
    frame.extend(payload);
    frame.extend(&crc);
    frame.push(FOOTER);

    frame
}

fn main() {
    println!("Hello, world!");
    let mut sock = serialport::new("/dev/tty.Pixoo-SerialPort1", 115_200)
        .open()
        .expect("Hello");
    //let mut sock = File::open("/dev/tty.Pixoo-SerialPort1").expect("Failed");

    sleep_ms(2000);

    let data = b"\x74\x40";
    let frame = create_frame(data);

    println!("{:?}", frame);
    //sock.write(&frame);
    sock.write(&frame);

    while true {
        let mut out = [0u8; 1];
        let data = sock.read(&mut out[..]);
        println!("{:?}", out);
    }

    //sock.write(b"\x01\x05\x00\x45\x00\x03\x04\x4B\x00\x02");
}
