use crate::Norm;
use variant_count::VariantCount;

#[derive(VariantCount)]
#[repr(usize)]
pub enum Motor {
    FrontLeft = 0,
    FrontRight,
    BackLeft,
    BackRight,
}

pub type Speeds = [Norm; Motor::VARIANT_COUNT];
