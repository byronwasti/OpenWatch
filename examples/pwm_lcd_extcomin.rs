#![feature(used)]
#![no_std]

extern crate cortex_m;
extern crate panic_abort;
extern crate stm32f30x_hal as hal;

use hal::prelude::*;
use hal::stm32f30x;
use hal::delay::Delay;

fn main() {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32f30x::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);

    /*
    let mut extcomin = gpiob
        .pb1
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);
    */
    // PB1 is AF6 and TIM1_ch3N or
    //        AF2 and TIM3_ch4

    rcc.apb1.enr().modify(|_, w| w.tim3en().enabled()); // Enable tim1
    dp.TIM3.cr1.modify(|_, w| w.cen().set_bit()); // Enable counter

    loop {
    }
}

