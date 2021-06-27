pub trait Console: super::UEFI {
    fn clear_screen(&mut self) {
        let console_out = unsafe { &mut *(self.borrow_system().console_out) };
        (console_out.clear_screen)(console_out);
    }

    fn write_string(&mut self, s: &str) {
        let console_out = unsafe { &mut *(self.borrow_system().console_out) };

        let mut buf = [0u16, 0u16];
        for byte in s.bytes() {
            buf[0] = match byte {
                0x20..=0x7e | b'\r' | b'\n' => byte as u16,
                _ => 0xfffd,
            };
            (console_out.output_string)(console_out, buf[..].as_ptr());
        }
    }
}

impl Console for super::Application {}
