use core::ptr::addr_of_mut;

use super::{Key, KeyEventKind, KeyLocation, KEY_QUEUE};
use crate::{
    asm::inb,
    interrupts::{irq::register_irq_handler, pic::get_pic_driver, InterruptData},
    kpanic, unwrap,
};

//const PORT_STATUS_REGISTER: u16 = 0x64;
const PORT_DATA_REGISTER: u16 = 0x60;

pub fn init_ps2_keyboard() -> bool {
    register_irq_handler(1, &ps2_interrupt_handler);
    unwrap(get_pic_driver(), b"Unexpected: No PIC driver").unmask(1);

    true
}

fn ps2_interrupt_handler(_: &mut InterruptData) {
    unsafe {
        let scancode = inb(PORT_DATA_REGISTER);

        // TODO: Handle other scancode sets
        handle_scanode_set1(scancode);
    }
}

#[derive(Clone, Copy)]
enum ScancodeStateSet1 {
    Empty,
    E0,
    E1(usize, usize),
}

#[derive(Clone, Copy)]
enum ScancodeSet1PrintScreenState {
    Empty,
    Present(u8),
}

static mut SCANCODE_STATE_SET1: ScancodeStateSet1 = ScancodeStateSet1::Empty;
static mut SCANCODE_SET1_PRINT_SCREEN_STATE: ScancodeSet1PrintScreenState =
    ScancodeSet1PrintScreenState::Empty;

macro_rules! add_key_to_queue {
    ($key: ident, $is_release: ident, $scancode: ident) => {
        let event = $crate::driver::keyboard::KeyEvent {
            $key,
            kind: if $is_release {
                KeyEventKind::Release
            } else {
                KeyEventKind::Press
            },
            scan_code: $scancode as u64,
            location: match $key {
                Key::Keypad(_) => KeyLocation::Keypad,
                Key::KeypadEnter => KeyLocation::Keypad,
                _ => KeyLocation::Normal,
            },
        };

        if let Some(queue) = &mut *addr_of_mut!(KEY_QUEUE) {
            queue.add(event);
        }
    };
}

