// Embed kernel via build script

include!(concat!(env!("OUT_DIR"), "/kernel_info.rs"));

pub static KERNEL: PageAligned<[u8; KERNEL_SIZE]> = PageAligned(KERNEL_BYTES);

#[repr(align(4096))]
pub struct PageAligned<T>(T);

impl <T> PageAligned<T> {
    pub fn borrow(&self) -> &T {
        &self.0
    }
}
