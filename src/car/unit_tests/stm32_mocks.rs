use crate::Stm32f3xxPeripherals;

#[derive(Debug)]
pub struct Peripherals {}

impl Peripherals {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

impl Stm32f3xxPeripherals for Peripherals {}
