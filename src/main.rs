extern crate r6502;
extern crate sdl2;
extern crate sdl2_image;
#[macro_use]
extern crate log;
extern crate env_logger;

mod sdl_appleii;
mod appleii;
mod mapper;
mod peripheral_card;

use std::env;

fn main()
{
    env_logger::init().unwrap();
    let filename = env::args().nth(1).expect("No file specified.");
    let mut sdl_apple = sdl_appleii::SDLAppleII::new(filename);

    sdl_apple.run();
}
