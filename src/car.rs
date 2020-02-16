#[cfg(test)]
mod unit_tests;
use crate::{Motor, MotorSpeeds, Norm, Stm32f3xxPeripherals};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Car<P: Stm32f3xxPeripherals> {
    periphs: P,
    motor_speeds: MotorSpeeds,
}

impl<P: Stm32f3xxPeripherals> Car<P> {
    #[must_use]
    pub fn new(periphs: P) -> Self {
        let res = Self {
            periphs,
            motor_speeds: MotorSpeeds::default(),
        };
        // TODO: Evolve this into a compile-time check
        debug_assert!(res.motor_speeds.len() == Motor::VARIANT_COUNT);
        res
    }

    #[must_use]
    pub fn with_motor_speeds(periphs: P, motor_speeds: MotorSpeeds) -> Self {
        let res = Self {
            periphs,
            motor_speeds,
        };
        // TODO: Evolve this into a compile-time check
        debug_assert!(res.motor_speeds.len() == Motor::VARIANT_COUNT);
        res
    }

    // The length of the `Motors` array is defined at compile time to equal the number of
    // `Motor` enum variants, and so cannot panic.  Also `debug_assert`ed to ensure against an
    // contract-violating code change.
    // Compiler has enough information to elide array index bounds check (unverified whether it does
    // or not)
    #[allow(clippy::indexing_slicing)]
    #[must_use]
    pub fn motor_speed(&self, motor: Motor) -> Norm {
        self.motor_speeds[motor as usize]
    }

    #[must_use]
    pub fn motor_speeds(&self) -> [Norm; Motor::VARIANT_COUNT] {
        self.motor_speeds
    }

    // The length of the `Motors` array is defined at compile time to equal the number of
    // `Motor` enum variants, and so cannot panic.  Also `debug_assert`ed to ensure against an
    // contract-violating code change.
    // Compiler has enough information to elide array index bounds check (unverified whether it does
    // or not)
    #[allow(clippy::indexing_slicing)]
    pub fn set_motor_speed(&mut self, motor: Motor, value: Norm) -> &mut Self {
        debug_assert!(self.motor_speeds.len() == Motor::VARIANT_COUNT);
        self.motor_speeds[motor as usize] = value;
        self
    }

    // The length of the `Motors` array is defined at compile time to equal the number of
    // `Motor` enum variants, and so cannot panic.  Also `debug_assert`ed to ensure against an
    // contract-violating code change.
    // Compiler has enough information to elide array index bounds check (unverified whether it does
    // or not)
    #[allow(clippy::indexing_slicing)]
    pub fn set_motor_speeds(&mut self, motor_speeds: MotorSpeeds) -> &mut Self {
        self.motor_speeds = motor_speeds;
        self
    }
}
