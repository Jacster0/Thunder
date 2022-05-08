use lazy_static::lazy_static;
use spin::Mutex;
use core::fmt;
use crate::kernel::arch::x86::vga;

pub struct Writer {
    column_position: usize,
    color_code: vga::ColorCode,
    buffer: &'static mut vga::Buffer,
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: vga::ColorCode::new(vga::Color::Yellow, vga::Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut vga::Buffer) },
    });
}


impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= vga::BUFFER_WIDTH {
                    self.new_line();
                }

                let row = vga::BUFFER_HEIGHT -1;
                let col = self.column_position;
                let color_code = self.color_code;

                self.buffer.chars[row][col].write( vga::ScreenChar {
                    ascii_character: byte,
                    color_code
                });

                self.column_position += 1;
            }
        }
    }

    pub fn write_str(&mut self, str: &str) {
        for byte in str.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1.. vga::BUFFER_HEIGHT {
            for col in 0..vga::BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(vga::BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = vga::ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..vga::BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_str(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::print::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

