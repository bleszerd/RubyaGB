#![allow(warnings)]

mod cpu;

use std::{
    fs::{self, File},
    io::Read,
};

use cpu::{FlagsRegister, MemoryBus, Registers, CPU};

use crate::cpu::CartridgeHeader;

fn main() {
    let filename: String = String::from("/home/bleszerd/repositories/rubyagb/roms/blue.gb");

    let mut file = File::open(&filename).expect("No file found");
    let metadata: fs::Metadata = fs::metadata(&filename).expect("Unable to read metada");
    let mut buffer: Vec<u8> = vec![0; metadata.len() as usize];
    file.read(&mut buffer).expect("Buffer overflow");

    let cartridge_header: CartridgeHeader = CartridgeHeader::init(buffer);

    println!("{:?}", cartridge_header);
}
