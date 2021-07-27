#![no_std]
#![no_main]

use cortex_m::asm;
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f3xx_hal::{adc, i2c, pac, prelude::*, serial::Serial};

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

    //I2C setting
    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);
    let i2c_pins = (
        gpiob.pb6.into_af4_open_drain(
            &mut gpiob.moder,
            &mut gpiob.otyper,
            &mut gpiob.afrl
        ),
        gpiob.pb7.into_af4_open_drain(
            &mut gpiob.moder,
            &mut gpiob.otyper,
            &mut gpiob.afrl
        )
    );
    let mut i2c = i2c::I2c::new(
        dp.I2C1,
        i2c_pins,
        100_000.Hz(),
        clk,
        &mut rcc.apb1,
    );

    //DAC MCP4726 controll 
    let i2c_buf: [u8; 2] = [0x02, 0x00];
    let mut i2c_read_buf: [u8; 6] = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05];
    i2c.write(0x60, &i2c_buf).unwrap();
    i2c.read(0x60, &mut i2c_read_buf).unwrap();
    /*
    match i2c.write_read(0x60, &i2c_buf, &mut i2c_read_buf){
        Ok(()) => {},
        Err(e) =>{
            match e {
                i2c::Error::Arbitration =>
                    debug.bwrite_all(b"Arbitration error.\r\n").unwrap(),
                i2c::Error::Bus =>
                    debug.bwrite_all(b"Bus error.\r\n").unwrap(),
                i2c::Error::Busy =>
                    debug.bwrite_all(b"Busy error.\r\n").unwrap(),
                i2c::Error::Nack =>
                    debug.bwrite_all(b"Nack error.\r\n").unwrap(),
                _ => {}
            }
        }
    }
    */

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