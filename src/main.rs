#![no_std]
#![no_main]

use esp_idf_sys as idf;
use esp_idf_logger;
use log;

const LED : u32 = 2;

#[no_mangle]
pub fn app_main() {
    esp_idf_logger::init().unwrap();
    log::info!("Hello with logger");

    // log manually
    unsafe {
      let text = b"Hello World\n\0";
      idf::printf(text.as_ptr() as *const _);

      // enable LED
      idf::gpio_set_direction(LED, idf::GPIO_MODE_DEF_OUTPUT);
    }

    let mut led_on : bool = true;

    loop {
        enable_status_led(led_on);
        if led_on {
            led_on = false;
        } else {
            led_on = true;
        }
	light_sleep(1000000);
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

fn enable_status_led(enable : bool) {
  unsafe {
    if enable {
      idf::gpio_set_level(LED, 1);
    } else {
      idf::gpio_set_level(LED, 0);
    }
  }
}

fn light_sleep(duration_us : u64) {
        unsafe {
            // Set RTC timer to trigger wakeup and then enter light sleep
            // idf::esp_sleep_enable_timer_wakeup(25000);
            idf::gpio_hold_en(LED);
            idf::esp_sleep_enable_timer_wakeup(duration_us);
            idf::esp_light_sleep_start();
            idf::gpio_hold_dis(LED);
  log::info!("awoke because of {}", idf::esp_sleep_get_wakeup_cause());
        }
}
