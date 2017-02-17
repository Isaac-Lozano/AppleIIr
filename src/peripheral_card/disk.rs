use peripheral_card::PeripheralCard;

use std::io::Read;

/* Disk has 35 concentric tracks.
 * Outer = $00, inner = $22
 *
 * 16 sectors per track.
 * $0 to $F
 *
 * 256 bytes can be stored in each sector.
 * $00 to $100
 */

/* The rom for the Disk2.
 * It will be "copied" into
 * the Apple II's memory.
 * Taken from Apple Win.
 */

static DISK2_ROM: [u8; 0x100] =
    [0xA2, 0x20, 0xA0, 0x00, 0xA2, 0x03, 0x86, 0x3C, 0x8A, 0x0A, 0x24, 0x3C, 0xF0, 0x10, 0x05,
     0x3C, 0x49, 0xFF, 0x29, 0x7E, 0xB0, 0x08, 0x4A, 0xD0, 0xFB, 0x98, 0x9D, 0x56, 0x03, 0xC8,
     0xE8, 0x10, 0xE5, 0x20, 0x58, 0xFF, 0xBA, 0xBD, 0x00, 0x01, 0x0A, 0x0A, 0x0A, 0x0A, 0x85,
     0x2B, 0xAA, 0xBD, 0x8E, 0xC0, 0xBD, 0x8C, 0xC0, 0xBD, 0x8A, 0xC0, 0xBD, 0x89, 0xC0, 0xA0,
     0x50, 0xBD, 0x80, 0xC0, 0x98, 0x29, 0x03, 0x0A, 0x05, 0x2B, 0xAA, 0xBD, 0x81, 0xC0, 0xA9,
     0x56, 0x20, 0xA8, 0xFC, 0x88, 0x10, 0xEB, 0x85, 0x26, 0x85, 0x3D, 0x85, 0x41, 0xA9, 0x08,
     0x85, 0x27, 0x18, 0x08, 0xBD, 0x8C, 0xC0, 0x10, 0xFB, 0x49, 0xD5, 0xD0, 0xF7, 0xBD, 0x8C,
     0xC0, 0x10, 0xFB, 0xC9, 0xAA, 0xD0, 0xF3, 0xEA, 0xBD, 0x8C, 0xC0, 0x10, 0xFB, 0xC9, 0x96,
     0xF0, 0x09, 0x28, 0x90, 0xDF, 0x49, 0xAD, 0xF0, 0x25, 0xD0, 0xD9, 0xA0, 0x03, 0x85, 0x40,
     0xBD, 0x8C, 0xC0, 0x10, 0xFB, 0x2A, 0x85, 0x3C, 0xBD, 0x8C, 0xC0, 0x10, 0xFB, 0x25, 0x3C,
     0x88, 0xD0, 0xEC, 0x28, 0xC5, 0x3D, 0xD0, 0xBE, 0xA5, 0x40, 0xC5, 0x41, 0xD0, 0xB8, 0xB0,
     0xB7, 0xA0, 0x56, 0x84, 0x3C, 0xBC, 0x8C, 0xC0, 0x10, 0xFB, 0x59, 0xD6, 0x02, 0xA4, 0x3C,
     0x88, 0x99, 0x00, 0x03, 0xD0, 0xEE, 0x84, 0x3C, 0xBC, 0x8C, 0xC0, 0x10, 0xFB, 0x59, 0xD6,
     0x02, 0xA4, 0x3C, 0x91, 0x26, 0xC8, 0xD0, 0xEF, 0xBC, 0x8C, 0xC0, 0x10, 0xFB, 0x59, 0xD6,
     0x02, 0xD0, 0x87, 0xA0, 0x00, 0xA2, 0x56, 0xCA, 0x30, 0xFB, 0xB1, 0x26, 0x5E, 0x00, 0x03,
     0x2A, 0x5E, 0x00, 0x03, 0x2A, 0x91, 0x26, 0xC8, 0xD0, 0xEE, 0xE6, 0x27, 0xE6, 0x3D, 0xA5,
     0x3D, 0xCD, 0x00, 0x08, 0xA6, 0x2B, 0x90, 0xDB, 0x4C, 0x01, 0x08, 0x00, 0x00, 0x00, 0x00,
     0x00];

/*  Helps with the bit fiddling necessary to extract the bottom
 *  two bits during the 256 - 342 byte nibblize.
 */

static TAB1: [u8; 64] = [0x00, 0x08, 0x04, 0x0C, 0x20, 0x28, 0x24, 0x2C, 0x10, 0x18, 0x14, 0x1C,
                         0x30, 0x38, 0x34, 0x3C, 0x80, 0x88, 0x84, 0x8C, 0xA0, 0xA8, 0xA4, 0xAC,
                         0x90, 0x98, 0x94, 0x9C, 0xB0, 0xB8, 0xB4, 0xBC, 0x40, 0x48, 0x44, 0x4C,
                         0x60, 0x68, 0x64, 0x6C, 0x50, 0x58, 0x54, 0x5C, 0x70, 0x78, 0x74, 0x7C,
                         0xC0, 0xC8, 0xC4, 0xCC, 0xE0, 0xE8, 0xE4, 0xEC, 0xD0, 0xD8, 0xD4, 0xDC,
                         0xF0, 0xF8, 0xF4, 0xFC];


