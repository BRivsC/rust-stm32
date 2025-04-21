#![no_std]
#![no_main]
/// Ejemplo 1: Encender LED al pulsar el botón en la placa
/// Bastián Rivas
/// Este ejemplo fue hecho pensando en una placa STM32 NUCLEO-F411RE
/// 
/// Compilar: cargo build --target thumbv7em-none-eabihf
/// Flashear: cargo embed



// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

//use cortex_m::asm;
//use cortex_m_rt::entry;

use cortex_m_rt as rt;
use stm32f4xx_hal as hal;
use hal::prelude::*; // needed for the GpioExt trait (-> .split)

//#[entry]
#[rt::entry]
fn main() -> ! {
    //asm::nop(); // To not have main optimize to abort in release mode, remove when you add code
    // El siguiente código fue tomado de https://github.com/krenzlin/rust-stm32f446-blinky/blob/master/src/main.rs
    if let Some(peripherals) = hal::stm32::Peripherals::take() {
        let gpioa = peripherals.GPIOA.split(); // + sets RCC->AHB1ENR GPIOA bit

        // .into_push_pull_output performs three steps
        // 1) set PUPDR: 00 -> no pull-up, no pull-down
        // 2) set OTYPER: 0 -> output push-pull
        // 3) set MODER: 01 -> general purpose output mode
        let mut led = gpioa.pa5.into_push_pull_output();

        let gpioc = peripherals.GPIOC.split();
        let button = gpioc.pc13; // pins are input by default

        loop {
            // .is_high reads IDR
            if button.is_high() {
                // .set_low uses BSRR
                led.set_low();
            } else {
                led.set_high();
            }
        }
    }

    loop {
        // your code goes here
    }
}
