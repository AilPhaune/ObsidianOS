use core::cell::SyncUnsafeCell;

use crate::asm::outb;

pub trait DebugDriver {
    fn write_char(&mut self, character: u8);
    fn write_chars(&mut self, characters: &[u8]);
}

pub struct E9HackDebug {}

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
}

static E9_DEBUG: SyncUnsafeCell<E9HackDebug> = SyncUnsafeCell::new(E9HackDebug {});

pub fn get_debug() -> &'static mut impl DebugDriver {
    unsafe { &mut *(E9_DEBUG.get()) }
}
