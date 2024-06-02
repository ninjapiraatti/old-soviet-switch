#![no_std]
#![no_main]

use core::{borrow::Borrow, cell::{Cell, RefCell}};
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
    let mut ivan = OldSovietSwitch::new(
        top_left,
        top_right,
        main_switch,
        led,
        |switch_state| {
            // Handle the switch state change here
            println!("Switch state changed to: {:?}", switch_state);
            // Add your custom logic here
        },
    );
    critical_section::with(|cs| TOP_LEFT.borrow_ref_mut(cs).replace(ivan.pin_top_left));
    critical_section::with(|cs| TOP_RIGHT.borrow_ref_mut(cs).replace(ivan.pin_top_right));
    critical_section::with(|cs| MAIN_SWITCH.borrow_ref_mut(cs).replace(ivan.pin_main_switch));
    critical_section::with(|cs| LED.borrow_ref_mut(cs).replace(ivan.pin_led));

    esp_println::logger::init_logger_from_env();
    log::info!("Logger is setup");
    println!("Hello world!");
    loop {
        //println!("Testpin listening: {:?} | High: {:?}", ivan.pin_main_switch.is_listening(), ivan.pin_main_switch.is_high());
        delay.delay_ms(500u32);
        println!("LOOP");
    }
}


#[interrupt]

fn GPIO() {
    critical_section::with(|cs| {
        let mut top_left = TOP_LEFT.borrow(cs).borrow_mut();
        let mut top_right = TOP_RIGHT.borrow(cs).borrow_mut();
        let mut main_switch = MAIN_SWITCH.borrow(cs).borrow_mut();

        /*
        if let Some(top_left_pin) = TOP_LEFT.borrow(cs).borrow().as_ref() {
            // If `is_high` and similar methods require `&mut self`, you'll need
            // to ensure your design accommodates mutable access safely, possibly
            // rethinking shared access patterns.
            let is_top_left_high = top_left_pin.is_high().unwrap_or(false);
            println!("top left: {:?}", is_top_left_high);
            // Use `is_top_left_high` as needed here
        }
        // Check if all pins are available (i.e., not None)
        if let (Some(top_left_pin), Some(top_right_pin), Some(main_switch_pin), Some(led_pin)) = (top_left, top_right, main_switch, led) {
            // Access the pins directly without taking them out.
            // Example: Read pin state, clear interrupt, etc.
            // top_left_pin.is_high()...
            // led_pin.set_high()...

            let new_state = match (
                top_left_pin.is_high().unwrap(),
                top_right_pin.is_high().unwrap(),
                main_switch_pin.is_high().unwrap(),
            ) {
                (true, false, false) => SwitchState::TopLeft,
                (false, true, false) => SwitchState::TopRight,
                (false, false, true) => SwitchState::Main,
                _ => SwitchState::Neutral,
            };

            println!("new_state: {:?}", new_state);
            //switch.pin_top_left.clear_interrupt();
            //switch.pin_top_right.clear_interrupt();
            //switch.pin_main_switch.clear_interrupt();
            // Note: Since we are not moving the pins, we don't need to replace them.

        } else {
            // Handle the case where one or more GPIO pins are unexpectedly None.
            // This could log an error or simply return.
            println!("Well shit");
        }
        */
        //println!("Top left: {:?}", top_left.as_mut());
        match top_left.as_mut() {
            Some(pin) => {
                if pin.is_high().unwrap_or(false) {
                    println!("Top left is high");
                } else {
                    println!("Top left is low");
                }
                pin.clear_interrupt();
            },
            None => println!("No top left pin"),
        }
        match top_right.as_mut() {
            Some(pin) => {
                if pin.is_high().unwrap_or(false) {
                    println!("Top right is high");
                } else {
                    println!("Top right is low");
                }
                pin.clear_interrupt();
            },
            None => println!("No top right pin"),
        }
        match main_switch.as_mut() {
            Some(pin) => {
                if pin.is_high().unwrap_or(false) {
                    println!("Main is high");
                } else {
                    println!("Main is low");
                }
                pin.clear_interrupt();
            },
            None => println!("No main"),
        }
        // if let Some(top_left_pin) = top_left.as_mut().unwrap() {
        //     let is_top_left_high = top_left_pin.is_high().unwrap_or(false);
        //     println!("GPIO interrupt");
        //     top_left.as_mut().unwrap().clear_interrupt();
        // }
        //top_right.as_mut().unwrap().clear_interrupt();
        //main_switch.as_mut().unwrap().clear_interrupt();
        //led.as_mut().unwrap().clear_interrupt();
    });
}
/*
fn GPIO() {
    critical_section::with(|cs| {
        if let (Some(top_left), Some(top_right), Some(main_switch), Some(led)) = (
            TOP_LEFT.borrow(cs).borrow_mut().take(),
            TOP_RIGHT.borrow(cs).borrow_mut().take(),
            MAIN_SWITCH.borrow(cs).borrow_mut().take(),
            LED.borrow(cs).borrow_mut().take(),
        ) {
            let mut switch = OldSovietSwitch::new(
                top_left,
                top_right,
                main_switch,
                led,
                |_| {}, // Provide a dummy callback for now
            );

            let current_state = switch.switch_state.borrow(cs).get();
            let new_state = match (
                switch.pin_top_left.is_high().unwrap(),
                switch.pin_top_right.is_high().unwrap(),
                switch.pin_main_switch.is_high().unwrap(),
            ) {
                (true, false, false) => SwitchState::TopLeft,
                (false, true, false) => SwitchState::TopRight,
                (false, false, true) => SwitchState::Main,
                _ => SwitchState::Neutral,
            };

            println!("Current state: {:?}", current_state);
            if new_state != current_state {
                //switch.switch_state.lock().set(new_state);
                switch.switch_state.get_mut().set(new_state);
                println!("Switch state changed: {:?}", new_state);

                // Handle LED state based on the new switch state
                match new_state {
                    SwitchState::Main => switch.pin_led.set_high().unwrap(),
                    _ => switch.pin_led.set_low().unwrap(),
                }

                // Call the callback with the new switch state
                switch.callback.call(new_state);
            }

            // Remember to replace the taken values if necessary
            // Example:
            // *TOP_LEFT.borrow(cs).borrow_mut() = Some(top_left);
            // Similarly for others...
            switch.pin_top_left.clear_interrupt();
            switch.pin_top_right.clear_interrupt();
            switch.pin_main_switch.clear_interrupt();
        } else {
            // Handle the case where one or more GPIO pins are not available
            // Could be logging or an early return
            println!("Well shit");
        }
        // Reset the interrupts
    });
}
*/
