// /Users/josiah/github/rustOS/src/lib.rs

#![no_std]                      // we keep std out so that we do not need it
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;

extern crate rlibc;

pub mod serial;
pub mod vga_buffer;
pub mod interrupts;
pub mod gdt;

pub trait Testable {
    fn run(&self) -> ();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,         // 16
    Failed = 0x11,          // 17
}

pub fn exit_qemu(exit_code: QemuExitCode) {     // define a public function named exit_qemu that
                                                // params exit_code that is a QemuExitCode, a
                                                // public enum
    use x86_64::instructions::port::Port;       // from x86_64 namespace, etc, use Port

    unsafe {                                    // define an unsafe block of code
        let mut port = Port::new(0xf4);         // define a mutable variable name port that is-a
                                                // new Port on 0xf4 = 244
        port.write(exit_code as u32);           // to that port, write exit_code having converted
                                                // it to an unsigned 32-bit integer
    }
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    hlt_loop();
}

pub fn init() {
    gdt::init();
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

/// Entry point for `cargo test`
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();
    test_main();
    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}
