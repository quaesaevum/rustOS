// /Users/josiah/github/rustOS/src/serial.rs

use uart_16550::SerialPort;     // SerialPort implements fmt::Write trait
use spin::Mutex;
use lazy_static::lazy_static;

lazy_static! {  // "use lazy_static and a spinlock to create a static writer instance"
    pub static ref SERIAL1: Mutex<SerialPort> = {   // Mutex 'mutual exclusion' locks out a thread
                                                    // at least, I think it does
        let mut serial_port = unsafe { SerialPort::new(0x3F8) };    // port 1016/0b0011 1111 1000
        serial_port.init();
        Mutex::new(serial_port)
    };
}

#[doc(hidden)]  // prevent this from entering docs
pub fn _print(args: ::core::fmt::Arguments) {   // core print function
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        SERIAL1
            .lock()
            .write_fmt(args)
            .expect("Printing to serial failed");
    });
}

/// Prints to the host through the serial interface
#[macro_export]
macro_rules! serial_print { // define a macro for printing a single line
    ($($arg:tt)*) => {
        $crate::serial::_print(format_args!($($arg)*));
    };
}

/// Prints to the host through the serial interface, appending a newline.
#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(
        concat!($fmt, "\n"), $($arg)*));
}
