#![no_std]
#![no_main]

use cortex_m::{asm, singleton};
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f3xx_hal::{pac, serial::Serial, prelude::*};

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

    let debug = Serial::new(dp.USART2, uart_pins, 9600.Bd(), clk, &mut rcc.apb1);


    //dma setting
    let (tx, rx) = debug.split();

    let dma1 = dp.DMA1.split(&mut rcc.ahb);
    let rx_dma = dma1.ch6;
    let tx_dma = dma1.ch7;

    //dma buf
    let rx_buf = singleton!(: [u8; 1] = [0; 1]).unwrap();
    let tx_buf = singleton!(: [u8; 1] = [0; 1]).unwrap();

    //wait receive
    let debug_receive = rx.read_exact( rx_buf, rx_dma);
    let (rx_buf,rx_dma, rx) = debug_receive.wait();

    //send receive data
    tx_buf[0] = rx_buf[0]; 
    let debug_send = tx.write_all(tx_buf, tx_dma);
    let (tx_buf,tx_dma, tx) = debug_send.wait();

    loop {
        asm::delay(2_000_000);
    }
}