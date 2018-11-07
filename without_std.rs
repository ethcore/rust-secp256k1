extern "C" {
    fn ext_malloc(size: usize) -> *mut u8;
    fn ext_free(ptr: *mut u8);
    fn ext_print_utf8(utf8_data: *const u8, utf8_len: u32);
    fn ext_print_num(value: u64);
}

/// Wasm allocator
pub struct WasmAllocator;

#[global_allocator]
static ALLOCATOR: WasmAllocator = WasmAllocator;

mod __impl {
    use core::alloc::{GlobalAlloc, Layout};

    use super::WasmAllocator;

    unsafe impl GlobalAlloc for WasmAllocator {
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            super::ext_malloc(layout.size()) as *mut u8
        }

        unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
            super::ext_free(ptr as *mut u8)
        }
    }
}

#[doc(hidden)]
#[panic_handler]
#[no_mangle]
pub fn panic(info: &::core::panic::PanicInfo) -> ! {
    unsafe {
        if let Some(loc) = info.location() {
            ext_print_utf8(loc.file().as_ptr() as *const u8, loc.file().len() as u32);
            ext_print_num(loc.line() as u64);
            ext_print_num(loc.column() as u64);
        }
        core::intrinsics::abort()
    }
}

#[doc(hidden)]
#[alloc_error_handler]
pub extern fn oom(_: ::core::alloc::Layout) -> ! {
    static OOM_MSG: &str = "Runtime memory exhausted. Aborting";

    unsafe {
        ext_print_utf8(OOM_MSG.as_ptr(), OOM_MSG.len() as u32);
        core::intrinsics::abort()
    }
}