fn handle_scanode_set1(scancode: u8) {
    unsafe {
        match SCANCODE_STATE_SET1 {
            ScancodeStateSet1::Empty => match scancode {
                0xE0 => SCANCODE_STATE_SET1 = ScancodeStateSet1::E0,
                0xE1 => SCANCODE_STATE_SET1 = ScancodeStateSet1::E1(1, 0xE1),
                _ => {
                    let is_release = scancode & 0x80 == 0x80;
                    let key_scan_code = scancode & !0x80;

                    let key = match key_scan_code {
                        0x01 => Key::Escape,
                        0x02 => Key::Char(b'1'),
                        0x03 => Key::Char(b'2'),
                        0x04 => Key::Char(b'3'),
                        0x05 => Key::Char(b'4'),
                        0x06 => Key::Char(b'5'),
                        0x07 => Key::Char(b'6'),
                        0x08 => Key::Char(b'7'),
                        0x09 => Key::Char(b'8'),
                        0x0A => Key::Char(b'9'),
                        0x0B => Key::Char(b'0'),
                        0x0C => Key::Char(b'-'),
                        0x0D => Key::Char(b'='),
                        0x0E => Key::Backspace,
                        0x0F => Key::Tab,
                        0x10 => Key::Char(b'q'),
                        0x11 => Key::Char(b'w'),
                        0x12 => Key::Char(b'e'),
                        0x13 => Key::Char(b'r'),
                        0x14 => Key::Char(b't'),
                        0x15 => Key::Char(b'y'),
                        0x16 => Key::Char(b'u'),
                        0x17 => Key::Char(b'i'),
                        0x18 => Key::Char(b'o'),
                        0x19 => Key::Char(b'p'),
                        0x1A => Key::Char(b'['),
                        0x1B => Key::Char(b']'),
                        0x1C => Key::Enter,
                        0x1D => Key::LeftControl,
                        0x1E => Key::Char(b'a'),
                        0x1F => Key::Char(b's'),
                        0x20 => Key::Char(b'd'),
                        0x21 => Key::Char(b'f'),
                        0x22 => Key::Char(b'g'),
                        0x23 => Key::Char(b'h'),
                        0x24 => Key::Char(b'j'),
                        0x25 => Key::Char(b'k'),
                        0x26 => Key::Char(b'l'),
                        0x27 => Key::Char(b';'),
                        0x28 => Key::Char(b'\''),
                        0x29 => Key::Backtick,
                        0x2A => Key::LeftShift,
                        0x2B => Key::Char(b'\\'),
                        0x2C => Key::Char(b'z'),
                        0x2D => Key::Char(b'x'),
                        0x2E => Key::Char(b'c'),
                        0x2F => Key::Char(b'v'),
                        0x30 => Key::Char(b'b'),
                        0x31 => Key::Char(b'n'),
                        0x32 => Key::Char(b'm'),
                        0x33 => Key::Char(b','),
                        0x34 => Key::Char(b'.'),
                        0x35 => Key::Char(b'/'),
                        0x36 => Key::RightShift,
                        0x37 => Key::Keypad(b'*'),
                        0x38 => Key::LeftAlt,
                        0x39 => Key::Space,
                        0x3A => Key::CapsLock,
                        0x3B => Key::FunctionKey(1),
                        0x3C => Key::FunctionKey(2),
                        0x3D => Key::FunctionKey(3),
                        0x3E => Key::FunctionKey(4),
                        0x3F => Key::FunctionKey(5),
                        0x40 => Key::FunctionKey(6),
                        0x41 => Key::FunctionKey(7),
                        0x42 => Key::FunctionKey(8),
                        0x43 => Key::FunctionKey(9),
                        0x44 => Key::FunctionKey(10),
                        0x45 => Key::NumberLock,
                        0x46 => Key::ScrollLock,
                        0x47 => Key::Keypad(b'7'),
                        0x48 => Key::Keypad(b'8'),
                        0x49 => Key::Keypad(b'9'),
                        0x4A => Key::Keypad(b'-'),
                        0x4B => Key::Keypad(b'4'),
                        0x4C => Key::Keypad(b'5'),
                        0x4D => Key::Keypad(b'6'),
                        0x4E => Key::Keypad(b'+'),
                        0x4F => Key::Keypad(b'1'),
                        0x50 => Key::Keypad(b'2'),
                        0x51 => Key::Keypad(b'3'),
                        0x52 => Key::Keypad(b'0'),
                        0x53 => Key::Keypad(b'.'),
                        0x54 => Key::None,
                        0x55 => Key::None,
                        0x56 => Key::None,
                        0x57 => Key::FunctionKey(11),
                        0x58 => Key::FunctionKey(12),
                        _ => Key::None,
                    };

                    add_key_to_queue!(key, is_release, scancode);
                }
            },
            ScancodeStateSet1::E0 => match SCANCODE_SET1_PRINT_SCREEN_STATE {
                ScancodeSet1PrintScreenState::Present(data) => {
                    SCANCODE_SET1_PRINT_SCREEN_STATE = ScancodeSet1PrintScreenState::Empty;
                    let is_release = data == 0xB7;

                    if (scancode != 0x37 && !is_release) || (scancode != 0xAA && is_release) {
                        return;
                    }

                    let large_scancode: u64 = if is_release { 0xE0B7E0AA } else { 0xE02AE037 };

                    let key = Key::PrintScreen;
                    add_key_to_queue!(key, is_release, large_scancode);
                }
                _ => {
                    if scancode == 0x2A || scancode == 0xB7 {
                        SCANCODE_SET1_PRINT_SCREEN_STATE =
                            ScancodeSet1PrintScreenState::Present(scancode);
                        return;
                    }
                    let is_release = scancode & 0x80 == 0x80;
                    let key_scan_code = scancode & !0x80;

                    let key = match key_scan_code {
                        0x10 => Key::MultimediaPreviousTrack,
                        0x19 => Key::MultimediaNextTrack,
                        0x1C => Key::KeypadEnter,
                        0x1D => Key::RightControl,
                        0x20 => Key::MultimediaMute,
                        0x21 => Key::MultimediaCalculator,
                        0x22 => Key::MultimediaPlayPause,
                        0x24 => Key::MultimediaStop,
                        0x2E => Key::MultimediaVolumeDown,
                        0x30 => Key::MultimediaVolumeUp,
                        0x32 => Key::WWWHome,
                        0x35 => Key::Keypad(b'/'),
                        0x38 => Key::RightAlt,
                        0x47 => Key::Home,
                        0x48 => Key::CursorUp,
                        0x49 => Key::PageUp,
                        0x4B => Key::CursorLeft,
                        0x4D => Key::CursorRight,
                        0x4F => Key::End,
                        0x50 => Key::CursorDown,
                        0x51 => Key::PageDown,
                        0x52 => Key::Insert,
                        0x53 => Key::Delete,
                        0x5B => Key::LeftGui,
                        0x5C => Key::RightGui,
                        0x5D => Key::Apps,
                        0x5E => Key::ACPIPower,
                        0x5F => Key::ACPISleep,
                        0x63 => Key::ACPIWake,
                        0x65 => Key::WWWSearch,
                        0x66 => Key::WWWFavorites,
                        0x67 => Key::WWWRefresh,
                        0x68 => Key::WWWStop,
                        0x69 => Key::WWWForward,
                        0x6A => Key::WWWBack,
                        0x6B => Key::MultimediaMyComputer,
                        0x6C => Key::MultimediaEmail,
                        0x6D => Key::MultimediaMediaSelect,
                        _ => Key::None,
                    };

                    let large_scancode: u64 = 0xE000 | scancode as u64;

                    add_key_to_queue!(key, is_release, large_scancode);
                }
            },
            ScancodeStateSet1::E1(count, data) => {
                let new_data = (data << 8) | scancode as usize;
                SCANCODE_STATE_SET1 = ScancodeStateSet1::E1(count + 1, new_data);

                match count {
                    1 => {
                        if scancode == 0x45 || scancode == 0x9D {
                            return;
                        }
                        // Unknown key
                        SCANCODE_STATE_SET1 = ScancodeStateSet1::Empty;
                    }
                    2 => {
                        SCANCODE_STATE_SET1 = ScancodeStateSet1::Empty;
                        match new_data {
                            0xE11D45 => {
                                let key = Key::Pause;
                                let is_release = false;
                                add_key_to_queue!(key, is_release, new_data);
                            }
                            0xE19DC5 => {
                                let key = Key::Pause;
                                let is_release = false;
                                add_key_to_queue!(key, is_release, new_data);
                            }
                            _ => {
                                // Unknown key
                            }
                        }
                    }
                    _ => kpanic(),
                }
            }
        }
    }
}

