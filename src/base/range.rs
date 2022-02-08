use super::vec_ext::*;

/// A Range type (i.e. something which has a minimum and a maximum).
/// This is commonly used as `Range<TV>` and `Range<IV>` to represent a box, though can be used with
/// other numeric types as well.
///
/// Note that, unlike the rest of the library, we do not use the 2d and 3d feature flags to allow
/// `Range<T>` to be dimension independent. This is because `Range` can be generic over different
/// types (not just f32 and f64), and potentially even works for types that aren't vectors.
///
/// Finally, note that we also do not use the standard library's [`std::ops::Range`] because it
/// would provide little benefit other than the `..` syntax, since none of the methods would carry
/// over, while creating additional hassle with the orphan rules (requiring us to create extension
/// traits).
#[derive(Debug, Default, Copy, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct Range<T> {
    pub min: T,
    pub max: T,
}

impl<T> Range<T> {
    /// Creates a new `Range` given a `min` and `max` value.
    pub fn new(min: T, max: T) -> Self {
        Self { min, max }
    }
}

impl<T: na::Scalar + na::ClosedSub, const D: usize> Range<na::SVector<T, D>> {
    /// Calculates the size of a `Range` as a vector.
    pub fn size(&self) -> na::SVector<T, D> {
        &self.max - &self.min
    }
}

impl<T: na::Scalar + PartialOrd, const D: usize> Range<na::SVector<T, D>> {
    /// Whether `a` is within the `Range` (returns `true` at the endpoints).
    pub fn contains(&self, a: na::SVector<T, D>) -> bool {
        self.min.all_le(&a) && self.max.all_ge(&a)
    }

    /// Whether `a` is within the `Range`, but treating it as a half-open interval
    /// (i.e. returns true if the components are equal to the `min` but not the `max`).
    pub fn contains_half_open(&self, a: na::SVector<T, D>) -> bool {
        self.min.all_le(&a) && self.max.all_gt(&a)
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Range<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}..{}", self.min, self.max)
    }
}
