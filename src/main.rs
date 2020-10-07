// /Users/josiah/rustOS/rust_os/src/main.rs

#![no_std]  // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)] // allows custom tests instead of using std which we disabled
#![test_runner(rust_os::test_runner)] // does something to bring in the test runner I think
#![reexport_test_harness_main = "test_main"]    // allows use of the test_main() function

// extern crate rlibc;

use core::panic::PanicInfo;
use rust_os::println;
use rust_os::print;
use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);

// #[no_mangle]    // don't mangle the name of this function. it must be _start because that is the
                // default entry point for most systems. it must not be mangled
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use rust_os::memory::active_level_4_table;
    use rust_os::memory::translate_addr;
    use x86_64::VirtAddr;

    println!("Hello World{}", "!");
    rust_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

    let addresses = [
     // the identity-mapped vga buffer page
     0xb8000,
     // some code page
     0x201008,
     // some stack page
     0x0100_0020_1a10,
     // virtual address mapped to physical address 0
     boot_info.physical_memory_offset,
    ];

    for &address in &addresses {
     let virt = VirtAddr::new(address);
     let phys = unsafe { translate_addr(virt, phys_mem_offset) };
     println!("{:?} -> {:?}", virt, phys);
    }

    #[cfg(test)]
    test_main();    // invoke tests

    println!("It did not crash!");
    rust_os::hlt_loop();
}

/// panic handler in non-testing mode
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    rust_os::hlt_loop();
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
