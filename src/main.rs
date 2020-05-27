#![no_std]
#![no_main]

use esp_idf_sys as idf;
use esp_idf_logger;
use log;

#[no_mangle]
pub fn app_main() {
    esp_idf_logger::init().unwrap();
    log::info!("Hello with logger");

    // log manually
    unsafe {
      let text = b"Hello World\n\0";
      idf::printf(text.as_ptr() as *const _);
    }
}

extern "C" {
    fn abort() -> !;
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe {
        abort();
    }
}
