#![no_std]
#![no_main]

use esp32c3_hal::{ clock::ClockControl, gpio::Input, peripherals::Peripherals, prelude::*, Delay};
use esp_backtrace as _;
use esp_println::println;

pub struct OldSovietSwitch{
    pin_top_left: Input<u32>,
    pin_top_right: Input<u32>,
    pin_main_switch: Input<u32>,
    pin_led: Input<u32>,
}

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();

    let clocks = ClockControl::max(system.clock_control).freeze();
    let mut delay = Delay::new(&clocks);

    // setup logger
    // To change the log_level change the env section in .cargo/config.toml
    // or remove it and set ESP_LOGLEVEL manually before running cargo run
    // this requires a clean rebuild because of https://github.com/rust-lang/cargo/issues/10358
    esp_println::logger::init_logger_from_env();
    log::info!("Logger is setup");
    println!("Hello world!");
    loop {
        println!("Loop...");
        delay.delay_ms(500u32);
    }
}
