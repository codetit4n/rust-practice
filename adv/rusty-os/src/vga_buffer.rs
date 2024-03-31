use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

use crate::serial_println;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)] // repr(u8) tells the compiler to represent the enum as an u8
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
#[repr(transparent)] // repr(transparent) tells the compiler to ensure that the
                     // struct has the exact same data layout as u8
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)] // repr(C) tells the compiler to represent the struct as if it were a C struct
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)] // to ensure it has the same memory layout as its single field
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

// writng to screen
pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte), // anything between space and ~
                // not part of printable ASCII range
                _ => self.write_byte(0xfe), // print ■
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };

        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank)
        }
    }
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

// write! macro support
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

// print! and println! macros support

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)] // to hide the function from the generated documentation
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
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
    let s = "Some test string that fits on a single line";
    println!("{}", s);
    for (i, c) in s.chars().enumerate() {
        let screen_char = WRITER.lock().buffer.chars[BUFFER_HEIGHT - 2][i].read();
        assert_eq!(char::from(screen_char.ascii_character), c);
    }
}

#[test_case]
fn test_println_long_line_creates_no_panic() {
    for _ in 0..10 {
        println!(
            "A short story: Ivan’s family is healthy, hard-working and prosperous and they live happily. The balance is upset by a feud with their neighbor, Gabriel. A hen belonging to Ivan’s daughter-in-law flew into Gabriel’s yard and laid an egg. When she inquires about it, Gabriel’s mother respond rudely. It quickly escalates into name calling and a shouting match. Legal proceeding  follow. Ivan’s father, who used to run the farm, advises his family to reconcile, and not let this disagreement over a trifle get out of hand. His words go unheeded, and quarreling becomes a daily occurrence."
        );
    }
}

#[test_case]
fn test_println_long_line_wraps_correctly() {
    let s = "A short story Ivans family is healthy hardworking and prosperous and they live happily The balance is upset"; // removing all special characters
    let s_line_1_expected =
        "A short story Ivans family is healthy hardworking and prosperous and they live h"; // MAX_WIDTH = 80
    println!("{}", s);
    for (i, c) in s_line_1_expected.chars().enumerate() {
        let screen_char = WRITER.lock().buffer.chars[(BUFFER_HEIGHT - 3)][i].read(); // line split into 2 lines so -3
        assert_eq!(char::from(screen_char.ascii_character), c);
    }
    let s_line_2_expected = "appily The balance is upset";
    for (i, c) in s_line_2_expected.chars().enumerate() {
        let screen_char = WRITER.lock().buffer.chars[(BUFFER_HEIGHT - 2)][i].read(); // line split into 2 lines so -2
        assert_eq!(char::from(screen_char.ascii_character), c);
    }
}

#[test_case]
fn test_println_correctly_prints_non_printable_chars() {
    println!("☺");
    // Note: ☺ is a character which takes 3 bytes to store
    let screen_char_1 = WRITER.lock().buffer.chars[BUFFER_HEIGHT - 2][0].read();
    let screen_char_2 = WRITER.lock().buffer.chars[BUFFER_HEIGHT - 2][1].read();
    let screen_char_3 = WRITER.lock().buffer.chars[BUFFER_HEIGHT - 2][2].read();
    let screen_char_4 = WRITER.lock().buffer.chars[BUFFER_HEIGHT - 2][3].read();
    assert_eq!(screen_char_1.ascii_character, u8::from(0xfe)); // 0xfe is the unicode for ■
    assert_eq!(screen_char_2.ascii_character, u8::from(0xfe));
    assert_eq!(screen_char_3.ascii_character, u8::from(0xfe));
    assert_ne!(screen_char_4.ascii_character, u8::from(0xfe));
    assert_eq!(screen_char_4.ascii_character, u8::from(0x20)); // 0x20 is the unicode for space
}
