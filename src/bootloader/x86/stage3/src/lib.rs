#![no_std]
#![no_main]
#![feature(sync_unsafe_cell)]

use core::ptr::addr_of_mut;

use datastructures::vector::Vector;
use debug::{get_debug, DebugDriver};
use driver::keyboard::{consume_key_event, init_keyboard_driver, Key, KeyEventKind};
use fs::{detect_filesystem, Bootable, FileSystem};
use interrupts::{
    initialize_idt,
    irq::{initialize_irq, register_irq_handler},
    pic::get_pic_driver,
    InterruptData,
};
use kernel::KernelLoader;
use memory::{
    allocators::{arena::ArenaAllocator, hardcoded::HardcodedAddressAllocator},
    MemoryAllocator,
};
use video::{Color, Cursor, Video};

pub mod asm;
pub mod datastructures;
pub mod debug;
pub mod driver;
pub mod fs;
pub mod interrupts;
pub mod kernel;
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
/// It's memcpy
#[no_mangle]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *mut u8, count: usize) {
    for i in 0..count {
        *(dest.add(i)) = *(src.add(i));
    }
}

extern "cdecl" {
    pub fn stage3_entry();
    pub fn clear_interrupts();
    pub fn enable_interrupts();
}

#[panic_handler]
pub fn panic(_info: &core::panic::PanicInfo) -> ! {
    kpanic();
}

