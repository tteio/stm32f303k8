#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use stm32f3xx_hal::{pac, prelude::*, serial::Serial, watchdog};

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

    //uart setting
    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);
    let uart_pins = (
        gpioa.pa2.into_af7_push_pull(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrl),  //tx
        gpioa.pa15.into_af7_push_pull(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrh), //rx
    );
    let mut debug = Serial::new(
        dp.USART2, 
        uart_pins, 
        9600.Bd(), 
        clk, 
        &mut rcc.apb1
    );
    debug.bwrite_all(b"Start sample.\r\n").unwrap();

    //watchdog setting
    let mut iwdg = watchdog::IndependentWatchDog::new(dp.IWDG);
    iwdg.stop_on_debug(&dp.DBGMCU, true);
    iwdg.start(3000.milliseconds());

    loop {
        match debug.read() {
            Ok(_) => iwdg.feed(),//clear wdt
            Err(_) => {},
        };
    }
}
