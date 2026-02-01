#![no_main]
#![no_std]

use pwm_led as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Hello, world!");

    pwm_led::exit()
}
