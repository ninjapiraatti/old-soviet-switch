#![no_std]
#![no_main]

use core::cell::RefCell;
use critical_section::Mutex;
use esp32c3_hal::{
    clock::ClockControl,
    gpio::{Gpio4, Gpio5, Gpio6, Input, PullDown, IO},
    interrupt,
    peripheral::Peripheral,
    peripherals::{self, Peripherals},
    prelude::*,
    Delay,
};
use esp_backtrace as _;
use esp_println::println;
use old_soviet_switch::*;

static OLD_SOVIET_SWITCH: Mutex<RefCell<Option<OldSovietSwitch<
Gpio6<Input<PullDown>>,
Gpio5<Input<PullDown>>,
Gpio4<Input<PullDown>
>>>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let main_switch = io.pins.gpio6.into_pull_down_input();
    let bottom_left = io.pins.gpio5.into_pull_down_input();
    let bottom_right = io.pins.gpio4.into_pull_down_input();

    let clocks = ClockControl::max(system.clock_control).freeze();
    let mut delay = Delay::new(&clocks);
    let ivan = OldSovietSwitch::new(
        main_switch,
        bottom_left,
        bottom_right,
    );
    critical_section::with(|cs| OLD_SOVIET_SWITCH.borrow_ref_mut(cs).replace(ivan));

    esp_println::logger::init_logger_from_env();
    log::info!("Logger is setup");
    println!("Hello world!");
    loop {
        delay.delay_ms(500u32);
        println!("LOOP");
    }
}

#[interrupt]

fn GPIO() {
    critical_section::with(|cs| {
        let switch_states = OLD_SOVIET_SWITCH.borrow_ref_mut(cs).as_mut().unwrap().read_state();
        println!("1 High: {:?}", switch_states.0);
        println!("2 High: {:?}", switch_states.1);
        println!("3 High: {:?}", switch_states.2);
    });
}
