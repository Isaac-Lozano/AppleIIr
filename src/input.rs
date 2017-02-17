use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::KeyboardUtil;
use sdl2::keyboard::{self, Keycode};

pub struct Input {
    events: EventPump,
    keyboard: KeyboardUtil,
}

impl Input {
    pub fn new(events: EventPump, keyboard: KeyboardUtil) -> Input {
        Input {
            events: events,
            keyboard: keyboard,
        }
    }

    pub fn keyboard_inputs(&mut self) -> KeyboardInputs {
        KeyboardInputs {
            input: self,
        }
    }

    /* TODO: keep track of mod keys ourselves */
    fn map_keycode(opt_code: Option<Keycode>, keyboard: &KeyboardUtil) -> Option<u8> {
        let code = match opt_code {
            Some(code) => code,
            None => return None,
        };
        let keystate = keyboard.mod_state();
        let mut ch = match code {
            Keycode::A => b'A' as u8,
            Keycode::B => b'B' as u8,
            Keycode::C => b'C' as u8,
            Keycode::D => b'D' as u8,
            Keycode::E => b'E' as u8,
            Keycode::F => b'F' as u8,
            Keycode::G => b'G' as u8,
            Keycode::H => b'H' as u8,
            Keycode::I => b'I' as u8,
            Keycode::J => b'J' as u8,
            Keycode::K => b'K' as u8,
            Keycode::L => b'L' as u8,
            Keycode::M => b'M' as u8,
            Keycode::N => b'N' as u8,
            Keycode::O => b'O' as u8,
            Keycode::P => b'P' as u8,
            Keycode::Q => b'Q' as u8,
            Keycode::R => b'R' as u8,
            Keycode::S => b'S' as u8,
            Keycode::T => b'T' as u8,
            Keycode::U => b'U' as u8,
            Keycode::V => b'V' as u8,
            Keycode::W => b'W' as u8,
            Keycode::X => b'X' as u8,
            Keycode::Y => b'Y' as u8,
            Keycode::Z => b'Z' as u8,
            Keycode::RightBracket => b']' as u8,
            Keycode::Space => b' ' as u8,
            Keycode::Quote => b'\'' as u8,
            Keycode::Comma => b',' as u8,
            Keycode::Minus => b'-' as u8,
            Keycode::Period => b'.' as u8,
            Keycode::Slash => b'/' as u8,
            Keycode::Num0 => b'0' as u8,
            Keycode::Num1 => b'1' as u8,
            Keycode::Num2 => b'2' as u8,
            Keycode::Num3 => b'3' as u8,
            Keycode::Num4 => b'4' as u8,
            Keycode::Num5 => b'5' as u8,
            Keycode::Num6 => b'6' as u8,
            Keycode::Num7 => b'7' as u8,
            Keycode::Num8 => b'8' as u8,
            Keycode::Num9 => b'9' as u8,
            Keycode::Semicolon => b';' as u8,
            Keycode::Equals => b'=' as u8,
            Keycode::Return => 0x0D,
            Keycode::Left | Keycode::Backspace => 0x08,
            Keycode::Right => 0x15,
            Keycode::Escape => 0x1B,
            _ => return None,
        };
        if keystate.intersects(keyboard::LSHIFTMOD | keyboard::RSHIFTMOD) {
            ch = match code {
                Keycode::Num1 => b'!' as u8,
                Keycode::Num2 => b'@' as u8,
                Keycode::Num3 => b'#' as u8,
                Keycode::Num4 => b'$' as u8,
                Keycode::Num5 => b'%' as u8,
                Keycode::Num6 => b'^' as u8,
                Keycode::Num7 => b'&' as u8,
                Keycode::Num8 => b'*' as u8,
                Keycode::Num9 => b'(' as u8,
                Keycode::Num0 => b')' as u8,
                Keycode::Equals => b'+' as u8,
                Keycode::Semicolon => b':' as u8,
                Keycode::Quote => b'"' as u8,
                Keycode::Comma => b'<' as u8,
                Keycode::Period => b'>' as u8,
                Keycode::Slash => b'?' as u8,
                _ => ch,
            };
        }
        if keystate.intersects(keyboard::LCTRLMOD | keyboard::RCTRLMOD) {
            ch = match code {
                Keycode::A => 0x81,
                Keycode::B => 0x82,
                Keycode::C => 0x83,
                Keycode::D => 0x84,
                Keycode::E => 0x85,
                Keycode::F => 0x86,
                Keycode::G => 0x87,
                Keycode::H => 0x88,
                Keycode::I => 0x89,
                Keycode::J => 0x8A,
                Keycode::K => 0x8B,
                Keycode::L => 0x8C,
                Keycode::M => 0x8D,
                Keycode::N => 0x8E,
                Keycode::O => 0x8F,
                Keycode::P => 0x90,
                Keycode::Q => 0x91,
                Keycode::R => 0x92,
                Keycode::S => 0x93,
                Keycode::T => 0x94,
                Keycode::U => 0x95,
                Keycode::V => 0x96,
                Keycode::W => 0x97,
                Keycode::X => 0x98,
                Keycode::Y => 0x99,
                Keycode::Z => 0x9A,
                _ => ch,
            };
            if keystate.intersects(keyboard::LSHIFTMOD | keyboard::RSHIFTMOD) {
                ch = match code {
                    Keycode::M => 0x9D,
                    Keycode::N => 0x9E,
                    Keycode::P => 0x80,
                    _ => ch,
                }
            }
        }
        Some(ch | 0x80)
    }
}

pub struct KeyboardInputs<'a> {
    input: &'a mut Input,
}

impl<'a> Iterator for KeyboardInputs<'a> {
    type Item = KeyboardInput;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(event) = self.input.events.poll_event() {
                match event {
                    Event::Quit { .. } => return Some(KeyboardInput::Quit),
                    Event::KeyDown { keycode, .. } => {
                        if keycode == Some(Keycode::F2) {
                            return Some(KeyboardInput::Reset);
                        }
                        else if let Some(val) = Input::map_keycode(keycode, &self.input.keyboard) {
                            return Some(KeyboardInput::Key(val));
                        }
                    }
                    _ => {}
                }
            }
            else {
                return None;
            }
        }
    }
}

pub enum KeyboardInput {
    Quit,
    Reset,
    Key(u8),
}
