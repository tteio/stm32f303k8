#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use stm32f3xx_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.constrain();

    //GPIO setting
    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);
    let mut led = 
        gpiob
        .pb3
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);
    let button =
        gpiob
        .pb0
        .into_pull_up_input(&mut gpiob.moder, &mut gpiob.pupdr);


    loop {
        if button.is_low().unwrap() {
            led.set_high().unwrap();
        }
        else{
            led.set_low().unwrap();
        }
    }
}