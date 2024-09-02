#![no_std]
#![no_main]
#![feature(sync_unsafe_cell)]

use debug::{get_debug, DebugDriver};
use interrupts::{
    initialize_idt,
    irq::{initialize_irq, register_irq_handler},
    pic::get_pic_driver,
    InterruptData,
};
use memory::allocators::hardcoded::HardcodedAddressAllocator;
use video::{Color, Cursor, Video};

pub mod asm;
pub mod debug;
pub mod interrupts;
pub mod memory;
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

/// # Safety
/// It's memset
#[no_mangle]
pub unsafe extern "C" fn memset(dest: *mut u8, src: *mut u8, count: usize) {
    for i in 0..count {
        *(dest.add(i)) = *(src.add(i));
    }
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

pub fn unwrap<T>(option: Option<T>, message: &[u8]) -> T {
    match option {
        None => {
            Video::println(b"Unwrapped a None option:", Color::LightRed, Color::Black);
            Video::println(message, Color::LightRed, Color::Black);
            kpanic();
        }
        Some(t) => t,
    }
}

pub fn ignore_irq(_: &mut InterruptData) {}

pub fn on_timer(_data: &mut InterruptData) {}

#[no_mangle]
pub extern "cdecl" fn rust_entry(_boot_disk: usize) -> ! {
    unsafe {
        // 32Mb
        let allocator = HardcodedAddressAllocator::new(1024 * 1024, 32 * 1024 * 1024);

        let video = Video::get();
        video.set_color(Color::White, Color::Black);
        video.clear();

        clear_interrupts();
        initialize_idt();
        initialize_irq();
        unwrap(get_pic_driver(), b"PIC Driver unset").set_mask(0xFFFF);

        register_irq_handler(0, &on_timer);
        register_irq_handler(7, &ignore_irq);
        unwrap(get_pic_driver(), b"PIC Driver unset").unmask(0);
        enable_interrupts();

        Cursor::enable_cursor(0, 15);
        video.write_char(b'\n');
        video.set_color(Color::White, Color::Blue);
        video.write_string(b"Hello world !!\n");

        get_debug().write_chars(b"Stage 3 init\n");
    }
    #[allow(clippy::empty_loop)]
    loop {}
}
