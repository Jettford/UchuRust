use std::fs::File;
use std::io::{Read, Write};
use std::io::Result as Res;

use endio::{Deserialize, LERead, LEWrite, Serialize, EWrite, BigEndian,};

use endio_bit::{BEBitWriter, BitWriter};

use byteorder::{WriteBytesExt, LittleEndian};

pub fn log(from: &str, message: &str) {
    println!("[{}] {}", from, message);
}

pub fn write_packet(name: String, data: &[u8]) {
    let mut file = File::create(name).unwrap();
    file.write_all(data);
}

trait StringExtensions {
    fn write_as_wstring(&self);
    fn write_as_string(&self, writer: &BEBitWriter<Vec<u8>>);
}

impl StringExtensions for String {
    fn write_as_wstring(&self) {

    }

    fn write_as_string(&self, writer: &BEBitWriter<Vec<u8>>) {
        
    }
}