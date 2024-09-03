use core::cell::SyncUnsafeCell;

use crate::asm::outb;

pub trait DebugDriver {
    fn write_char(&mut self, character: u8);
    fn write_chars(&mut self, characters: &[u8]);
    fn write_hex_u8(&mut self, value: u8);
}

fn get_hex_digit(value: u8) -> u8 {
    if value >= 10 {
        b'A' - 10 + value
    } else {
        b'0' + value
    }
}

pub struct E9HackDebug;

impl DebugDriver for E9HackDebug {
    fn write_char(&mut self, character: u8) {
        unsafe {
            outb(0xE9, character);
        }
    }

    fn write_chars(&mut self, characters: &[u8]) {
        unsafe {
            for c in characters.iter() {
                outb(0xE9, *c);
            }
        }
    }

    fn write_hex_u8(&mut self, value: u8) {
        self.write_char(get_hex_digit(value >> 4));
        self.write_char(get_hex_digit(value & 0xF));
    }
}

impl DebugDriver for [&mut dyn DebugDriver] {
    fn write_char(&mut self, character: u8) {
        for driver in self.iter_mut() {
            driver.write_char(character);
        }
    }
    fn write_chars(&mut self, characters: &[u8]) {
        for driver in self.iter_mut() {
            driver.write_chars(characters);
        }
    }
    fn write_hex_u8(&mut self, value: u8) {
        for driver in self.iter_mut() {
            driver.write_hex_u8(value);
        }
    }
}

static E9_DEBUG: SyncUnsafeCell<E9HackDebug> = SyncUnsafeCell::new(E9HackDebug {});

pub fn get_debug() -> &'static mut impl DebugDriver {
    unsafe { &mut *(E9_DEBUG.get()) }
}
