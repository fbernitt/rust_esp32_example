#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

use core::mem::ManuallyDrop;
use cstr_core::CStr;
use esp_idf_logger;
use esp_idf_sys as idf;
use log;

extern crate alloc;
use alloc::vec::Vec;

mod alloc_support;

const LED: u32 = 2;

#[no_mangle]
pub fn app_main() {
    esp_idf_logger::init().unwrap();

    dump_tasks();
    let mut vec = Vec::<u32>::new();
    for i in 1..100 {
        vec.push(i);
    }

    log::info!("Hello with logger");

    // log manually
    unsafe {
        let text = b"Hello World\n\0";
        idf::printf(text.as_ptr() as *const _);

        // enable LED
        idf::gpio_set_direction(LED, idf::GPIO_MODE_DEF_OUTPUT);
    }

    let mut led_on: bool = true;

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

fn enable_status_led(enable: bool) {
    unsafe {
        if enable {
            idf::gpio_set_level(LED, 1);
        } else {
            idf::gpio_set_level(LED, 0);
        }
    }
}

fn light_sleep(duration_us: u64) {
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

fn dump_tasks() {
    let mut tasks_buffer = ManuallyDrop::new(Vec::<idf::TaskStatus_t>::with_capacity(20));
    let mut runtime: u32 = 0;
    let cap = tasks_buffer.capacity();
    let buffer = tasks_buffer.as_mut_ptr();

    unsafe {
        let count = idf::uxTaskGetSystemState(
            buffer as *mut idf::TaskStatus_t,
            cap as u32,
            &mut runtime as *mut u32,
        ) as usize;
        let tasks = Vec::from_raw_parts(buffer, count, cap);
        for i in 0..count {
            let task = tasks[i];
            log::info!(
                "{}. task: {}: {:?} ({}), min stack: {}",
                i,
                task.xTaskNumber,
                CStr::from_ptr(task.pcTaskName).to_str(),
                task.ulRunTimeCounter,
                task.usStackHighWaterMark
            );
        }
        log::info!("total runtime ticks: {}", runtime);
    }
}
