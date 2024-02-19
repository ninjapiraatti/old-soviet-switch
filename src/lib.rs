#![no_std]
#![no_main]

use critical_section::Mutex;
use esp32c3_hal::{
    clock::ClockControl,
    gpio::{
        Event, Gpio4, Gpio5, Gpio6, Gpio7, GpioPin, Input, Output, InputPin, OutputPin, PullDown, PullUp, PushPull, IO,
    },
    interrupt,
    peripheral::Peripheral,
    peripherals::{self, Peripherals},
    prelude::*,
    Delay,
};
use esp_backtrace as _;
use esp_println::println;

pub struct OldSovietSwitch<T1, T2, T3, T4>
where
    T1: InputPin,
    T2: InputPin,
    T3: InputPin,
    T4: OutputPin,
{
    pub pin_top_left: T1,
    pub pin_top_right: T2,
    pub pin_main_switch: T3,
    pub pin_led: T4,
}

impl <T1, T2, T3, T4> OldSovietSwitch<T1, T2, T3, T4>
    where
    T1: InputPin,
    T2: InputPin,
    T3: InputPin,
    T4: OutputPin,
{
    pub fn new(
        pin_top_left: T1,
        pin_top_right: T2,
        pin_main_switch: T3,
        pin_led: T4,
    ) -> Self {
        let mut instance = Self {
            pin_top_left,
            pin_top_right,
            pin_main_switch,
            pin_led,
        };

        instance.setup_interrupts();
        instance
    }

    pub fn setup_interrupts(&mut self) {
        self.pin_top_left.listen(Event::FallingEdge);
        self.pin_top_right.listen(Event::FallingEdge);
        self.pin_main_switch.listen(Event::FallingEdge);
        interrupt::enable(peripherals::Interrupt::GPIO, interrupt::Priority::Priority3).unwrap();
    
    }
}

#[interrupt]
fn GPIO() {
    critical_section::with(|cs| {
        println!("GPIO interrupt");
    });
}
/*
impl InterruptHandler for OldSovietSwitch {
    fn handle_interrupt(&self) {
        // Your logic to handle interrupt
        // For example, toggle LED when the main switch is pressed
        if self.pin_main_switch.is_high().unwrap() {
            self.pin_led.set_high().unwrap();
        } else {
            self.pin_led.set_low().unwrap();
        }
    }
}
*/