/*  Translates to "disk bytes"
 */

static TAB2: [u8; 64] = [0x96, 0x97, 0x9A, 0x9B, 0x9D, 0x9E, 0x9F, 0xA6, 0xA7, 0xAB, 0xAC, 0xAD,
                         0xAE, 0xAF, 0xB2, 0xB3, 0xB4, 0xB5, 0xB6, 0xB7, 0xB9, 0xBA, 0xBB, 0xBC,
                         0xBD, 0xBE, 0xBF, 0xCB, 0xCD, 0xCE, 0xCF, 0xD3, 0xD6, 0xD7, 0xD9, 0xDA,
                         0xDB, 0xDC, 0xDD, 0xDE, 0xDF, 0xE5, 0xE6, 0xE7, 0xE9, 0xEA, 0xEB, 0xEC,
                         0xED, 0xEE, 0xEF, 0xF2, 0xF3, 0xF4, 0xF5, 0xF6, 0xF7, 0xF9, 0xFA, 0xFB,
                         0xFC, 0xFD, 0xFE, 0xFF];


/*  Dos 3.3 to physical sector conversion
 */

static PHYS: [u8; 16] = [0x00, 0x0D, 0x0B, 0x09, 0x07, 0x05, 0x03, 0x01, 0x0E, 0x0C, 0x0A, 0x08,
                         0x06, 0x04, 0x02, 0x0F];

pub struct Drive {
    sectors: Option<Box<[[[u8; 0x200]; 16]; 70]>>,
    track: usize,
    sector: usize,
    idx: usize,
    /* holds bitmap of magnets enabled */
    magnets: u32,
    /* holds current magnet phase */
    phase: u32,
}

impl Drive {
    pub fn new() -> Drive
    {
        Drive {
            sectors: None,
            track: 0,
            sector: 15,
            idx: 0,
            magnets: 0,
            phase: 0,
        }
    }

    pub fn add_disk<R>(&mut self, mut disk: R)
        where R: Read
    {
        let mut data = [[[0; 0x200]; 16]; 70];
        for (track_num, track) in data.iter_mut().enumerate() {
            for (sector_num, sector) in track.iter_mut().enumerate() {
                let mut idx = 0;
                let phys_sector = PHYS[sector_num];

                for _ in 0..16 {
                    sector[idx] = 0xFF;
                    idx += 1;
                }

                /* address header */
                sector[idx] = 0xD5;
                idx += 1;
                sector[idx] = 0xAA;
                idx += 1;
                sector[idx] = 0x96;
                idx += 1;

                /* disk volume = 254 */
                sector[idx] = 0xFF;
                idx += 1;
                sector[idx] = 0xFE;
                idx += 1;

                sector[idx] = Drive::nib_odd(track_num as u8);
                idx += 1;
                sector[idx] = Drive::nib_even(track_num as u8);
                idx += 1;

                sector[idx] = Drive::nib_odd(phys_sector);
                idx += 1;
                sector[idx] = Drive::nib_even(phys_sector);
                idx += 1;

                let checksum = 254 ^ track_num ^ phys_sector as usize;
                sector[idx] = Drive::nib_odd(checksum as u8);
                idx += 1;
                sector[idx] = Drive::nib_even(checksum as u8);
                idx += 1;

                /* address trailer */
                sector[idx] = 0xDE;
                idx += 1;
                sector[idx] = 0xAA;
                idx += 1;
                sector[idx] = 0xEB;
                idx += 1;

                for _ in 0..8 {
                    sector[idx] = 0xFF;
                    idx += 1;
                }

                /* data header */
                sector[idx] = 0xD5;
                idx += 1;
                sector[idx] = 0xAA;
                idx += 1;
                sector[idx] = 0xAD;
                idx += 1;

                /* encode data */
                let mut buf = [0u8; 344];

                /* ignore if it doesn't read the entire length */
                disk.read(&mut buf[0x56..0x56 + 0x100]).unwrap();

                for off in 0..0x56 {
                    let i = (buf[off + 0x56] & 3) | (buf[off + 0x56 + 0x56] & 3) << 2 |
                            (buf[off + 0x56 + 0x56 + 0x56] & 3) << 4;
                    buf[off] = TAB1[i as usize];
                }

                sector[idx] = buf[0];
                for off in 1..343 {
                    sector[idx + off] = buf[off - 1] ^ buf[off];
                }

                for off in 0..343 {
                    sector[idx + off] = TAB2[(sector[idx + off] >> 2) as usize];
                }

                idx += 343;

                /* data trailer */
                sector[idx] = 0xDE;
                idx += 1;
                sector[idx] = 0xAA;
                idx += 1;
                sector[idx] = 0xEB;
            }
        }
        self.sectors = Some(Box::new(data));
    }

