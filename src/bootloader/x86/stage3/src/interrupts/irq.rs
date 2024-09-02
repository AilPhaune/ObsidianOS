use core::cell::SyncUnsafeCell;

use crate::{
    interrupts::{register_interrupt_handler, InterruptData},
    kpanic,
    video::{Color, Video},
};

use super::pic::{get_pic_driver, initialize_pic};

#[derive(Clone, Copy)]
enum IRQHandlerEntry {
    Absent,
    Present(&'static dyn Fn(&mut InterruptData)),
}

unsafe impl Sync for IRQHandlerEntry {}

pub const PIC_REMAP_OFFSET: u8 = 0x20;
static HANDLERS: SyncUnsafeCell<[IRQHandlerEntry; 16]> =
    SyncUnsafeCell::new([IRQHandlerEntry::Absent; 16]);

fn irq_handler(data: &mut InterruptData) {
    unsafe {
        let Some(driver) = get_pic_driver() else {
            return;
        };
        let irq = data.interrupt as u8 - PIC_REMAP_OFFSET;
        let handlers = &*HANDLERS.get();
        if let Some(handler) = handlers.get(irq as usize) {
            match handler {
                IRQHandlerEntry::Absent => {
                    let video = Video::get();
                    video.set_color(Color::LightRed, Color::Black);
                    video.scroll(u16::MAX);
                    video.carriage_return();
                    video.write_string(b"Unhandled hardware interrupt: 0x");
                    video.write_hex_u8(irq);
                    video.write_char(b'\n');
                    driver.send_end_of_interrupt(irq);
                    kpanic();
                }
                IRQHandlerEntry::Present(handler) => {
                    handler(data);
                    driver.send_end_of_interrupt(irq);
                }
            }
        }
    }
}

pub fn initialize_irq() {
    initialize_pic();

    // register ISR handler
    for i in 0..16 {
        register_interrupt_handler(PIC_REMAP_OFFSET + i, &irq_handler);
    }
}

pub fn register_irq_handler(irq: u8, handler: &'static dyn Fn(&mut InterruptData)) {
    unsafe {
        if irq < 16 {
            (*HANDLERS.get())[irq as usize] = IRQHandlerEntry::Present(handler);
        }
    }
}
