#![no_main]
#![no_std]

use iic_oled as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Hello, world!");

    iic_oled::exit()
}
