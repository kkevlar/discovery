//! Initialization code

#![no_std]

#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust#53964
extern crate panic_itm; // panic handler

pub use cortex_m::asm::bkpt;
pub use cortex_m_rt::entry;
pub use f3::hal::stm32f30x::{gpioa, gpioc, rcc};

use f3::hal::stm32f30x::{self, GPIOA, GPIOE, RCC};

pub fn init() -> (&'static gpioa::RegisterBlock, &'static gpioc::RegisterBlock, &'static rcc::RegisterBlock) {
    // restrict access to the other peripherals
    (stm32f30x::Peripherals::take().unwrap());

    unsafe { (&*GPIOA::ptr(), &*GPIOE::ptr(), &*RCC::ptr()) }
}
