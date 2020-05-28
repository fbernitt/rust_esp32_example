use esp_idf_sys as idf;

extern "C" {
    fn nvs_flash_init();
}

pub fn start_wifi() {
    unsafe {
        nvs_flash_init();
    }
}
