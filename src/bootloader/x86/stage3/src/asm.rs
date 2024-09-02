extern "cdecl" {
    pub fn outb(port: u16, value: u8) -> u8;
    pub fn inb(port: u16) -> u8;
}
