#![no_main]
#![no_std]

use wdg_win as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Hello, world!");

    wdg_win::exit()
}
