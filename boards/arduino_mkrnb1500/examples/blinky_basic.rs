#![no_std] // NickP: we are on an embedded dev environment, so we can't assume we have std libs available
#![no_main] // NickP: side effect of MCU
#![forbid(unsafe_code)] // Guaranteed 100% safe Rust :)

// use core::sync::atomic::AtomicU64;

use arduino_mkrnb1500 as bsp;
use bsp::hal;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _; // NickP: When code panics on MCU, this instructs it to HALT
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use bsp::entry;
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;

// static PUCKET_COUNTER: AtomicU64 = AtomicU64::new(0);

#[entry] // NickP: side effect of using MCU - this is where it starts
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap(); // This line controls access to peripherals so multiple instances cannot exist (ie only one thing can control a pin at once)
    let core = CorePeripherals::take().unwrap(); // Similar ^ but something to do with interrupts?

    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.gclk,
        &mut peripherals.pm,
        &mut peripherals.sysctrl,
        &mut peripherals.nvmctrl,
    ); // Inits clocks of system
    let pins = bsp::Pins::new(peripherals.port); // Creates an alias for bsp::Pins
    let mut led = pins.d6.into_push_pull_output(); // Creates LED pin like pinMode led = OUTPUT;
    let mut delay = Delay::new(core.SYST, &mut clocks); // Creates a new delay instance out of the system timer

    loop {
        delay.delay_ms(500u32);
        led.set_high().unwrap();
        delay.delay_ms(200u32);
        led.set_low().unwrap();
    }
}
