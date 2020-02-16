//#![no_std]
//#![no_main]
//#![warn(clippy::all, clippy::nursery, clippy::pedantic, rust_2018_idioms)]
//#![forbid(bare_trait_objects)]
//#![allow(clippy::match_bool)]
//// To use the `unsafe` keyword, change to `#![allow(unsafe_code)]` (do not remove); aids auditing.
//#![forbid(unsafe_code)]
//// Safety-critical application lints
//#![deny(
//    clippy::pedantic,
//    clippy::float_cmp_const,
//    clippy::indexing_slicing,
//    clippy::integer_arithmetic,
//    clippy::option_unwrap_used,
//    clippy::result_unwrap_used
//)]
//
//// Uncomment before ship to reconcile use of possibly redundant crates, debug remnants, missing license files and more
////#![warn(clippy::cargo, clippy::restriction, missing_docs, warnings)]
////#![deny(warnings)]
//
//use discovery_car::{Car, Motor, MotorSpeeds, Norm};
//use stm32f3_discovery::stm32f3xx_hal::stm32;
//
//#[entry]
//fn main() -> ! {
//    let periphss = stm32::Peripherals;
//    let car = Car::new(periphss);
//}
