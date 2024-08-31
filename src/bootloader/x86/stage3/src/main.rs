#![no_std]
#![no_main]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[derive(Clone, Copy)]
struct Character {
    character: u8,
    color: u8,
}

#[no_mangle]
pub extern "C" fn rust_entry(disk: u8) -> ! {
    unsafe {
        let video_memory: *mut Character = 0xB8000 as *mut Character;
        let test = "Hello World";
        for (i, c) in test.chars().enumerate() {
            let character = &mut *video_memory.add(i);
            character.character = c as u8;
            character.color = 0x0F;
        }
    }

    #[allow(clippy::empty_loop)]
    loop {}
}
