#![no_main]
#![no_std]

use iic_mpu_soft as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Hello, world!");

    iic_mpu_soft::exit()
}
