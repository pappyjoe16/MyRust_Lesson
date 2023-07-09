#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, Delay, DelayMs, LedArray, OutputSwitch};

#[entry]
fn main() -> ! {
    let (mut _delay, mut leds): (Delay, LedArray) = aux5::init();
    loop {
        leds[0].on().ok();
    }
}
