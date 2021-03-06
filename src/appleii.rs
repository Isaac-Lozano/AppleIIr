use mapper::{Mapper, ROM_SIZE};
use monitor::Monitor;
use input::{Input, KeyboardInput};
use peripheral_card::{LanguageCard, DiskII};

use r6502::cpu6502::Cpu6502;

use std::fs;
use std::thread;
use std::time::{Instant, Duration};

use sdl2;

pub struct AppleII<'a> {
    cpu: Cpu6502<Mapper<'a>>,
    monitor: Monitor<'a>,
    input: Input,
    paused: bool,
}

impl<'a> AppleII<'a> {
    pub fn new(rom: [u8; ROM_SIZE]) -> AppleII<'a> {
        let disk_file = fs::File::open("diskii.img").expect("Disk file not found.");

        let mut dc = DiskII::new();
        dc.set_first_disk(disk_file);

        let lc = LanguageCard::new(rom);

        let mut map = Mapper::new(rom);
        info!("Adding card lang");
        map.add_card(lc, 0);
        info!("Adding card disk");
        map.add_card(dc, 6);

        let sdl_context = sdl2::init().expect("Could not init SDL2.");
        let sdl_video = sdl_context.video()
            .expect("Could not init SDL2 video.");
        let sdl_events = sdl_context.event_pump()
            .expect("Could not event pump.");
        let sdl_keyboard = sdl_context.keyboard();

        AppleII {
            cpu: Cpu6502::new(map),
            monitor: Monitor::new(sdl_video),
            input: Input::new(sdl_events, sdl_keyboard),
            paused: false,
        }
    }

    pub fn run(&mut self) {
        'runloop: loop {
            let begin = Instant::now();

            for input in self.input.keyboard_inputs()
            {
                match input {
                    KeyboardInput::Quit => break 'runloop,
                    KeyboardInput::Reset => if !self.paused { self.cpu.reset() },
                    KeyboardInput::Key(val) => if !self.paused { self.cpu.memory.set_key(val) },
                    KeyboardInput::Pause => self.paused = !self.paused,
                }
            }

            self.monitor.update_window(&self.cpu.memory, self.cpu.cycles);

            if !self.paused
            {
                /* 16666 clocks per 1/60 seconds */
                self.cpu.run(16666).expect("AAAAA CPU DIED");
            }

            let elapsed = begin.elapsed();
            let fps60 = Duration::new(0, 16666666);
            if elapsed > fps60 {
                warn!("Cpu overrun by {}.{:010}s",
                      (elapsed - fps60).as_secs(),
                      (elapsed - fps60).subsec_nanos());
            } else {
                thread::sleep(fps60 - elapsed);
            }
        }
    }
}
