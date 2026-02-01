#![no_main]
#![no_std]

use buzzer_of_light_sensor as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Hello, world!");

    buzzer_of_light_sensor::exit()
}
