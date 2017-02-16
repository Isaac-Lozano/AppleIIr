pub mod language_card;
pub mod disk;

pub use self::language_card::LanguageCard;
pub use self::disk::DiskII;

/* TODO: with and without mm */
pub trait PeripheralCard {
    fn read_switch(&mut self, switch: u16) -> u8 {
        self.read_switch_without_mm(switch)
    }

    fn write_switch(&mut self, switch: u16, val: u8) {
        self.write_switch_without_mm(switch, val);
    }

    fn read_switch_without_mm(&mut self, switch: u16) -> u8;
    fn write_switch_without_mm(&mut self, switch: u16, _val: u8) {
        self.read_switch_without_mm(switch);
    }

    fn read_rom(&mut self, addr: u16) -> u8;

    fn read_expansion_rom(&mut self, addr: u16) -> u8;

    fn read_language_rom(&mut self, _addr: u16) -> u8 {
        unreachable!()
    }

    fn write_language_rom(&mut self, _addr: u16, _val: u8) {
        unreachable!()
    }

    fn is_language_card(&self) -> bool {
        false
    }
}
