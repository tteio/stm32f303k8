#![no_std]
#![no_main]

use core::cell::RefCell;
use cortex_m::asm;
use cortex_m::interrupt::Mutex;
use cortex_m::peripheral::NVIC;
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f3xx_hal::{gpio, 
                    gpio::PushPull,
                    interrupt,
                    pac::{self, TIM6},
                    prelude::*,
                    timer
                };

type LedPin = gpio::gpiob::PB3<gpio::Output<PushPull>>;
static LED: Mutex<RefCell<Option<LedPin>>> = 
    Mutex::new(RefCell::new(None));

type TimerT = timer::Timer<TIM6>;
static P_TIMER: Mutex<RefCell<Option<TimerT>>> = 
    Mutex::new(RefCell::new(None));

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
    let led = 
        gpiob
        .pb3
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);

    //Timer setting
    let mut period_timer = timer::Timer::tim6(dp.TIM6, 10.Hz(), clk, &mut rcc.apb1);
    period_timer.listen(timer::Event::Update);

    // Move the ownership of the led and period_timer to global
    cortex_m::interrupt::free(|cs| *LED.borrow(cs).borrow_mut() = Some(led));
    cortex_m::interrupt::free(|cs| *P_TIMER.borrow(cs).borrow_mut() = Some(period_timer));

    unsafe { NVIC::unmask(interrupt::TIM6_DACUNDER) };

    loop {
        asm::delay(2_000_000);
    }
}

#[interrupt]
fn TIM6_DACUNDER(){
    cortex_m::interrupt::free(|cs| {
        // Clear interrupt flag
        P_TIMER.borrow(cs)
        .borrow_mut()
        .as_mut()
        .unwrap()
        .clear_update_interrupt_flag();

        // Toggle the LED
        LED.borrow(cs)
            .borrow_mut()
            .as_mut()
            .unwrap()
            .toggle()
            .unwrap();
    })
}