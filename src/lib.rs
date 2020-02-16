#![no_std]
#![warn(clippy::all, clippy::nursery, clippy::pedantic, rust_2018_idioms)]
#![forbid(bare_trait_objects)]
#![allow(clippy::match_bool)]
// To use the `unsafe` keyword, change to `#![allow(unsafe_code)]` (do not remove); aids auditing.
#![forbid(unsafe_code)]
// Safety-critical application lints
#![deny(
    clippy::pedantic,
    clippy::float_cmp_const,
    clippy::indexing_slicing,
    clippy::integer_arithmetic,
    clippy::option_unwrap_used,
    clippy::result_unwrap_used
)]

// Uncomment before ship to reconcile use of possibly redundant crates, debug remnants, missing license files and more
//#![warn(clippy::cargo, clippy::restriction, missing_docs, warnings)]
//#![deny(warnings)]

mod car;
mod consts;
mod error;
mod motor;
mod norm;
mod stm32f3xx_ext;

pub use car::Car;
pub use error::Error;
pub use motor::{Motor, Speeds as MotorSpeeds};
pub use norm::Norm;
#[cfg(target = "thumbv7em-none-eabi")]
use stm32f3_discovery::stm32f3xx_hal::stm32;
pub use stm32f3xx_ext::Stm32f3xxPeripherals;

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[cfg(target = "thumbv7em-none-eabi")]
impl Stm32f3xxPeripherals for stm32::Peripherals {}
