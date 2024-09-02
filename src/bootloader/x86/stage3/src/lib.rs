#![no_std]
#![no_main]
#![feature(sync_unsafe_cell)]

use interrupts::{
    i8259::i8259_get_driver,
    initialize_idt,
    irq::{initialize_irq, register_irq_handler},
    pic::PICDriver,
    InterruptData,
};
use video::{Color, Cursor, Video};

pub mod asm;
pub mod interrupts;
pub mod video;

#[macro_export]
macro_rules! integer_enum_impl {
    ($enum_name: ident, $int_type: ident) => {
        impl BitOr<$enum_name> for $enum_name {
            type Output = $int_type;
            fn bitor(self, rhs: $enum_name) -> Self::Output {
                (self as $int_type) | (rhs as $int_type)
            }
        }

        impl BitOr<$enum_name> for $int_type {
            type Output = $int_type;
            fn bitor(self, rhs: $enum_name) -> Self::Output {
                self | (rhs as $int_type)
            }
        }

        impl BitOr<$int_type> for $enum_name {
            type Output = $int_type;
            fn bitor(self, rhs: $int_type) -> Self::Output {
                (self as $int_type) | rhs
            }
        }

        impl BitOrAssign<$enum_name> for $int_type {
            fn bitor_assign(&mut self, rhs: $enum_name) {
                *self = (*self) | rhs
            }
        }
    };
}

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

pub fn on_timer(_data: &mut InterruptData) {
    unsafe {
        Video::get().write_char(b'.');
    }
}

#[no_mangle]
pub extern "cdecl" fn rust_entry(_disk: u8) -> ! {
    unsafe {
        let video = Video::get();
        video.set_color(Color::White, Color::Black);
        video.clear();

        clear_interrupts();
        initialize_idt();
        initialize_irq();
        enable_interrupts();

        register_irq_handler(0, &on_timer);
        i8259_get_driver().unmask(0);

        Cursor::enable_cursor(0, 15);
        video.write_char(b'\n');
        video.set_color(Color::White, Color::Blue);
        video.write_string(b"Hello world !!\n");
    }
    #[allow(clippy::empty_loop)]
    loop {}
}
