use lib_rust_boi::cpu::CPU;

use clap::{App, Arg};
use std::io::Read;

pub fn main() {
    let boot_buffer = Some(buffer_from_file("./test_roms/dmg_boot.bin"));
    let game_buffer = buffer_from_file("./test_roms/tetris.gb");
    
    let mut cpu = CPU::new(boot_buffer, game_buffer);
    loop {
        cpu.step();
    }
}



fn buffer_from_file(path: &str) -> Vec<u8> {
    let mut file = std::fs::File::open(path).expect("File not there");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Could not read file");
    buffer
}