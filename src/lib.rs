#![no_std]
#![no_main]

use esp32c3_hal::{
    gpio::{
        Event, InputPin,
    },
    interrupt,
    peripherals::{self},
};
use esp_backtrace as _;

pub struct OldSovietSwitch<T1, T2, T3>
where
    T1: InputPin,
    T2: InputPin,
    T3: InputPin,
{
    pub pin1_main: T1,
    pub pin2_bottom_left: T2,
    pub pin3_bottom_right: T3,
}

impl <T1, T2, T3> OldSovietSwitch<T1, T2, T3>
    where
    T1: InputPin,
    T2: InputPin,
    T3: InputPin,
{
    pub fn new(
        pin1_main: T1,
        pin2_bottom_left: T2,
        pin3_bottom_right: T3,
    ) -> Self {
        let mut instance = Self {
            pin1_main,
            pin2_bottom_left,
            pin3_bottom_right,
        };

        instance.setup();
        instance
    }

    pub fn setup(&mut self) {
        self.pin1_main.listen(Event::FallingEdge);
        self.pin2_bottom_left.listen(Event::FallingEdge);
        self.pin3_bottom_right.listen(Event::FallingEdge);
        interrupt::enable(peripherals::Interrupt::GPIO, interrupt::Priority::Priority3).unwrap();
    }
    pub fn read_state(&mut self) -> (bool, bool, bool) {
        self.pin1_main.clear_interrupt();
        self.pin2_bottom_left.clear_interrupt();
        self.pin3_bottom_right.clear_interrupt();
        (
            self.pin1_main.is_input_high(),
            self.pin2_bottom_left.is_input_high(),
            self.pin3_bottom_right.is_input_high(),
        )
    }
}
