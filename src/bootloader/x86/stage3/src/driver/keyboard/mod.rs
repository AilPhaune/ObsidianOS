use core::ptr::addr_of_mut;

use ps2::init_ps2_keyboard;

use crate::{
    datastructures::queue::Queue,
    kpanic,
    memory::MemoryAllocator,
    unwrap,
    video::{Color, Video},
};

pub mod ps2;

static mut KEY_QUEUE: Option<Queue<KeyEvent>> = None;

pub fn init_keyboard_driver(allocator: &mut dyn MemoryAllocator) {
    unsafe {
        let queue_ptr = unwrap(
            allocator.allocate(size_of::<KeyEvent>() * 100),
            b"Failed to allocate memory for Keyboard Event Queue.",
        );
        KEY_QUEUE = Some(Queue::new(queue_ptr.with_type().get(), 100));

        let video = Video::get();
        video.set_color(Color::White, Color::Black);
        video.write_string(b"Initializing keyboard driver...\nPS/2 Keyboard driver");
        if init_ps2_keyboard() {
            video.set_writing_column(-7);
            video.set_color(Color::LightGreen, Color::Black);
            video.write_string(b"SUCCESS\n");
        } else {
            video.set_writing_column(-6);
            video.set_color(Color::LightRed, Color::Black);
            video.write_string(b"FAILED\n");
            kpanic();
        }
    }
}

pub fn poll_key_event() -> Option<KeyEvent> {
    unsafe {
        match &mut *addr_of_mut!(KEY_QUEUE) {
            None => None,
            Some(queue) => queue.get().copied(),
        }
    }
}

pub fn consume_key_event() -> Option<KeyEvent> {
    unsafe {
        match &mut *addr_of_mut!(KEY_QUEUE) {
            None => None,
            Some(queue) => queue.dequeue(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum KeyEventKind {
    Press,
    Hold,
    Release,
}

#[derive(Clone, Copy)]
pub enum Key {
    Char(u8),
    FunctionKey(u8),
    Keypad(u8),
    Space,
    Tab,
    Backtick,
    LeftAlt,
    LeftShift,
    LeftControl,
    CapsLock,
    RightShift,
    Enter,
    KeypadEnter,
    Backspace,
    Escape,
    NumberLock,
    ScrollLock,
    RightAlt,
    WWWSearch,
    RightControl,
    MultimediaPreviousTrack,
    WWWFavorites,
    LeftGui,
    WWWRefresh,
    MultimediaVolumeDown,
    MultimediaMute,
    RightGui,
    WWWStop,
    MultimediaCalculator,
    Apps,
    WWWForward,
    MultimediaVolumeUp,
    MultimediaPlayPause,
    ACPIPower,
    WWWBack,
    WWWHome,
    MultimediaStop,
    ACPISleep,
    MultimediaMyComputer,
    MultimediaEmail,
    MultimediaNextTrack,
    MultimediaMediaSelect,
    ACPIWake,
    End,
    CursorLeft,
    Home,
    Insert,
    Delete,
    CursorDown,
    CursorRight,
    CursorUp,
    PageDown,
    PageUp,
    PrintScreen,
    Pause,
    None,
}

#[derive(Clone, Copy)]
pub enum KeyLocation {
    Normal,
    Keypad,
}

#[derive(Clone, Copy)]
pub struct KeyEvent {
    pub kind: KeyEventKind,
    pub scan_code: u64,
    pub key: Key,
    pub location: KeyLocation,
}
