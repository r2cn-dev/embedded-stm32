#![no_main]
#![no_std]

use dma_adc_loop as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Hello, world!");

    dma_adc_loop::exit()
}
