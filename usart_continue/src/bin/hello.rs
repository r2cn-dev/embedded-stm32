#![no_main]
#![no_std]

use usart_continue as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Hello, world!");

    usart_continue::exit()
}
