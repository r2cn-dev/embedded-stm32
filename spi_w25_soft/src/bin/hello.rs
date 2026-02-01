#![no_main]
#![no_std]

use spi_w25_soft as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Hello, world!");

    spi_w25_soft::exit()
}
