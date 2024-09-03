#[derive(Clone, Copy)]
pub enum FileSystem {
    Fat12,
    Fat16,
    Fat32,
    ExFat,
    Ntfs,
    Ext2,
    Ext3,
    Ext4,
    Unknwown,
}

#[derive(Clone)]
pub enum Bootable<'a> {
    Partition {
        lba: u32,
        partition_length: u32,
        fs: FileSystem,
        bootable: bool,
    },
    ObsidianKernel {
        lba: u32,
        partition_length: u32,
        fs: FileSystem,
        obsidian_kernel_verision: &'a [u8],
    },
}

impl FileSystem {
    pub fn info_string(&self) -> &'static [u8] {
        match self {
            Self::Unknwown => b"Unknown",
            Self::Fat12 => b"FAT 12",
            Self::Fat16 => b"FAT 16",
            Self::Fat32 => b"FAT 32",
            Self::ExFat => b"ExFAT",
            Self::Ntfs => b"NTFS",
            Self::Ext2 => b"ext2",
            Self::Ext3 => b"ext3",
            Self::Ext4 => b"ext4",
        }
    }
}

fn compare_index(partition_data: *mut u8, offset: usize, data: &[u8]) -> bool {
    unsafe {
        for (i, character) in data.iter().enumerate() {
            if *(partition_data.add(i + offset)) != *character {
                return false;
            }
        }
    }
    true
}

pub fn detect_filesystem(partition_data: *mut u8) -> FileSystem {
    if compare_index(partition_data, 0x03, b"NTFS") {
        return FileSystem::Ntfs;
    }
    if compare_index(partition_data, 0x03, b"EXFAT") {
        return FileSystem::ExFat;
    }
    if compare_index(partition_data, 0x52, b"FAT32") {
        return FileSystem::Fat32;
    }
    if compare_index(partition_data, 0x36, b"FAT16") {
        return FileSystem::Fat16;
    }
    if compare_index(partition_data, 0x36, b"FAT12") {
        return FileSystem::Fat12;
    }

    // TODO: Detect ext2/ext3/ext4

    FileSystem::Unknwown
}
