#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Unit(f64);
impl Eq for Unit {}
impl Ord for Unit {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.total_cmp(&other.0)
    }
}
impl From<f64> for Unit {
    fn from(value: f64) -> Self {
        assert!(
            (0.0..=1.0).contains(&value),
            "Unit value must be in [0, 1] and not NaN, got {value}"
        );
        Self(value + 0.0)
    }
}
impl From<Unit> for f64 {
    fn from(value: Unit) -> Self {
        value.0
    }
}
impl From<f32> for Unit {
    fn from(value: f32) -> Self {
        Self::from(f64::from(value))
    }
}
impl From<Unit> for f32 {
    fn from(value: Unit) -> Self {
        value.0 as f32
    }
}
