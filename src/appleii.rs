use mapper::{Mapper, ROM_SIZE};
use peripheral_card::{LanguageCard, DiskII};

use r6502::cpu6502::{CPU6502, CPU6502Result};

use std::fs;
use std::io::Read;

pub struct AppleII<'a>
{
    pub cpu: CPU6502<Mapper<'a>>,
}

impl<'a> AppleII<'a>
{
    pub fn new(filename: String) -> AppleII<'a>
    {
        let mut file = fs::File::open(filename).expect("File not found.");;
        let file_size = file.metadata().expect("Could not get metadata").len();

        if file_size != ROM_SIZE as u64
        {
            panic!("File not the proper size. Found {} bytes, should be {} bytes.",
                   file_size, ROM_SIZE);
        }

        let mut buf = [0x00; ROM_SIZE];
        file.read(&mut buf).expect("Could not read from file.");

        let mut disk_file = fs::File::open("diskii.img").expect("File not found.");;

        let dc = DiskII::new(Some(&mut disk_file), None);
        let lc = LanguageCard::new(buf);

        let mut map = Mapper::new(buf);
        info!("Adding card lang");
        map.add_card(lc, 0);
        info!("Adding card disk");
        map.add_card(dc, 6);
        
        AppleII{ cpu: CPU6502::new(map), }
    }

    pub fn run(&mut self, cycles: u32) -> CPU6502Result<u32>
    {
        self.cpu.run(cycles)
    }
}
