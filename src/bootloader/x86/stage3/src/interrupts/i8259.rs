use core::{
    cell::SyncUnsafeCell,
    ops::{BitOr, BitOrAssign},
};

use crate::{
    asm::{inb, iowait, outb},
    integer_enum_impl,
    video::{Color, Video},
};

use super::pic::PICDriver;

const PIC1_COMMAND_PORT: u16 = 0x20;
const PIC1_DATA_PORT: u16 = 0x21;
const PIC2_COMMAND_PORT: u16 = 0xA0;
const PIC2_DATA_PORT: u16 = 0xA1;

#[repr(u8)]
pub enum I8259Icw1 {
    Icw4 = 0x01,
    Single = 0x02,
    Interval4 = 0x04,
    Level = 0x08,
    Initialize = 0x10,
}
integer_enum_impl!(I8259Icw1, u8);

#[repr(u8)]
pub enum I8259Icw4 {
    Mode8086 = 0x01,
    AutoEoi = 0x02,
    BufferMaster = 0x04,
    BufferSlave = 0x00,
    Buffered = 0x08,
    Sfnm = 0x10,
}
integer_enum_impl!(I8259Icw4, u8);

#[repr(u8)]
pub enum I8259Command {
    EndOfInterrupt = 0x20,
    ReadIRR = 0x0A,
    ReadISR = 0x0B,
}
integer_enum_impl!(I8259Command, u8);

pub struct I8259Driver {
    current_mask: u16,
    auto_eoi: bool,
}

impl PICDriver for I8259Driver {
    fn get_name(&mut self) -> &[u8] {
        b"I8259 Driver"
    }
    fn enable(&mut self) {
        self.set_mask(0x0000);
    }
    fn disable(&mut self) {
        self.set_mask(0xFFFF);
    }
    fn mask(&mut self, irq: u8) {
        self.set_mask(self.current_mask | (1u16 << (irq as u16)))
    }
    fn unmask(&mut self, irq: u8) {
        self.set_mask(self.current_mask & !(1u16 << (irq as u16)))
    }
    fn probe(&mut self) -> bool {
        let prev_mask = i8259_get_mask();
        self.set_mask(0x4269);
        let got_mask = i8259_get_mask();
        self.set_mask(prev_mask);
        got_mask == 0x4269
    }
    fn send_end_of_interrupt(&mut self, irq: u8) {
        if self.auto_eoi {
            return;
        }
        unsafe {
            if irq >= 8 {
                outb(PIC2_COMMAND_PORT, I8259Command::EndOfInterrupt as u8);
            }
            outb(PIC1_COMMAND_PORT, I8259Command::EndOfInterrupt as u8);
        }
    }
    fn set_mask(&mut self, mask: u16) {
        unsafe {
            outb(PIC1_DATA_PORT, (mask & 0xFF) as u8);
            iowait();
            outb(PIC2_DATA_PORT, ((mask >> 8) & 0xFF) as u8);
            iowait();
            self.current_mask = mask;
        }
    }
}

static I8259_DRIVER: SyncUnsafeCell<I8259Driver> = SyncUnsafeCell::new(I8259Driver {
    current_mask: 0,
    auto_eoi: false,
});

pub fn i8259_configure(pic1_offset: u8, pic2_offset: u8, auto_eoi: bool) -> bool {
    unsafe {
        let driver = i8259_get_driver();
        let video = Video::get();
        video.set_color(Color::White, Color::Black);
        video.write_string(driver.get_name());
        if !driver.probe() {
            video.set_writing_column(-6);
            video.set_color(Color::LightRed, Color::Black);
            video.write_string(b"FAILED\n");
            return false;
        }
        video.set_writing_column(-7);
        video.set_color(Color::LightGreen, Color::Black);
        video.write_string(b"SUCCESS\n");
        driver.set_mask(0xFFFF);

        outb(PIC1_COMMAND_PORT, I8259Icw1::Icw4 | I8259Icw1::Initialize);
        iowait();
        outb(PIC2_COMMAND_PORT, I8259Icw1::Icw4 | I8259Icw1::Initialize);
        iowait();

        outb(PIC1_DATA_PORT, pic1_offset);
        iowait();
        outb(PIC2_DATA_PORT, pic2_offset);
        iowait();

        // Tell it that the 2nd I8259 is connected on port 2 (0b 0000 0100)
        outb(PIC1_DATA_PORT, 0x4);
        iowait();
        // Tell I82592 that its cascade identity
        outb(PIC2_DATA_PORT, 2);
        iowait();

        let mut icw4 = I8259Icw4::Mode8086 as u8;
        if auto_eoi {
            icw4 |= I8259Icw4::AutoEoi;
        }

        outb(PIC1_DATA_PORT, icw4);
        iowait();
        outb(PIC2_DATA_PORT, icw4);
        iowait();

        // Clear data lines
        driver.set_mask(0xFFFF);
    }
    true
}

pub fn i8259_get_driver() -> &'static mut I8259Driver {
    unsafe { &mut *(I8259_DRIVER.get()) }
}

pub fn i8259_read_irq_request_registers() -> u16 {
    unsafe {
        outb(PIC1_COMMAND_PORT, I8259Command::ReadIRR as u8);
        outb(PIC2_COMMAND_PORT, I8259Command::ReadIRR as u8);
        (inb(PIC1_COMMAND_PORT) as u16) | ((inb(PIC2_COMMAND_PORT) as u16) << 8)
    }
}

pub fn i8259_read_in_service_registers() -> u16 {
    unsafe {
        outb(PIC1_COMMAND_PORT, I8259Command::ReadISR as u8);
        outb(PIC2_COMMAND_PORT, I8259Command::ReadISR as u8);
        (inb(PIC1_COMMAND_PORT) as u16) | ((inb(PIC2_COMMAND_PORT) as u16) << 8)
    }
}

pub fn i8259_get_mask() -> u16 {
    unsafe { inb(PIC1_DATA_PORT) as u16 | ((inb(PIC2_DATA_PORT) as u16) << 8) }
}
