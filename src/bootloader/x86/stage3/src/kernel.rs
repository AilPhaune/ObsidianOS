use crate::{
    fs::Bootable,
    kpanic,
    video::{Color, Video},
};

pub struct KernelLoader;

impl KernelLoader {
    pub fn load_kernel(_boot_device: Bootable, _boot_drive: usize) {
        // TODO: Load kernel
        Video::println(b"Work in progress.", Color::White, Color::Black);
        kpanic();
    }
}
