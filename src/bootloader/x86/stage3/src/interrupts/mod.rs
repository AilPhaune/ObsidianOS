pub mod i8259;
pub mod irq;
pub mod isr;
pub mod pic;

use core::{
    cell::SyncUnsafeCell,
    ops::{BitOr, BitOrAssign},
};

use isr::initialize_interrupt_serive_routines;

use crate::{
    integer_enum_impl, kpanic,
    video::{Color, Video},
};

extern "cdecl" {
    fn load_idt(ptr: *const IDTDescriptor);
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct IDTEntry {
    pub base_low: u16,
    pub segment_descriptor: u16,
    pub reserved: u8,
    pub flags: u8,
    pub base_high: u16,
}

impl IDTEntry {
    const fn zero() -> Self {
        Self {
            base_low: 0,
            segment_descriptor: 0,
            reserved: 0,
            flags: 0,
            base_high: 0,
        }
    }
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct IDTDescriptor {
    pub limit: u16,
    pub ptr: *const (),
}

unsafe impl Sync for IDTDescriptor {}

#[repr(u8)]
pub enum IDTFlagNumeric {
    GateTask = 0x5,
    GateInterrupt16Bit = 0x6,
    GateTrap16Bit = 0x7,
    GateInterrupt32Bit = 0xE,
    GateTrap32Bit = 0xF,

    Ring0 = (0 << 5),
    Ring1 = (1 << 5),
    Ring2 = (2 << 5),
    Ring3 = (3 << 5),

    Present = 0x80,
}

integer_enum_impl!(IDTFlagNumeric, u8);

#[repr(C, packed)]
pub struct InterruptData {
    // in reverse order they are pushed:
    pub ds: u32,
    pub edi: u32,
    pub esi: u32,
    pub ebp: u32,
    pub kernel_esp: u32,
    pub ebx: u32,
    pub edx: u32,
    pub ecx: u32,
    pub eax: u32,
    pub interrupt: u32,
    pub error_code: u32,
    pub eip: u32,
    pub cs: u32,
    pub eflags: u32,
    pub esp: u32,
    pub ss: u32,
}

static IDT_ENTRIES: SyncUnsafeCell<[IDTEntry; 256]> = SyncUnsafeCell::new([IDTEntry::zero(); 256]);
static IDT_DESCRIPTOR: SyncUnsafeCell<IDTDescriptor> = SyncUnsafeCell::new(IDTDescriptor {
    limit: (size_of::<IDTEntry>() * 256 - 1) as u16,
    ptr: 0 as _,
});

pub fn set_idt_gate(interrupt: u8, base: *const (), segment_descriptor: u16, flags: u8) {
    let idt = unsafe { &mut *IDT_ENTRIES.get() };
    let entry = &mut idt[interrupt as usize];
    let base = base as usize;
    entry.base_low = (base & 0xFFFF) as u16;
    entry.base_high = ((base >> 16) & 0xFFFF) as u16;
    entry.reserved = 0;
    entry.flags = flags;
    entry.segment_descriptor = segment_descriptor;
}

pub fn enable_idt_gate(interrupt: u8) {
    let idt = unsafe { &mut *IDT_ENTRIES.get() };
    let entry = &mut idt[interrupt as usize];
    entry.flags |= IDTFlagNumeric::Present as u8;
}

pub fn disable_idt_gate(interrupt: u8) {
    let idt = unsafe { &mut *IDT_ENTRIES.get() };
    let entry = &mut idt[interrupt as usize];
    entry.flags &= !(IDTFlagNumeric::Present as u8);
}

pub fn initialize_idt() {
    unsafe {
        let desc = IDT_DESCRIPTOR.get();
        let entries = IDT_ENTRIES.get();
        (*desc).ptr = &(*entries)[0] as *const IDTEntry as *const ();
        load_idt(IDT_DESCRIPTOR.get());
        initialize_interrupt_serive_routines();
        for i in 0..=255 {
            enable_idt_gate(i);
        }
    }
}

#[derive(Clone, Copy)]
enum InterruptHandlerEntry {
    Absent,
    Present(&'static dyn Fn(&mut InterruptData)),
}

unsafe impl Sync for InterruptHandlerEntry {}

static INTERRUPT_HANDLERS: SyncUnsafeCell<[InterruptHandlerEntry; 256]> =
    SyncUnsafeCell::new([InterruptHandlerEntry::Absent; 256]);

/// # Safety
/// DO NOT CALL. Called by assembly.
#[no_mangle]
pub unsafe extern "cdecl" fn stage3_isr_handler(interrupt_data: *mut InterruptData) {
    let handlers = INTERRUPT_HANDLERS.get();
    let interrupt = (*interrupt_data).interrupt;
    let handler = &(*handlers)[(interrupt & 0xFF) as usize];
    match handler {
        InterruptHandlerEntry::Absent => {
            let video = Video::get();
            video.set_color(Color::LightRed, Color::Black);
            video.scroll(u16::MAX);
            video.carriage_return();
            video.write_string(b"Unhandled interrupt: 0x");
            video.write_hex_u8(interrupt as u8);
            video.write_char(b'\n');
            kpanic();
        }
        InterruptHandlerEntry::Present(handler) => (**handler)(&mut *interrupt_data),
    }
}

pub fn register_interrupt_handler<T>(interrupt: u8, handler: &'static T)
where
    T: Fn(&mut InterruptData),
{
    unsafe {
        let handlers = INTERRUPT_HANDLERS.get();
        (*handlers)[interrupt as usize] = InterruptHandlerEntry::Present(handler);
    }
}
