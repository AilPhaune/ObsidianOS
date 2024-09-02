extern "cdecl" {
    pub fn outb(port: u16, value: u8) -> u8;
    pub fn inb(port: u16) -> u8;
}

const UNUSED_PORT: u16 = 0x80;
pub fn iowait() {
    unsafe { outb(UNUSED_PORT, 0) };
}
