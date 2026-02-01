#![deny(unsafe_code)]
#![no_std]
#![no_main]

// 出现故障死循环
use panic_halt as _;

// 处理非阻塞任务
use nb::block;

use cortex_m_rt::entry;

use stm32f1xx_hal::{pac, prelude::*, timer::Timer};

use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {

    rtt_init_print!();
    // Get access to the core peripherals from the cortex-m crate,SYST、NVIC
    let cp = cortex_m::Peripherals::take().unwrap();
    // Get access to the device specific peripherals from the peripheral access crate
    let dp = pac::Peripherals::take().unwrap();

    // 将RCC外设转换为HAL层的安全抽象类型，绑定到rcc上
    let mut rcc = dp.RCC.constrain();

    // Acquire the GPIOC peripheral,split() acquire pc0、、、pc15 to configure respectively.
    let mut gpioc = dp.GPIOC.split(&mut rcc);

    // Configure gpio C pin 13 as a push-pull output. The `crh` register is passed to the function
    // in order to configure the port. For pins 0-7, crl should be passed instead.
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    // Configure the syst timer to trigger an update every second
    let mut timer = Timer::syst(cp.SYST, &rcc.clocks).counter_hz();
    timer.start(2.Hz()).unwrap();

    // Wait for the timer to trigger an update and change the state of the LED
    loop {
        //  阻塞等待，直到定时周期结束
        block!(timer.wait()).unwrap();
        rprintln!("led on");
        led.set_high();
        block!(timer.wait()).unwrap();
        rprintln!("led off");
        led.set_low();
    }
}