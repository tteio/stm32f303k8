#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use stm32f3xx_hal::{pac, serial::Serial, nb, prelude::*};
use stm32f3xx_hal as hal;

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
    let button =
        gpiob
        .pb0
        .into_pull_up_input(&mut gpiob.moder, &mut gpiob.pupdr);

    //uart setting
    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);
    let uart_pins = (
        gpioa.pa2.into_af7_push_pull(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrl),  //tx
        gpioa.pa15.into_af7_push_pull(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrh), //rx
    );

    let mut debug = Serial::new(dp.USART2, uart_pins, 9600.Bd(), clk, &mut rcc.apb1);
    debug.bwrite_all(b"Start sample.\r\n").unwrap();

    loop {
        if button.is_low().unwrap() {
            led.set_high().unwrap();
        }
        else{
            led.set_low().unwrap();
        }

        match debug.read() {
            Ok(s) => nb::block!(debug.write(s)).unwrap(),
            Err(error) => {
                match error {
                    nb::Error::Other(e) => {
                        match e {
                            hal::serial::Error::Framing => debug.bwrite_all(b"Framing error.\r\n").unwrap(),
                            hal::serial::Error::Noise => debug.bwrite_all(b"Noise error.\r\n").unwrap(),
                            hal::serial::Error::Overrun => debug.bwrite_all(b"Overrun error.\r\n").unwrap(),
                            hal::serial::Error::Parity => debug.bwrite_all(b"Parity error.\r\n").unwrap(),
                            _ => {},
                        };
                    },
                    nb::Error::WouldBlock => {},
                };
            },
        };

    }
}