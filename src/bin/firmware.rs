#![no_std]
#![no_main]

use panic_halt as _;
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use nb::block;
use cortex_m_rt::entry;
use stm32f1xx_hal as hal;
use hal::{pac, prelude::*, timer::Timer, gpio::Output};
use fugit;

const LED_FREQ: fugit::HertzU32 = fugit::HertzU32::kHz(1000);

fn setup(
    dp: pac::Peripherals,
    cp: cortex_m::Peripherals,
) -> (hal::timer::SysCounterHz, hal::gpio::Pin<'C', 13, Output>) {
    let rcc = dp.RCC.constrain();
    let mut flash = dp.FLASH.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut gpioc = dp.GPIOC.split();
    let led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    let timer = Timer::syst(cp.SYST, &clocks).counter_hz();
    return (timer, led);
}

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();
    let (mut timer, mut led) = setup(dp, cp);
    timer.start(LED_FREQ).unwrap();

    loop {
        block!(timer.wait()).unwrap();
        led.set_high();
        block!(timer.wait()).unwrap();
        led.set_low();
    }
}
