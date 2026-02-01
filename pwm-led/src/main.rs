#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

use defmt::println;
use defmt_rtt as _;
use panic_probe as _;

use cortex_m_rt::entry;
use stm32f1xx_hal::pac;
use stm32f1xx_hal::prelude::_fugit_ExtU32;
use stm32f1xx_hal::prelude::_fugit_RateExtU32;
use stm32f1xx_hal::prelude::_stm32_hal_afio_AfioExt;
use stm32f1xx_hal::prelude::_stm32_hal_flash_FlashExt;
use stm32f1xx_hal::prelude::_stm32_hal_gpio_GpioExt;
use stm32f1xx_hal::rcc::RccExt;
use stm32f1xx_hal::timer::SysTimerExt;
use stm32f1xx_hal::timer::{Channel, PwmExt, Tim2NoRemap};


#[entry]
fn main() -> ! {
    // 获取对外设的访问对象
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let syst = cp.SYST;
    let mut afio = dp.AFIO.constrain();
    let tim2 = dp.TIM2;
    let mut gpioa = dp.GPIOA.split();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    // 具有自定义精度的阻塞延迟函数
    let mut delay = syst.delay(&clocks);
    // TIM2
    // 复用推挽输出
    let c1 = gpioa.pa0.into_alternate_push_pull(&mut gpioa.crl);
    let pins = c1;
    println!("load pwm...");
    let mut pwm = tim2.pwm_hz
    ::<Tim2NoRemap, _, _>(pins, &mut afio.mapr, 1.kHz(), &clocks);
    
    // Enable clock on each of the channels
    pwm.enable(Channel::C1);

    // Return to the original frequency
    pwm.set_period(100.kHz());

    loop {
        // 给tim2定时器的通道1设置占空比为i，延时
        for i in 0..=100 {
            pwm.set_duty(Channel::C1, i);
            delay.delay(10.millis());
        }
        for i in 0..=100 {
            pwm.set_duty(Channel::C1, 100 - i);
            delay.delay(10.millis());
        }
    }
}