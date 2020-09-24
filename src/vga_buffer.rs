// /Users/josiah/github/rustOS/src/vga_buffer.rs

use volatile::Volatile;
use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8)) // bitwise OR with two u4 (but
                                                                // Rust does not do u4 so "u8")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]  // give the struct the C standard representation, including fields
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {   // create an implementation for the Writer struct
    /// Writes an ASCII byte to the buffer
    ///
    /// Wraps lines at `BUFFER_WIDTH`. Supports the `/n` newline character.
    pub fn write_byte(&mut self, byte: u8) {    // create a fn to write a single byte to screen
        match byte {                    // match on byte
            b'\n' => self.new_line(),   // if byte is the new line byte, move to the next
                                        // line (CR also? - depends on implementation of new_line)

            byte => {                   // any other byte, do this
                if self.column_position >= BUFFER_WIDTH {   // go to new line if beyond width
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;    // set row position
                let col = self.column_position; // set column position

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {  // assign the char byte and color to
                                                            // the buffer at position row, col
                                                            // this is the 'print to screen' step
                    ascii_character: byte,
                    color_code: color_code,
                });
                self.column_position += 1;  // move the 'cursor' to the next position
            }
        }
    }

    fn new_line(&mut self) {    // new_line reads all characters successively and writes them
                                // to the line above in an orderly fashion. lastly, it erases the
                                // final line (which has been rewritten one line above) and
                                // prepares a new line for writing
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {   // "clears" a row by overwriting with spaces
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }


    pub fn write_string(&mut self, s: &str) {   // define a fn to write strings by writing them
                                                // byte by byte using the write_byte fn
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }

        }
    }
}

impl fmt::Write for Writer {    // from the fmt namespace, we implement "write" functionality
    fn write_str(&mut self, s: &str) -> fmt::Result {   // self is "Writer", so any self call
                                                        // refers to the instance of "Writer"
        self.write_string(s);
        Ok(())
    }
}

// pub static WRITER: Writer = Writer {
//     column_position: 0,
//     color_code: ColorCode::new(Color::Yellow, Color::Black),
//     buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
// };

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}

#[test_case]
fn test_println_simple() {
    println!("test_println_simple output");
}

#[test_case]
fn test_println_many() {
    for _ in 0..200 {
        println!("test_println_many output");
    }
}

#[test_case]
fn test_println_output() {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    let s = "Some test string that fits on a single line";
    interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();
        writeln!(writer, "\n{}", s).expect("writeln failed");
        for (i, c) in s.chars().enumerate() {
            let screen_char = writer.buffer.chars[BUFFER_HEIGHT - 2][i].read();
            assert_eq!(char::from(screen_char.ascii_character), c);
        }
    });
}