pub fn kpanic() -> ! {
    unsafe {
        clear_interrupts();

        let video = Video::get();
        video.set_color(Color::Black, Color::Red);
        video.write_string(b"BOOTLOADER PANIC /!\\\n");
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

static mut TIMER_COUNTER: u128 = 0;
static mut BLINK_COUNTER: u128 = 0;
static mut BEGIN_AT_TIMER_COUNTER: u128 = 0;
static mut SELECTED_PARTITION: isize = 0;

pub static mut BOOTABLE: Option<Vector<Bootable>> = None;

const BLINK_SPEED: u16 = 20;

pub fn on_timer(_data: &mut InterruptData) {
    unsafe {
        TIMER_COUNTER += 1;
        BLINK_COUNTER += 1;

        refresh_selection_panel();
    }
}

pub fn get_bootable_count() -> usize {
    unsafe {
        match &*addr_of_mut!(BOOTABLE) {
            None => 0,
            Some(vec) => vec.length(),
        }
    }
}

pub fn refresh_selection_panel() {
    unsafe {
        if TIMER_COUNTER >= BEGIN_AT_TIMER_COUNTER {
            let video = Video::get();
            let (x, y) = video.current_writing_position();
            video.set_writing_position(0, 0);

            video.set_color(Color::Yellow, Color::Black);
            video.write_centered_line(b"######################");
            video.write_centered_line(b"#                    #");
            video.write_centered_line(b"#   OBSIDIAN LOADER  #");
            video.write_centered_line(b"#                    #");
            video.write_centered_line(b"######################");

            video.clear_current_line();
            video.carriage_return();
            video.line_feed();
            video.clear_current_line();
            video.set_color(Color::Gray, Color::Black);
            video.write_string(b"Select boot partition using arrow keys:\n");

            let boot_count = get_bootable_count() as isize;
            if boot_count <= 0 {
                return;
            }
            let Some(bootable_devices) = &mut *addr_of_mut!(BOOTABLE) else {
                return;
            };
            SELECTED_PARTITION = ((SELECTED_PARTITION % boot_count) + boot_count) % boot_count;

            for i in 0..(boot_count as usize) {
                video.clear_current_line();

                if i as isize == SELECTED_PARTITION
                    && (BLINK_COUNTER as u16) % BLINK_SPEED <= (BLINK_SPEED / 2)
                {
                    video.write_string(b"--> ");
                } else {
                    video.write_string(b"    ");
                }

                let Some(bootable_device) = bootable_devices.get(i) else {
                    continue;
                };

                match bootable_device {
                    Bootable::Partition { fs, bootable, .. } => {
                        video.write_string(b"Partition ");
                        video.write_char(b'1' + i as u8);
                        video.write_string(b" - File system: ");
                        video.set_color(
                            match fs {
                                FileSystem::Unknwown => Color::LightRed,
                                _ => Color::Yellow,
                            },
                            Color::Black,
                        );
                        let fs_info = fs.info_string();
                        video.write_string(fs_info);
                        let mut j = fs_info.len();
                        while j < 8 {
                            video.write_char(b' ');
                            j += 1;
                        }
                        if *bootable {
                            video.set_color(Color::LightGreen, Color::Black);
                            video.write_string(b"[BOOTABLE: YES]");
                        } else {
                            video.set_color(Color::LightRed, Color::Black);
                            video.write_string(b"[BOOTABLE:  NO]");
                        }
                        video.set_color(Color::Gray, Color::Black);
                    }
                    Bootable::ObsidianKernel {
                        fs,
                        obsidian_kernel_verision,
                        ..
                    } => {
                        video.write_string(b"Obsidian kernel v");
                        video.write_string(obsidian_kernel_verision);
                        video.write_string(b" - File system: ");
                        video.set_color(
                            match fs {
                                FileSystem::Unknwown => Color::LightRed,
                                _ => Color::Yellow,
                            },
                            Color::Black,
                        );
                        video.write_string(fs.info_string());
                        video.set_color(Color::Gray, Color::Black);
                    }
                }

                video.line_feed();
                video.carriage_return();
                video.clear_current_line();
            }
            video.line_feed();

            if y >= video.current_writing_position().1 {
                video.set_writing_position(x as _, y as _);
            }
        }
    }
}

static mut MEMORY: HardcodedAddressAllocator<'static> =
    HardcodedAddressAllocator::new(1024 * 1024, 32 * 1024 * 1024);
pub static mut ALLOCATOR: Option<ArenaAllocator<'static>> = None;

#[no_mangle]
pub extern "cdecl" fn rust_entry(boot_drive: usize) -> ! {
    unsafe {
        ALLOCATOR = ArenaAllocator::<'static>::new(&mut *addr_of_mut!(MEMORY), 32 * 1024 * 1024);
        let allocator = unwrap(ALLOCATOR.as_mut(), b"Failed to allocate memory");

        BOOTABLE = Some(Vector::new(
            unwrap(
                allocator.allocate(20 * size_of::<Bootable>()),
                b"Failed to allocate memory for boot device data",
            )
            .with_type()
            .get(),
            20,
        ));
        detect_bootable_devices();

        let video = Video::get();
        video.set_color(Color::White, Color::Black);
        video.clear();

        clear_interrupts();
        initialize_idt();
        initialize_irq();
        unwrap(get_pic_driver(), b"PIC Driver unset").set_mask(0xFFFF);

        init_keyboard_driver(allocator);

        register_irq_handler(0, &on_timer);
        register_irq_handler(7, &ignore_irq);
        unwrap(get_pic_driver(), b"PIC Driver unset").unmask(0);
        enable_interrupts();

        get_debug().write_chars(b"Stage 3 (rust): Initialized\n");

        BEGIN_AT_TIMER_COUNTER = TIMER_COUNTER + 30;
        Cursor::disable_cursor();
    }
    loop {
        if let Some(key_event) = consume_key_event() {
            if key_event.kind == KeyEventKind::Press {
                match key_event.key {
                    Key::CursorUp => unsafe {
                        SELECTED_PARTITION -= 1;
                        BLINK_COUNTER = 0;
                        refresh_selection_panel();
                    },
                    Key::CursorDown => unsafe {
                        SELECTED_PARTITION += 1;
                        BLINK_COUNTER = 0;
                        refresh_selection_panel();
                    },
                    Key::Enter | Key::KeypadEnter => unsafe {
                        clear_interrupts();
                        refresh_selection_panel();
                        if let Some(boot_devices) = &*addr_of_mut!(BOOTABLE) {
                            if let Some(boot_device) =
                                boot_devices.get(SELECTED_PARTITION as _).cloned()
                            {
                                handle_selection(boot_device, boot_drive);
                            }
                        }
                        refresh_selection_panel();
                    },
                    _ => {}
                }
            }
        }
    }
}

pub fn handle_selection(boot_device: Bootable, boot_drive: usize) {
    match boot_device {
        Bootable::Partition { .. } => {
            unsafe {
                let video = Video::get();
                // TODO: Support loading partition bootloader
                video.write_string(b"Loading partition unsupported\n");
                enable_interrupts();
            }
        }
        Bootable::ObsidianKernel { .. } => {
            KernelLoader::load_kernel(boot_device, boot_drive);
        }
    }
}

pub fn detect_bootable_devices() {
    unsafe {
        for i in 1..=4 {
            let ptr = (0x7c00usize + 430 + 16 * i) as *mut u8;
            if (*ptr) & 0x80 != 0 {
                let device = Bootable::Partition {
                    fs: detect_filesystem((0x7c00usize + 512 * i) as _),
                    bootable: *((0x7c00usize + i * 512 + 510) as *mut u16) == 0xAA55,
                    lba: *(ptr.add(0x08) as *mut u32),
                    partition_length: *(ptr.add(0x0C) as *mut u32),
                };
                if let Some(bootable_devices) = &mut *addr_of_mut!(BOOTABLE) {
                    bootable_devices.push(device);
                }
            }
        }
    }
}
