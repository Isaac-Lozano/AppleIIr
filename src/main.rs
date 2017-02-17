extern crate r6502;
extern crate sdl2;
#[macro_use]
extern crate log;
extern crate env_logger;

mod appleii;
mod monitor;
mod input;
mod mapper;
mod peripheral_card;

use mapper::ROM_SIZE;

use std::env;
use std::fs;
use std::io::Read;

fn main() {
    env_logger::init().unwrap();
    let filename = env::args().nth(1).expect("No file specified.");

    let mut file = fs::File::open(filename).expect("File not found.");
    let file_size = file.metadata().expect("Could not get metadata").len();

    if file_size != ROM_SIZE as u64 {
        panic!("File not the proper size. Found {} bytes, should be {} bytes.",
               file_size,
               ROM_SIZE);
    }

    let mut buf = [0x00; ROM_SIZE];
    file.read_exact(&mut buf).expect("Could not read from file.");

    let mut sdl_apple = appleii::AppleII::new(buf);

    sdl_apple.run();
}
