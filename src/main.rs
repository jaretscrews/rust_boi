use lib_rust_boi::cpu::CPU;

use clap::{App, Arg};
use std::io::Read;

pub fn main() {
    let matches = App::new("DMG-01")
        .author("Ryan Levick <ryan.levick@gmail.com>")
        .arg(Arg::with_name("boot rom").short("b").value_name("FILE"))
        .arg(
            Arg::with_name("rom")
                .short("r")
                .required(true)
                .value_name("FILE"),
        )
        .get_matches();

    let boot_buffer = matches
        .value_of("boot rom")
        .map(|path| buffer_from_file(path));
    let game_buffer = matches
        .value_of("rom")
        .map(|path| buffer_from_file(path))
        .unwrap();
    
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