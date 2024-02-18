#![no_std]
#![no_main]

use esp32c3_hal::{
    clock::ClockControl,
    gpio::{GpioPin, Input, InputPin, Output, OutputPin, PullDown, PullUp, PushPull},
    peripherals::Peripherals,
    prelude::*,
    Delay,
};
use esp_backtrace as _;
use esp_println::println;

pub struct OldSovietSwitch {
    pub pin_top_left: GpioPin<Input<PullDown>, 4>,
    pub pin_top_right: GpioPin<Input<PullDown>, 5>,
    pub pin_main_switch: GpioPin<Input<PullDown>, 6>,
    pub pin_led: GpioPin<Output<PushPull>, 7>,
}

impl OldSovietSwitch {
    pub fn new(
        pin_top_left: GpioPin<Input<PullDown>, 4>,
        pin_top_right: GpioPin<Input<PullDown>, 5>,
        pin_main_switch: GpioPin<Input<PullDown>, 6>,
        pin_led: GpioPin<Output<PushPull>, 7>,
    ) -> Self {
        Self {
            pin_top_left,
            pin_top_right,
            pin_main_switch,
            pin_led,
        }
    }
}
