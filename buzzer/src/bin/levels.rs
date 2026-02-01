#![no_main]
#![no_std]

use buzzer as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    // try setting the DEFMT_LOG environment variable
    // e.g. `export DEFMT_LOG=info` or `DEFMT_LOG=trace cargo rb levels`
    // 日志级别：trace < debug < info < warn < error
    defmt::info!("info");   
    defmt::trace!("trace");
    defmt::warn!("warn");
    defmt::debug!("debug");
    defmt::error!("error");

    buzzer::exit()
}
