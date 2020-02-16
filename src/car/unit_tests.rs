#![allow(non_snake_case, clippy::option_unwrap_used)]
mod new;
mod set_motor_speed;
mod set_motor_speeds;
mod stm32_mocks;
mod with_motor_speeds;

use super::*;
use stm32_mocks::Peripherals as MockPeripherals;
