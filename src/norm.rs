// TODO: Replace with `RangeInclusive::<-1.0, 1.0>` when implemented
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Norm(f64);

impl Norm {
    #[must_use]
    pub fn new(value: f64) -> Option<Self> {
        match value >= -1.0 && value <= 1.0 {
            true => Some(Self(value)),
            false => None,
        }
    }
}
