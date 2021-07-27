#![no_std]
#![no_main]

use cortex_m::asm;
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f3xx_hal::{pac, prelude::*, pwm};

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
    let pwm_out = 
        gpiob
        .pb3
        .into_af1_push_pull(&mut gpiob.moder, &mut gpiob.otyper, &mut gpiob.afrl);

    //Timer setting
    let tim2_channels = pwm::tim2(
        dp.TIM2, 
        16_000, //arr reg value
        10.Hz(), 
        &clk
    );
    let mut tim2_ch2 = tim2_channels.1.output_to_pb3(pwm_out);
    tim2_ch2.set_duty(tim2_ch2.get_max_duty() / 5); //20%
    tim2_ch2.enable();

    loop {
        asm::delay(2_000_000);
    }
}