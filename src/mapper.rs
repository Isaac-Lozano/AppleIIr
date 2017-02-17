use r6502::memory::Memory;
use peripheral_card::PeripheralCard;

pub const RAM_SIZE: usize = 0xC000;
pub const ROM_SIZE: usize = 0x3000;

pub const APPLE_II_TEXT_WIDTH: usize = 40;
pub const APPLE_II_TEXT_HEIGHT: usize = 24;

pub struct ScreenState {
    pub graphics: bool,
    pub all: bool,
    pub primary: bool,
    pub low_res: bool,
}

pub struct Mapper<'a> {
    pub ram: [u8; RAM_SIZE],
    pub rom: [u8; ROM_SIZE],
    pub key: u8,
    pub screen: ScreenState,
    pub cards: [Option<Box<PeripheralCard + 'a>>; 8],
    pub has_lang_card: bool,
}

impl<'a> Mapper<'a> {
    pub fn new(rom: [u8; ROM_SIZE]) -> Mapper<'a> {
        Mapper {
            ram: [0x00; RAM_SIZE],
            rom: rom,
            key: 0,
            screen: ScreenState {
                graphics: false,
                all: true,
                primary: true,
                low_res: true,
            },
            cards: [None, None, None, None, None, None, None, None],
            has_lang_card: false,
        }
    }

    pub fn set_key(&mut self, key: u8) {
        self.key = key;
    }

    pub fn add_card<T: PeripheralCard + 'a>(&mut self, card: T, slot: usize) {
        if slot == 0 {
            self.has_lang_card = card.is_language_card();
        }

        self.cards[slot] = Some(Box::new(card));
    }

    pub fn remove_card(&mut self, slot: usize) {
        if slot == 0 {
            self.has_lang_card = false;
        }

        self.cards[slot] = None;
    }
}

impl<'a> Memory<u8> for Mapper<'a> {
    fn read_without_mm(&mut self, addr: u16) -> u8 {
        match addr {
            0x0000...0xBFFF => self.ram[addr as usize],
            0xC000 => self.key,
            0xC010 => {
                self.key &= 0x7F;
                0x00
            }
            0xC050 => {
                self.screen.graphics = true;
                0x00
            }
            0xC051 => {
                self.screen.graphics = false;
                0x00
            }
            0xC052 => {
                self.screen.all = true;
                0x00
            }
            0xC053 => {
                self.screen.all = false;
                0x00
            }
            0xC054 => {
                self.screen.primary = true;
                0x00
            }
            0xC055 => {
                self.screen.primary = false;
                0x00
            }
            0xC056 => {
                self.screen.low_res = true;
                0x00
            }
            0xC057 => {
                self.screen.low_res = false;
                0x00
            }
            0xC080...0xC0FF => {
                let slot = (((addr - 0xC000) >> 4) - 8) as usize;
                match self.cards[slot] {
                    Some(ref mut card) => card.read_switch(addr & 0xF),
                    None => 0xFF,
                }
            }
            0xC100...0xC7FF => {
                let slot = ((addr - 0xC000) >> 8) as usize;
                match self.cards[slot] {
                    Some(ref mut card) => card.read_rom(addr),
                    None => 0xFF,
                }
            }
            0xD000...0xFFFF => {
                if self.has_lang_card {
                    self.cards
                        .get_mut(0)
                        .unwrap()
                        .as_mut()
                        .unwrap()
                        .read_language_rom(addr)
                } else {
                    self.rom[(addr - 0xD000) as usize]
                }
            }
            _ => 0x00,
        }
    }

    fn write_without_mm(&mut self, addr: u16, val: u8) {
        match addr {
            0x0000...0xBFFF => self.ram[addr as usize] = val,
            0xC010 => self.key &= 0x7F,
            0xC050 => self.screen.graphics = true,
            0xC051 => self.screen.graphics = false,
            0xC052 => self.screen.all = true,
            0xC053 => self.screen.all = false,
            0xC054 => self.screen.primary = true,
            0xC055 => self.screen.primary = false,
            0xC056 => self.screen.low_res = true,
            0xC057 => self.screen.low_res = false,
            0xC080...0xC0FF => {
                let slot = (((addr - 0xC000) >> 4) - 8) as usize;
                if let Some(ref mut card) = self.cards[slot] {
                    card.write_switch(addr & 0xF, val);
                }
            }
            0xD000...0xFFFF => {
                if self.has_lang_card {
                    self.cards
                        .get_mut(0)
                        .unwrap()
                        .as_mut()
                        .unwrap()
                        .write_language_rom(addr, val);
                }
            }
            _ => {}
        }
    }
}
