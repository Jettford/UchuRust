use std::fs::File;
use std::io::Write;

pub fn log(from: &str, message: &str) {
    println!("[{}] {}", from, message);
}

pub fn write_packet(name: String, data: &[u8]) {
    let mut file = File::create(name).unwrap();
    file.write_all(data);
}