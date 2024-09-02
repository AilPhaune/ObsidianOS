#![no_std]
#![no_main]
#![feature(sync_unsafe_cell)]

use interrupts::initialize_idt;
use video::{Color, Cursor, Video};

pub mod asm;
pub mod interrupts;
pub mod isr;
pub mod video;

extern "cdecl" {
    pub fn stage3_entry();
    pub fn clear_interrupts();
    pub fn enable_interrupts();
    pub fn inter();
}

#[panic_handler]
pub fn panic(_info: &core::panic::PanicInfo) -> ! {
    kpanic();
}

pub fn kpanic() -> ! {
    unsafe {
        let video = Video::get();
        video.set_color(Color::Black, Color::Red);
        video.write_string(b"BOOTLOADER PANIC /!\\\n");

        clear_interrupts();
    }

    #[allow(clippy::empty_loop)]
    loop {}
}

#[no_mangle]
pub extern "cdecl" fn rust_entry(_disk: u8) -> ! {
    unsafe {
        clear_interrupts();
        initialize_idt();
        enable_interrupts();

        Cursor::enable_cursor(0, 15);
        let video = Video::get();
        video.set_color(Color::White, Color::Blue);
        video.scroll(u16::MAX);
        video.write_string(b"Hello world !!");
    }
    #[allow(clippy::empty_loop)]
    loop {}
}
