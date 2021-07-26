#![no_std]
#![no_main]

use cortex_m::peripheral;
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f3xx_hal::{pac, delay, prelude::*};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    //CLK setting
    let clk = rcc.cfgr
        //.use_hse(8.MHz())     //Use external oscillator
        //.bypass_hse()         //Use external clock signal
        //.enable_css()         //Enable CSS (Clock Security System)
        .hclk(64.MHz())
        .sysclk(64.MHz())
        .pclk1(32.MHz())
        .pclk2(64.MHz())
        .freeze(&mut flash.acr);    //flash access wait setting

    //GPIO setting
    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);
    let mut led = 
        gpiob
        .pb3
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);

    //delay setting
    //let mut core = cortex_m::Peripherals::take().unwrap();
    let core = peripheral::Peripherals::take().unwrap();
    let systick = core.SYST;
    let mut blocking = delay::Delay::new(systick, clk);

    loop {
        led.toggle().unwrap();
        blocking.delay_ms(500u32);
    }
}