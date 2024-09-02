use core::cell::SyncUnsafeCell;

use crate::{
    kpanic,
    video::{Color, Video},
};

use super::{
    i8259::{i8259_configure, i8259_get_driver},
    irq::PIC_REMAP_OFFSET,
};

pub trait PICDriver {
    fn get_name(&mut self) -> &[u8];
    fn enable(&mut self);
    fn disable(&mut self);
    fn probe(&mut self) -> bool;
    fn send_end_of_interrupt(&mut self, irq: u8);
    fn mask(&mut self, irq: u8);
    fn unmask(&mut self, irq: u8);
    fn set_mask(&mut self, mask: u16);
}

#[derive(Clone, Copy)]
pub enum PICDriverOption {
    None,
    Driver(*mut dyn PICDriver),
}

unsafe impl Sync for PICDriverOption {}

static PIC_DRIVER: SyncUnsafeCell<PICDriverOption> = SyncUnsafeCell::new(PICDriverOption::None);

pub fn set_pic_driver(driver: &'static mut dyn PICDriver) {
    unsafe {
        (*PIC_DRIVER.get()) = PICDriverOption::Driver(driver);
    }
}

pub fn get_pic_driver() -> Option<&'static mut dyn PICDriver> {
    unsafe {
        match *(PIC_DRIVER.get()) {
            PICDriverOption::None => None,
            PICDriverOption::Driver(driver) => Some(&mut *driver),
        }
    }
}

pub fn initialize_pic() {
    unsafe {
        let video = Video::get();
        video.set_color(Color::White, Color::Black);
        video.write_string(b"Initializing PIC...\n");
    }
    if i8259_configure(PIC_REMAP_OFFSET, PIC_REMAP_OFFSET + 8, true) {
        set_pic_driver(i8259_get_driver());
    }

    if get_pic_driver().is_none() {
        unsafe {
            let video = Video::get();
            video.set_color(Color::Red, Color::Black);
            video.write_string(b"\nCouldn't load a PIC driver\n");
            kpanic();
        }
    }
}
