#![no_std]
#![no_main]

use cortex_m::asm;
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f3xx_hal::{pac, serial::Serial, adc, prelude::*};

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

    //ADC setting
    let mut adc1_2 = dp.ADC1_2;
    let mut adc1 = adc::Adc::adc1(
        dp.ADC1, 
        &mut adc1_2, 
        &mut rcc.ahb,
        adc::CkMode::default(),
        clk
    );
    let mut vr = gpioa.pa0.into_analog(&mut gpioa.moder, &mut gpioa.pupdr);
    
    let mut send_buf: [u8; 8] = ['0' as u8,'x' as u8,0,0,0,0,'\r' as u8,'\n' as u8];
    loop {
        let vr_data: u16 = adc1.read(&mut vr).unwrap();

        to_str( vr_data, &mut send_buf[2..6]);
        debug.bwrite_all(&send_buf).unwrap();
        asm::delay(2_000_000);
    }
}

fn to_str(mut num: u16, str:&mut [u8]) {
    for i in 0..4 {
        let nbuf:u8 = (num & 0x000f) as u8;
        if nbuf >= 10 {
            str[3-i] = 0x41 - 10 + nbuf;
        }
        else{
            str[3-i] = 0x30 + nbuf;
        }
        num = num >> 4;
    }
}