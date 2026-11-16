use core::fmt;
use core::fmt::Write as _;
use font8x8::legacy::BASIC_LEGACY;
use bootloader_api::info::{FrameBufferInfo, PixelFormat};
use uart_16550::SerialPort;

pub struct Screen<'a> {
    buffer: &'a mut [u8],
    info: FrameBufferInfo,
    x: usize,
    y: usize,
}

impl<'a> Screen<'a> {
    pub fn new(buffer: &'a mut [u8], info: FrameBufferInfo) -> Self {
        let mut screen = Self {
            buffer,
            info,
            x: 8,
            y: 8,
        };

        screen.clear();
        screen
    }

    fn clear(&mut self) {
        for y in 0..self.info.height {
            for x in 0..self.info.width {
                self.pixel(x, y, 8, 12, 24);
            }
        }
    }

    fn pixel(&mut self, x: usize, y: usize, r: u8, g: u8, b: u8) {
        if x >= self.info.width || y >= self.info.height {
            return;
        }

        let bpp = self.info.bytes_per_pixel;
        let index = (y * self.info.stride + x) * bpp;

        if index + bpp > self.buffer.len() {
            return;
        }

        match self.info.pixel_format {
            PixelFormat::Rgb => {
                if bpp >= 3 {
                    self.buffer[index] = r;
                    self.buffer[index + 1] = g;
                    self.buffer[index + 2] = b;
                }
            }
            PixelFormat::Bgr => {
                if bpp >= 3 {
                    self.buffer[index] = b;
                    self.buffer[index + 1] = g;
                    self.buffer[index + 2] = r;
                }
            }
            PixelFormat::U8 => {
                self.buffer[index] =
                    ((r as u16 + g as u16 + b as u16) / 3) as u8;
            }
            PixelFormat::Unknown {
                red_position,
                green_position,
                blue_position,
            } => {
                if bpp >= 4 {
                    let value =
                        ((r as u32) << red_position)
                        | ((g as u32) << green_position)
                        | ((b as u32) << blue_position);

                    self.buffer[index..index + 4]
                        .copy_from_slice(&value.to_ne_bytes());
                }
            }
            _ => {}
        }
    }

    fn draw_char(&mut self, ch: char) {
        if ch == '\n' {
            self.x = 8;
            self.y += 12;
            self.new_line_if_needed();
            return;
        }

        if ch == '\r' {
            self.x = 8;
            return;
        }

let code = ch as usize;
let glyph = if code < 128 {
    &BASIC_LEGACY[code]
} else {
    &BASIC_LEGACY[b'?' as usize]
};

for (row, bits) in glyph.iter().enumerate() {
    for col in 0..8 {
        if bits & (1 << col) != 0 {
            self.pixel(
                self.x + col,
                self.y + row,
                220,
                235,
                255,
            );
        }
    }
}
        self.x += 8;

        if self.x + 8 >= self.info.width {
            self.x = 8;
            self.y += 12;
            self.new_line_if_needed();
        }
    }

    fn new_line_if_needed(&mut self) {
        if self.y + 8 >= self.info.height {
            self.clear();
            self.x = 8;
            self.y = 8;
        }
    }
}

pub struct Console<'a> {
    serial: SerialPort,
    screen: Option<Screen<'a>>,
}

impl<'a> Console<'a> {
    pub fn new(
        serial: SerialPort,
        screen: Option<Screen<'a>>,
    ) -> Self {
        Self { serial, screen }
    }
}

impl fmt::Write for Console<'_> {
    fn write_str(&mut self, text: &str) -> fmt::Result {
        let _ = self.serial.write_str(text);

        if let Some(screen) = self.screen.as_mut() {
            for ch in text.chars() {
                screen.draw_char(ch);
            }
        }

        Ok(())
    }
}