/* TODO
#[derive(Clone, Copy)]
enum ScancodeStateSet2 {
    Empty,
    E0,
    E0F0,
    F0,
    E1,
}

static mut SCANCODE_STATE_SET2: ScancodeStateSet2 = ScancodeStateSet2::Empty;

fn handle_scanode_set2(scancode: u8) {
    unsafe {
        match SCANCODE_STATE_SET2 {
            ScancodeStateSet2::Empty => match scancode {
                0xE0 => SCANCODE_STATE_SET2 = ScancodeStateSet2::E0,
                0xE1 => SCANCODE_STATE_SET2 = ScancodeStateSet2::E1,
                0xF0 => SCANCODE_STATE_SET2 = ScancodeStateSet2::F0,
                _ => {
                    let is_release = scancode & 0x80 == 0x80;
                    let key_scan_code = scancode & !0x80;

                    let key = match key_scan_code {
                        0x01 => Key::FunctionKey(9),
                        0x02 => Key::None,
                        0x03 => Key::FunctionKey(5),
                        0x04 => Key::FunctionKey(3),
                        0x05 => Key::FunctionKey(1),
                        0x06 => Key::FunctionKey(2),
                        0x07 => Key::FunctionKey(12),
                        0x08 => Key::None,
                        0x09 => Key::FunctionKey(10),
                        0x0A => Key::FunctionKey(8),
                        0x0B => Key::FunctionKey(6),
                        0x0C => Key::FunctionKey(4),
                        0x0D => Key::Tab,
                        0x0E => Key::Backtick,
                        0x0F => Key::None,
                        0x10 => Key::None,
                        0x11 => Key::LeftAlt,
                        0x12 => Key::LeftShift,
                        0x13 => Key::None,
                        0x14 => Key::LeftControl,
                        0x15 => Key::Char(b'q'),
                        0x16 => Key::Char(b'1'),
                        0x17 => Key::None,
                        0x18 => Key::None,
                        0x19 => Key::None,
                        0x1A => Key::Char(b'z'),
                        0x1B => Key::Char(b's'),
                        0x1C => Key::Char(b'a'),
                        0x1D => Key::Char(b'w'),
                        0x1E => Key::Char(b'2'),
                        0x1F => Key::None,
                        0x20 => Key::None,
                        0x21 => Key::Char(b'c'),
                        0x22 => Key::Char(b'x'),
                        0x23 => Key::Char(b'd'),
                        0x24 => Key::Char(b'e'),
                        0x25 => Key::Char(b'4'),
                        0x26 => Key::Char(b'3'),
                        0x27 => Key::None,
                        0x28 => Key::None,
                        0x29 => Key::Space,
                        0x2A => Key::Char(b'v'),
                        0x2B => Key::Char(b'f'),
                        0x2C => Key::Char(b't'),
                        0x2D => Key::Char(b'r'),
                        0x2E => Key::Char(b'5'),
                        0x2F => Key::None,
                        0x30 => Key::None,
                        0x31 => Key::Char(b'n'),
                        0x32 => Key::Char(b'b'),
                        0x33 => Key::Char(b'h'),
                        0x34 => Key::Char(b'g'),
                        0x35 => Key::Char(b'y'),
                        0x36 => Key::Char(b'6'),
                        0x37 => Key::None,
                        0x38 => Key::None,
                        0x39 => Key::None,
                        0x3A => Key::Char(b'm'),
                        0x3B => Key::Char(b'j'),
                        0x3C => Key::Char(b'u'),
                        0x3D => Key::Char(b'7'),
                        0x3E => Key::Char(b'8'),
                        0x3F => Key::None,
                        0x40 => Key::None,
                        0x41 => Key::Char(b','),
                        0x42 => Key::Char(b'k'),
                        0x43 => Key::Char(b'i'),
                        0x44 => Key::Char(b'o'),
                        0x45 => Key::Char(b'0'),
                        0x46 => Key::Char(b'9'),
                        0x47 => Key::None,
                        0x48 => Key::None,
                        0x49 => Key::Char(b'.'),
                        0x4A => Key::Char(b'/'),
                        0x4B => Key::Char(b'l'),
                        0x4C => Key::Char(b';'),
                        0x4D => Key::Char(b'p'),
                        0x4E => Key::Char(b'-'),
                        0x4F => Key::None,
                        0x50 => Key::None,
                        0x51 => Key::None,
                        0x52 => Key::Char(b'\''),
                        0x53 => Key::None,
                        0x54 => Key::Char(b'['),
                        0x55 => Key::Char(b'='),
                        0x56 => Key::None,
                        0x57 => Key::None,
                        0x58 => Key::CapsLock,
                        0x59 => Key::RightShift,
                        0x5A => Key::Enter,
                        0x5B => Key::Char(b']'),
                        0x5C => Key::None,
                        0x5D => Key::Char(b'\\'),
                        0x5E => Key::None,
                        0x5F => Key::None,
                        0x60 => Key::None,
                        0x61 => Key::None,
                        0x62 => Key::None,
                        0x63 => Key::None,
                        0x64 => Key::None,
                        0x65 => Key::None,
                        0x66 => Key::Backspace,
                        0x67 => Key::None,
                        0x68 => Key::None,
                        0x69 => Key::Keypad(b'1'),
                        0x6A => Key::None,
                        0x6B => Key::Keypad(b'4'),
                        0x6C => Key::Keypad(b'7'),
                        0x6D => Key::None,
                        0x6E => Key::None,
                        0x6F => Key::None,
                        0x70 => Key::Keypad(b'0'),
                        0x71 => Key::Keypad(b'.'),
                        0x72 => Key::Keypad(b'2'),
                        0x73 => Key::Keypad(b'5'),
                        0x74 => Key::Keypad(b'6'),
                        0x75 => Key::Keypad(b'8'),
                        0x76 => Key::Escape,
                        0x77 => Key::NumberLock,
                        0x78 => Key::FunctionKey(11),
                        0x79 => Key::Keypad(b'+'),
                        0x7A => Key::Keypad(b'3'),
                        0x7B => Key::Keypad(b'-'),
                        0x7C => Key::Keypad(b'*'),
                        0x7D => Key::Keypad(b'9'),
                        0x7E => Key::ScrollLock,
                        0x7F => Key::None,
                        _ => Key::None,
                    };

                    add_key_to_queue!(key, is_release, scancode);
                }
            },
            ScancodeStateSet2::E0 => match scancode {
                0xF0 => SCANCODE_STATE_SET2 = ScancodeStateSet2::E0F0,
                _ => kpanic(),
            },
            ScancodeStateSet2::F0 => kpanic(),
            ScancodeStateSet2::E0F0 => {
                kpanic();
            }
            ScancodeStateSet2::E1 => kpanic(),
        }
    }
}
*/
