#![no_std]
#![no_main]

mod task;

use core::panic::PanicInfo;
use core::fmt::Write;
use bootloader_api::BootInfo;
use uart_16550::SerialPort;

use task::TaskManager;

bootloader_api::entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static mut BootInfo) -> ! {
    unsafe {
        let mut serial = SerialPort::new(0x3F8);
        serial.init();

        writeln!(serial, "================================").unwrap();
        writeln!(serial, "       MY RUST MICRO-OS").unwrap();
        writeln!(serial, "================================").unwrap();

        writeln!(serial, "[BOOT] Kernel started").unwrap();

        let mut manager = TaskManager::new();

        manager.create_task(1, "Task 1").unwrap();
        manager.create_task(2, "Task 2").unwrap();
        manager.create_task(3, "Task 3").unwrap();

        writeln!(serial, "[TASK] Three tasks created").unwrap();
        writeln!(serial, "[SCHEDULER] Ready Queue:").unwrap();

        while let Some(task) = manager.next_task() {
            writeln!(
                serial,
                "Task ID: {}, Name: {}, State: {:?}",
                task.id,
                task.name,
                task.state
            ).unwrap();
        }

        writeln!(serial, "[SCHEDULER] Queue is empty").unwrap();
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
