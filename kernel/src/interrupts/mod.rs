
use lazy_static::lazy_static;
use pic8259::ChainedPics;
use spin::Mutex;
use x86_64::instructions::port::Port;
use x86_64::structures::idt::{
    InterruptDescriptorTable,
    InterruptStackFrame,
};
use core::sync::atomic::{AtomicU64, Ordering};

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static TICKS: AtomicU64 = AtomicU64::new(0);

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        idt[32].set_handler_fn(timer_interrupt_handler);

        idt
    };
}

pub static PICS: Mutex<ChainedPics> = Mutex::new(
    unsafe {
        ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET)
    }
);

extern "x86-interrupt" fn timer_interrupt_handler(
    _stack_frame: InterruptStackFrame,
) {
    TICKS.fetch_add(1, Ordering::Relaxed);

    unsafe {
        PICS.lock().notify_end_of_interrupt(PIC_1_OFFSET);
    }
}

pub fn init_idt() {
    IDT.load();
}

pub fn init_pics() {
    unsafe {
        PICS.lock().initialize();
    }
}

pub fn init_pit(frequency_hz: u32) {
    let frequency = frequency_hz.clamp(1, 1000);
    let divisor = (1_193_182 / frequency) as u16;

    unsafe {
        let mut command: Port<u8> = Port::new(0x43);
        let mut channel_0: Port<u8> = Port::new(0x40);

        command.write(0x36);
        channel_0.write((divisor & 0xFF) as u8);
        channel_0.write((divisor >> 8) as u8);
    }
}
