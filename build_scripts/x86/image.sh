#!/usr/bin/sh

set -e

BUILD_DIR=build
SRC_DIR=src

dd if=/dev/zero of=${BUILD_DIR}/x86.img bs=1048576 count=128

# create partition table
sfdisk --wipe always ${BUILD_DIR}/x86.img < ${SRC_DIR}/x86_partition_table.txt

loop_device=$(sudo losetup --show -fP build/x86.img)
echo Loopback device: ${loop_device}

# MBR Loader
sudo dd if=${BUILD_DIR}/x86/bootloader.bin of=${loop_device} bs=446 count=1 conv=notrunc

# Stage 2
sudo dd if=${BUILD_DIR}/x86/bootloader_stage2.bin of=${loop_device} bs=512 seek=1 conv=notrunc

# Stage 3
sudo dd if=${BUILD_DIR}/x86/bootloader_stage3.bin of=${loop_device} bs=512 seek=17 conv=notrunc

sudo mkfs.fat -F 32 -n "AilPhauneOS" ${loop_device}p1

mkdir -p partition1
sudo mount ${loop_device}p1 partition1

echo Hello World > partition1/hello.txt

sudo umount partition1
rmdir partition1

sudo losetup -d ${loop_device}