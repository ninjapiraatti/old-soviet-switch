#![no_std]
#![no_main]

use core::cell::Cell;

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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SwitchState {
    TopLeft,
    TopRight,
    Main,
    Neutral,
}

pub trait SwitchCallback<Message> {
    fn call(&mut self, msg: Message);
}

impl<Message, F> SwitchCallback<Message> for F
where
    F: FnMut(Message),
{
    fn call(&mut self, msg: Message) {
        self(msg)
    }
}

pub struct OldSovietSwitch<T1, T2, T3, T4, Callback>
where
    T1: InputPin,
    T2: InputPin,
    T3: InputPin,
    T4: OutputPin,
    Callback: SwitchCallback<SwitchState>,
{
    pub pin_top_left: T1,
    pub pin_top_right: T2,
    pub pin_main_switch: T3,
    pub pin_led: T4,
    pub switch_state: Mutex<Cell<SwitchState>>,
    pub callback: Callback,
}

impl <T1, T2, T3, T4, Callback> OldSovietSwitch<T1, T2, T3, T4, Callback>
    where
    T1: InputPin,
    T2: InputPin,
    T3: InputPin,
    T4: OutputPin,
    Callback: SwitchCallback<SwitchState>,
{
    pub fn new(
        pin_top_left: T1,
        pin_top_right: T2,
        pin_main_switch: T3,
        pin_led: T4,
        callback: Callback,
    ) -> Self {
        let mut instance = Self {
            pin_top_left,
            pin_top_right,
            pin_main_switch,
            pin_led,
            switch_state: Mutex::new(Cell::new(SwitchState::Neutral)),
            callback,
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
