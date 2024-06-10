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

/*
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
*/

pub struct OldSovietSwitch<T1, T2, T3>
where
    T1: InputPin,
    T2: InputPin,
    T3: InputPin,
{
    pub pin_top_left: T1,
    pub pin_top_right: T2,
    pub pin_main_switch: T3,
}

impl <T1, T2, T3> OldSovietSwitch<T1, T2, T3>
    where
    T1: InputPin,
    T2: InputPin,
    T3: InputPin,
{
    pub fn new(
        pin_top_left: T1,
        pin_top_right: T2,
        pin_main_switch: T3,
    ) -> Self {
        let mut instance = Self {
            pin_top_left,
            pin_top_right,
            pin_main_switch,
        };

        instance.setup();
        instance
    }

    pub fn setup(&mut self) {
        self.pin_top_left.listen(Event::FallingEdge);
        self.pin_top_right.listen(Event::FallingEdge);
        self.pin_main_switch.listen(Event::FallingEdge);
        interrupt::enable(peripherals::Interrupt::GPIO, interrupt::Priority::Priority3).unwrap();
    }
    pub fn read_state(&mut self) -> (bool, bool, bool) {
        self.pin_top_left.clear_interrupt();
        self.pin_top_right.clear_interrupt();
        self.pin_main_switch.clear_interrupt();
        (
            self.pin_top_left.is_input_high(),
            self.pin_top_right.is_input_high(),
            self.pin_main_switch.is_input_high(),
        )
    }
}
