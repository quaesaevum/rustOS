// /Users/josiah/rustOS/rust_os/src/main.rs

#![no_std]  // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)] // allows custom tests instead of using std which we disabled
#![test_runner(rust_os::test_runner)] // does something to bring in the test runner I think
#![reexport_test_harness_main = "test_main"]    // allows use of the test_main() function

// extern crate rlibc;

use core::panic::PanicInfo;
use rust_os::println;

// mod vga_buffer; // define a module for vga buffer - located in src/vga_buffer.rs
// mod serial;

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// #[repr(u32)]
// pub enum QemuExitCode {
//     Success = 0x10,
//     Failed = 0x11,
// }
//
// pub trait Testable {
//     fn run(&self) -> ();
// }
//
// impl<T> Testable for T
// where
//     T: Fn(),
// {
//     fn run(&self) {
//         serial_print!("{}...\t", core::any::type_name::<T>());
//         self();
//         serial_println!("[ok]");
//     }
// }

// pub fn exit_qemu(exit_code: QemuExitCode) {
//     use x86_64::instructions::port::Port;
//
//     unsafe {
//         let mut port = Port::new(0xf4);
//         port.write(exit_code as u32);
//     }
// }

#[no_mangle]    // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();    // invoke tests

    loop {}
}

/// panic handler in non-testing mode
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info)
}

// #[cfg(test)]
// pub fn test_runner(tests: &[&dyn Testable]) {   // runs tests for our custom setup
//     serial_println!("Running {} tests", tests.len());
//     for test in tests {     // appears to collect all tests labeled via #[test_case] then loops
//         test.run();         // through each
//     }
//     exit_qemu(QemuExitCode::Success);
// }
//
// #[test_case]                // identifies a test
// fn trivial_assertion() {    // this is a trivial test to check our functionality
//     assert_eq!(1, 1);
// }
