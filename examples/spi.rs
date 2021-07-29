#![no_std]
#![no_main]

use core::convert::TryInto;
use cortex_m::asm;
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f3xx_hal::{pac, prelude::*, spi};

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
    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);
    let mut ss = gpioa
        .pa4
        .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);
    ss.set_high().unwrap();//ss is low active

    let sck = gpioa
        .pa5
        .into_af5_push_pull(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrl);
    let miso = gpioa
        .pa6
        .into_af5_push_pull(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrl);
    let mosi = gpioa
        .pa7
        .into_af5_push_pull(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrl);
    
    //Spi setting
    let spi_mode = spi::Mode{
        polarity: spi::Polarity::IdleLow,
        phase: spi::Phase::CaptureOnFirstTransition,
    };

    let mut spi = spi::Spi::spi1(
        dp.SPI1,
        (sck, miso, mosi),
        spi_mode,
        4u32.MHz().try_into().unwrap(),
        clk,
        &mut rcc.apb2,
    );

    //init mcp23s17
    let init_data: [u8;4] = [0x40, 0x00, 0x01, 0x00];//ADD000, A0=input, A1=output
    ss.set_low().unwrap();
    spi.write(&init_data).unwrap();
    ss.set_high().unwrap();


    loop {
        asm::delay(2_000_000);
        let mut read_data = [0x41, 0x12, 0x00, 0x00];
        ss.set_low().unwrap();
        let read_buf = spi.transfer(&mut read_data).unwrap();
        ss.set_high().unwrap();
    
        let mut write_data = [0x40, 0x14, 0x00, 0x00];
        if read_buf[2] & 0x01 == 0x01 {
            write_data[2] = 0x02;
        }
        ss.set_low().unwrap();
        spi.write(&write_data).unwrap();
        ss.set_high().unwrap();
    }
}