    fn step_motor(&mut self, magnet: u16, enable: bool) {
        /* magnet is range 0-3 inclusive */
        if enable {
            self.magnets |= 1 << magnet as u32;
        } else {
            self.magnets &= !(1 << magnet as u32);
        }

        if self.magnets & (1 << ((self.phase + 1) % 4)) != 0 && self.phase < 140 {
            self.phase += 1;
        }
        if self.magnets & (1 << ((self.phase + 3) % 4)) != 0 && self.phase > 0 {
            self.phase -= 1;
        }

        if self.track != ((self.phase + 1) / 2) as usize {
            info!("track {}", (self.phase + 1) / 2);
        }

        self.track = ((self.phase + 1) / 2) as usize;
    }

    fn read(&mut self) -> u8 {
        match self.sectors {
            Some(ref data) => {
                let mut ret = data[self.track][self.sector][self.idx];
                if ret == 0 {
                    self.sector += 15;
                    self.sector %= 16;
                    info!("sector {}", self.sector);
                    self.idx = 0;
                    ret = data[self.track][self.sector][self.idx];
                }
                self.idx += 1;
                ret
            }
            None => 0xFF,
        }
    }

    fn read_without_mm(&mut self) -> u8 {
        match self.sectors {
            Some(ref data) => {
                let mut ret = data[self.track][self.sector][self.idx];
                if ret == 0 {
                    self.sector += 15;
                    self.sector %= 16;
                    info!("sector {}", self.sector);
                    self.idx = 0;
                    ret = data[self.track][self.sector][self.idx];
                }
                ret
            }
            None => 0xFF,
        }
    }

    fn nib_odd(byte: u8) -> u8 {
        (byte >> 1) | 0xAA
    }

    fn nib_even(byte: u8) -> u8 {
        byte | 0xAA
    }
}

enum Mode {
    Read,
    Write,
}

pub struct DiskII {
    drives: [Drive; 2],
    //    write_reg: u8,
    drive_num: usize,
    mode: Mode,
    write_protect: bool,
}

impl DiskII {
    pub fn new() -> DiskII
    {
        DiskII {
            drives: [Drive::new(), Drive::new()],
            //            write_reg: 0,
            drive_num: 0,
            mode: Mode::Read,
            write_protect: false,
        }
    }

    pub fn set_first_disk<R>(&mut self, disk: R)
        where R: Read
    {
        self.drives[0].add_disk(disk);
    }

    pub fn set_second_disk<R>(&mut self, disk: R)
        where R: Read
    {
        self.drives[1].add_disk(disk);
    }

    fn current_drive(&mut self) -> &mut Drive {
        &mut self.drives[self.drive_num]
    }
}

impl PeripheralCard for DiskII {
    fn read_switch(&mut self, switch: u16) -> u8 {
        match switch {
            /* phase switches */
            0x00...0x07 => {
                info!("Phase switch {}, enable {}", switch >> 1, (switch & 1) != 0);
                self.current_drive()
                    .step_motor(switch >> 1, (switch & 1) != 0);
                0
            }
            /* ignore motor stuff */
            0x08...0x09 => {
                info!("Motor {}", switch & 1 != 0);
                0
            }
            0x0A => {
                info!("drive 0");
                self.drive_num = 0;
                0
            }
            0x0B => {
                info!("drive 1");
                self.drive_num = 1;
                0
            }
            0x0C => {
                match self.mode {
                    Mode::Read => self.current_drive().read(),
                    Mode::Write => 0,
                }
            }
            0x0D => {
                info!("Writing to write reg");
                0x00
            }
            0x0E => {
                info!("Setting read mode");
                self.mode = Mode::Read;
                if self.write_protect { 0xFF } else { 0x00 }
            }
            0x0F => {
                info!("Setting write mode");
                self.mode = Mode::Write;
                0x00
            }
            _ => 0,
        }
    }

    fn read_switch_without_mm(&mut self, switch: u16) -> u8 {
        match switch {
            /* phase switches */
            0x00...0x07 => 0,
            /* ignore motor stuff */
            0x08...0x09 => 0,
            0x0A => 0,
            0x0B => 0,
            0x0C => {
                match self.mode {
                    Mode::Read => self.current_drive().read_without_mm(),
                    Mode::Write => 0,
                }
            }
            0x0D => 0x00,
            0x0E => if self.write_protect { 0xFF } else { 0x00 },
            0x0F => 0x00,
            _ => 0,
        }
    }

    fn read_rom(&mut self, addr: u16) -> u8 {
        let rom_addr = (addr & 0xFF) as usize;
        match rom_addr {
            0x4C => 0xA9,
            0x4D => 0x00,
            0x4E => 0xEA,
            _ => DISK2_ROM[rom_addr],
        }
    }

    fn read_expansion_rom(&mut self, _addr: u16) -> u8 {
        0
    }
}
