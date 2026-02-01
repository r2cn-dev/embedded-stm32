#![no_main]
#![no_std]

use spi_w25_hard as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Hello, world!");

    spi_w25_hard::exit()
}
