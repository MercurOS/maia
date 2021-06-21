use core::fmt;

use crate::uefi::EfiStatus;

#[repr(C)]
pub struct SimpleTextInputProtocol {
    reset: extern "C" fn (&SimpleTextInputProtocol, bool) -> EfiStatus,
    read_key_stroke: extern "C" fn (&SimpleTextInputProtocol, *mut InputKey) -> EfiStatus,
}

#[repr(C)]
pub struct SimpleTextOutputProtocol {
    reset: extern "C" fn(&SimpleTextOutputProtocol, bool) -> EfiStatus,
    output_string: extern "C" fn(&SimpleTextOutputProtocol, *const u16) -> EfiStatus,
}

#[repr(C)]
pub struct InputKey {
    pub scan_code: u16,
    pub unicode_char: u16,
}

impl SimpleTextInputProtocol {
    pub fn reset(&mut self, extended_verification: bool) -> EfiStatus {
        (self.reset)(self, extended_verification)
    }

    pub fn read_key_stroke(&mut self) -> Option<InputKey> {
        let mut key = InputKey {
            scan_code: 0,
            unicode_char: 0,
        };
        let key_ptr = &mut key as *mut InputKey;
        let status = (self.read_key_stroke)(self, key_ptr);

        if !status.is_error() {
            Some(key)
        } else {
            None
        }
    }
}

impl SimpleTextOutputProtocol {
    pub fn reset(&mut self, extended_verification: bool) -> EfiStatus {
        (self.reset)(self, extended_verification)
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\r' | b'\n' => self.write_character(byte as u16),
                _ => self.write_character(0xfffd)
            }
        }
    }

    fn write_character(&mut self, character: u16) {
        let buf = [character, 0u16];
        let buf_ptr = &buf[..].as_ptr();

        (self.output_string)(self, *buf_ptr);
    }
}

impl fmt::Write for SimpleTextOutputProtocol {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}
