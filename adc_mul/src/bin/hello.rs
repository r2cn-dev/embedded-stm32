#![no_main]
#![no_std]

use adc_mul as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Hello, world!");

    adc_mul::exit()
}
