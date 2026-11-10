#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::fmt::Write;
use bootloader_api::BootInfo;
use uart_16550::SerialPort;

bootloader_api::entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static mut BootInfo) -> ! {
    unsafe {
        let mut serial = SerialPort::new(0x3F8);
        serial.init();

        writeln!(serial, "================================").unwrap();
        writeln!(serial, "       MY RUST MICRO-OS").unwrap();
        writeln!(serial, "================================").unwrap();
        writeln!(serial, "[BOOT] Kernel started successfully").unwrap();
        writeln!(serial, "[BOOT] Initializing kernel...").unwrap();
        writeln!(serial, "[BOOT] System is running!").unwrap();
    }

    loop {
        core::hint::spin_loop();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
