use peripheral_card::PeripheralCard;

const WRITE_SWITCH: u16 = 0x0001;
const READ_SWITCH: u16 = 0x0002;
const _UNUSED_SWITCH: u16 = 0x0004;
const BANK_SWITCH: u16 = 0x0008;

const HIGH_BANK_SIZE: usize = 0x2000;
const LOW_BANK_SIZE: usize = 0x1000;
const ROM_SIZE: usize = 0x3000;

pub struct LanguageCard
{
    rom: [u8; ROM_SIZE],
    high_bank: [u8; HIGH_BANK_SIZE],
    low_bank: [[u8; LOW_BANK_SIZE]; 2],
    last_write: bool,
    write: bool,
    read: bool,
    bank: usize,
}

impl LanguageCard
{
    pub fn new(rom: [u8; ROM_SIZE]) -> LanguageCard
    {
        LanguageCard
        {
            rom: rom,
            high_bank: [0; HIGH_BANK_SIZE],
            low_bank: [[0; LOW_BANK_SIZE]; 2],
            last_write: false,
            write: false,
            read: false,
            bank: 0,
        }
    }
}

impl PeripheralCard for LanguageCard
{
    fn read_switch(&mut self, switch: u16) -> u8
    {
        self.write = switch & WRITE_SWITCH != 0;

        /* READ == WRITE ? RAM : ROM */
        if ((switch & READ_SWITCH) != 0) == self.write
        {
            if self.last_write
            {
                self.read = true;
            }
            self.last_write = true;
        }
        else
        {
            self.read = false;
            self.last_write = false;
        }

        if switch & BANK_SWITCH != 0
        {
            self.bank = 0;
        }
        else
        {
            self.bank = 1;
        }

        0
    }

    fn read_switch_without_mm(&mut self, _switch: u16) -> u8
    {
        0
    }

    fn read_rom(&mut self, _addr: u16) -> u8
    {
        unreachable!()
    }

    fn read_expansion_rom(&mut self, _addr: u16) -> u8
    {
        0
    }

    fn read_language_rom(&mut self, addr: u16) -> u8
    {
        if self.read
        {
            if addr >= 0xE000
            {
                self.high_bank[(addr - 0xE000) as usize]
            }
            else
            {
                self.low_bank[(self.bank) as usize][(addr - 0xD000) as usize]
            }
        }
        else
        {
            self.rom[(addr - 0xD000) as usize]
        }
    }

    fn write_language_rom(&mut self, addr: u16, val: u8)
    {
        if self.write
        {
            if addr >= 0xE000
            {
                self.high_bank[(addr - 0xE000) as usize] = val;
            }
            else
            {
                self.low_bank[(self.bank) as usize][(addr - 0xD000) as usize] = val;
            }
        }
    }

    fn is_language_card(&self) -> bool
    {
        true
    }
}
