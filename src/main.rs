#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::{bind_interrupts, interrupt};
use embassy_stm32::gpio::{Level, Output, Speed, Pull};
use embassy_stm32::exti::{self, ExtiInput};

use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(
    pub struct Irqs {
        EXTI15_10 => exti::InterruptHandler<interrupt::typelevel::EXTI15_10>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    let mut button = ExtiInput::new(p.PC13, p.EXTI13, Pull::Down, Irqs);

    let mut led = Output::new(p.PA5, Level::High, Speed::Low);

    loop {
        button.wait_for_rising_edge().await;
        info!("Pressed!");
        led.toggle();
    }
}