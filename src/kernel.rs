// Embed kernel via build script

include!(concat!(env!("OUT_DIR"), "/kernel_info.rs"));

static KERNEL: PageAligned<[u8; KERNEL_SIZE]> = PageAligned(KERNEL_BYTES);

#[repr(align(4096))]
struct PageAligned<T>(T);
