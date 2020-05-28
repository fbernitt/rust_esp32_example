extern crate alloc;

extern crate esp_idf_alloc;
use core::alloc::Layout;

extern "C" {
    fn abort() -> !;
}

#[global_allocator]
static A: esp_idf_alloc::EspIdfAllocator = esp_idf_alloc::EspIdfAllocator;

#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    unsafe {
        abort();
    }
}
