use mapper::Mapper;

use r6502::cpu6502::CPU6502;

use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::KeyboardUtil;
use sdl2::keyboard::{self, Keycode};

pub struct Input
{
    events: EventPump,
    keyboard: KeyboardUtil,
}

impl Input
{
    pub fn new(events: EventPump, keyboard: KeyboardUtil) -> Input
    {
        Input
        {
            events: events,
            keyboard: keyboard,
        }
    }

    pub fn process_input(&mut self, cpu: &mut CPU6502<Mapper>) -> bool
    {
        for event in self.events.poll_iter()
        {
            match event
            {
                Event::Quit{..} =>
                    return true,
                Event::KeyDown{ keycode, ..} =>
                {
                    if keycode == Some(Keycode::F2)
                    {
                        cpu.reset();
                        return false;
                    }
                    if let Some(val) = Input::map_keycode(keycode, &self.keyboard)
                    {
                        cpu.memory.set_key(val);
                    }

                },
                _ => {},
            }
        }

        false
    }

    /* TODO: keep track of mod keys ourselves */
    fn map_keycode(opt_code: Option<Keycode>, keyboard: &KeyboardUtil) -> Option<u8>
    {
        let code = match opt_code
        {
            Some(code) => code,
            None => return None,
        };
        let keystate = keyboard.mod_state();
        let mut ch = match code
        {
            Keycode::A =>
                'A' as u8,
            Keycode::B =>
                'B' as u8,
            Keycode::C =>
                'C' as u8,
            Keycode::D =>
                'D' as u8,
            Keycode::E =>
                'E' as u8,
            Keycode::F =>
                'F' as u8,
            Keycode::G =>
                'G' as u8,
            Keycode::H =>
                'H' as u8,
            Keycode::I =>
                'I' as u8,
            Keycode::J =>
                'J' as u8,
            Keycode::K =>
                'K' as u8,
            Keycode::L =>
                'L' as u8,
            Keycode::M =>
                'M' as u8,
            Keycode::N =>
                'N' as u8,
            Keycode::O =>
                'O' as u8,
            Keycode::P =>
                'P' as u8,
            Keycode::Q =>
                'Q' as u8,
            Keycode::R =>
                'R' as u8,
            Keycode::S =>
                'S' as u8,
            Keycode::T =>
                'T' as u8,
            Keycode::U =>
                'U' as u8,
            Keycode::V =>
                'V' as u8,
            Keycode::W =>
                'W' as u8,
            Keycode::X =>
                'X' as u8,
            Keycode::Y =>
                'Y' as u8,
            Keycode::Z =>
                'Z' as u8,
            Keycode::RightBracket =>
                ']' as u8,
            Keycode::Space =>
                ' ' as u8,
            Keycode::Quote =>
                '\'' as u8,
            Keycode::Comma =>
                ',' as u8,
            Keycode::Minus =>
                '-' as u8,
            Keycode::Period =>
                '.' as u8,
            Keycode::Slash =>
                '/' as u8,
            Keycode::Num0 =>
                '0' as u8,
            Keycode::Num1 =>
                '1' as u8,
            Keycode::Num2 =>
                '2' as u8,
            Keycode::Num3 =>
                '3' as u8,
            Keycode::Num4 =>
                '4' as u8,
            Keycode::Num5 =>
                '5' as u8,
            Keycode::Num6 =>
                '6' as u8,
            Keycode::Num7 =>
                '7' as u8,
            Keycode::Num8 =>
                '8' as u8,
            Keycode::Num9 =>
                '9' as u8,
            Keycode::Semicolon =>
                ';' as u8,
            Keycode::Equals =>
                '=' as u8,
            Keycode::Return =>
                0x0D,
            Keycode::Left | Keycode::Backspace =>
                0x08,
            Keycode::Right =>
                0x15,
            Keycode::Escape =>
                0x1B,
            _ => return None,
        };
        if keystate.intersects(keyboard::LSHIFTMOD | keyboard::RSHIFTMOD)
        {
            ch = match code
            {
                Keycode::Num1 =>
                    '!' as u8,
                Keycode::Num2 =>
                    '@' as u8,
                Keycode::Num3 =>
                    '#' as u8,
                Keycode::Num4 =>
                    '$' as u8,
                Keycode::Num5 =>
                    '%' as u8,
                Keycode::Num6 =>
                    '^' as u8,
                Keycode::Num7 =>
                    '&' as u8,
                Keycode::Num8 =>
                    '*' as u8,
                Keycode::Num9 =>
                    '(' as u8,
                Keycode::Num0 =>
                    ')' as u8,
                Keycode::Equals =>
                    '+' as u8,
                Keycode::Semicolon =>
                    ':' as u8,
                Keycode::Quote =>
                    '"' as u8,
                Keycode::Comma =>
                    '<' as u8,
                Keycode::Period =>
                    '>' as u8,
                Keycode::Slash =>
                    '?' as u8,
                _ => ch,
            };
        }
        if keystate.intersects(keyboard::LCTRLMOD | keyboard::RCTRLMOD)
        {
            ch = match code
            {
                Keycode::A =>
                    0x81,
                Keycode::B =>
                    0x82,
                Keycode::C =>
                    0x83,
                Keycode::D =>
                    0x84,
                Keycode::E =>
                    0x85,
                Keycode::F =>
                    0x86,
                Keycode::G =>
                    0x87,
                Keycode::H =>
                    0x88,
                Keycode::I =>
                    0x89,
                Keycode::J =>
                    0x8A,
                Keycode::K =>
                    0x8B,
                Keycode::L =>
                    0x8C,
                Keycode::M =>
                    0x8D,
                Keycode::N =>
                    0x8E,
                Keycode::O =>
                    0x8F,
                Keycode::P =>
                    0x90,
                Keycode::Q =>
                    0x91,
                Keycode::R =>
                    0x92,
                Keycode::S =>
                    0x93,
                Keycode::T =>
                    0x94,
                Keycode::U =>
                    0x95,
                Keycode::V =>
                    0x96,
                Keycode::W =>
                    0x97,
                Keycode::X =>
                    0x98,
                Keycode::Y =>
                    0x99,
                Keycode::Z =>
                    0x9A,
                _ =>
                    ch,
            };
            if keystate.intersects(keyboard::LSHIFTMOD | keyboard::RSHIFTMOD)
            {
                ch = match code
                {
                    Keycode::M =>
                        0x9D,
                    Keycode::N =>
                        0x9E,
                    Keycode::P =>
                        0x80,
                    _ =>
                        ch,
                }
            }
        }
        Some(ch | 0x80)
    }
}
