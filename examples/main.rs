#![no_std]
#![no_main]

use core::cell::{Cell, RefCell};
use critical_section::Mutex;
use esp32c3_hal::{
    clock::ClockControl,
    gpio::{Event, Gpio4, Gpio5, Gpio6, Gpio7, Input, Output, PullDown, PullUp, PushPull, IO},
    interrupt,
    peripheral::Peripheral,
    peripherals::{self, Peripherals},
    prelude::*,
    Delay,
};
use esp_backtrace as _;
use esp_println::println;
use old_soviet_switch::*;

/*
use esp32c3_hal::{
    clock::ClockControl,
    esp_riscv_rt::entry,
    gpio::{Event, Gpio0, Input, PullUp},
    // gpio_types::{Event, Input, Pin, PullDown},
    interrupt,
    // pac::{Interrupt, Peripherals},
    prelude::*,
    soc::peripherals::{Interrupt, Peripherals},
    timer::TimerGroup,
    Delay,
    Rtc,
    IO,
};
*/

static TOP_LEFT: Mutex<RefCell<Option<Gpio4<Input<PullDown>>>>> = Mutex::new(RefCell::new(None));
static TOP_RIGHT: Mutex<RefCell<Option<Gpio5<Input<PullDown>>>>> = Mutex::new(RefCell::new(None));
static MAIN_SWITCH: Mutex<RefCell<Option<Gpio6<Input<PullDown>>>>> = Mutex::new(RefCell::new(None));
static LED: Mutex<RefCell<Option<Gpio7<Output<PushPull>>>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut top_left = io.pins.gpio4.into_pull_down_input();
    let mut top_right = io.pins.gpio5.into_pull_down_input();
    let mut main_switch = io.pins.gpio6.into_pull_down_input();
    let mut led = io.pins.gpio7.into_push_pull_output();

    let clocks = ClockControl::max(system.clock_control).freeze();
    let mut delay = Delay::new(&clocks);
    let mut ivan = OldSovietSwitch::new(top_left, top_right, main_switch, led);
    
    esp_println::logger::init_logger_from_env();
    log::info!("Logger is setup");
    println!("Hello world!");
    loop {
        println!("Testpin listening: {:?} | High: {:?}", ivan.pin_main_switch.is_listening(), ivan.pin_main_switch.is_high());
        delay.delay_ms(500u32);
    }
}
