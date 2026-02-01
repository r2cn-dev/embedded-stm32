#![no_main]
#![no_std]

use usart_tx_rx as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Hello, world!");

    usart_tx_rx::exit()
}
