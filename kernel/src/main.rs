

#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]
mod console;
mod task;
mod interrupts;
mod memory;
mod sync;
mod tests;


use console::{Console, Screen};
use core::fmt::Write;
use core::panic::PanicInfo;

use bootloader_api::BootInfo;
use uart_16550::SerialPort;
use task::TaskManager;

bootloader_api::entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    unsafe {
    let mut serial = SerialPort::new(0x3F8);
    serial.init();

    let screen = boot_info.framebuffer.as_mut().map(|fb| {
        let info = fb.info();
        let buffer = fb.buffer_mut();
        Screen::new(buffer, info)
    });

    let mut console = Console::new(serial, screen);

        writeln!(console, "================================").unwrap();
        writeln!(console, "       MY RUST MICRO-OS").unwrap();
        writeln!(console, "================================").unwrap();

        writeln!(console, "[BOOT] Kernel started").unwrap();

        // Round Robin scheduling
        let mut manager = TaskManager::new();

        manager.create_task(1, "Task 1").unwrap();
        manager.create_task(2, "Task 2").unwrap();
        manager.create_task(3, "Task 3").unwrap();

        writeln!(console, "[TASK] Three tasks created").unwrap();
        writeln!(console, "[SCHEDULER] Round Robin Started").unwrap();

        for tick in 1..=12 {
            if let Some(task) = manager.tick() {
                writeln!(
                    console,
                    "[TICK {}] Running: {} | ID: {} | Quantum: {}",
                    tick,
                    task.name,
                    task.id,
                    manager.quantum_remaining
                ).unwrap();
            }
        }

        writeln!(console, "[SCHEDULER] Simulation completed").unwrap();

        // Priority-based task selection demonstration
        writeln!(console, "[PRIORITY] Testing priority scheduler").unwrap();

        let mut priority_manager = TaskManager::new();

        priority_manager
            .create_task_with_priority(1, "Low Priority", 1)
            .unwrap();

        priority_manager
            .create_task_with_priority(2, "Medium Priority", 2)
            .unwrap();

        priority_manager
            .create_task_with_priority(3, "High Priority", 3)
            .unwrap();

        for _ in 0..3 {
            if let Some(task) = priority_manager.next_task() {
                writeln!(
                    console,
                    "[PRIORITY] Selected: {} | ID: {} | Priority: {}",
                    task.name,
                    task.id,
                    task.priority
                ).unwrap();
            }
        }

        writeln!(console, "[PRIORITY] Test completed").unwrap();

        // Hardware timer
        writeln!(console, "[TIMER] Initializing IDT").unwrap();
        interrupts::init_idt();

        writeln!(console, "[TIMER] Initializing PIC").unwrap();
        interrupts::init_pics();

        writeln!(console, "[TIMER] Configuring PIT at 100 Hz").unwrap();
        interrupts::init_pit(100);

        x86_64::instructions::interrupts::enable();

        writeln!(console, "[TIMER] Interrupts enabled").unwrap();

        let mut last_tick = 0;

        while last_tick < 5 {
            x86_64::instructions::hlt();

            let current_tick =
                interrupts::TICKS.load(
                    core::sync::atomic::Ordering::Relaxed
                );

            if current_tick > last_tick {
                last_tick = current_tick;

                writeln!(
                    console,
                    "[HARDWARE TIMER] Tick: {}",
                    last_tick
                ).unwrap();
            }
        }

        writeln!(console, "[TIMER] Test completed").unwrap();

        // Memory allocation test
        writeln!(console, "[MEMORY] Testing allocation").unwrap();

        if memory::test_allocation() {
            writeln!(
                console,
                "[MEMORY] 128 bytes allocated successfully"
            ).unwrap();
        } else {
            writeln!(console, "[MEMORY] Allocation failed").unwrap();
        }

        // Synchronization test
        writeln!(console, "[SYNC] Testing spinlock").unwrap();

        if sync::test_synchronization() {
            writeln!(console, "[SYNC] Spinlock test passed").unwrap();
        } else {
            writeln!(console, "[SYNC] Spinlock test failed").unwrap();
        }

        // Integration tests
        writeln!(console, "[TEST] Running integration tests").unwrap();

        let (passed, failed) = tests::run_tests();

        writeln!(console, "[TEST] Passed: {}", passed).unwrap();
        writeln!(console, "[TEST] Failed: {}", failed).unwrap();

        if failed == 0 {
            writeln!(console, "[TEST] All tests passed").unwrap();
        } else {
            writeln!(console, "[TEST] Some tests failed").unwrap();
        }

        writeln!(console, "[BOOT] System is running!").unwrap();
    }

    loop {
        x86_64::instructions::hlt();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
