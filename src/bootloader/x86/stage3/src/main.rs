#![no_std]
#![no_main]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn rust_entry(disk: u8) -> ! {
    // Your main function code here
    loop {
        loop {}
    }
}
