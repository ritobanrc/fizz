/// Extension trait for nalgebra Vectors for types that implement `PartialOrd`
pub trait VecExtPartialOrd {
    /// Returns true if all of the components of `self` are less than the respective components of
    /// `other`.
    fn all_lt(&self, other: &Self) -> bool;

    /// Returns true if all of the components of `self` are greater than the respective components of
    /// `other`.
    fn all_gt(&self, other: &Self) -> bool;

    /// Returns true if all of the components of `self` are less than or equal to therespective components of `other`.
    fn all_le(&self, other: &Self) -> bool;

    /// Returns true if all of the components of `self` are greater than or equal to the respective components of
    /// `other`.
    fn all_ge(&self, other: &Self) -> bool;

    /// The componentwise max of two vectors.
    fn component_max(&self, other: &Self) -> Self;

    /// The componentwise min of two vectors.
    fn component_min(&self, other: &Self) -> Self;
}

impl<T: na::Scalar + PartialOrd, const D: usize> VecExtPartialOrd for na::SVector<T, D> {
    fn all_lt(&self, other: &Self) -> bool {
        self.zip_fold(other, true, |acc, a, b| acc && (a < b))
    }

    fn all_gt(&self, other: &Self) -> bool {
        self.zip_fold(other, true, |acc, a, b| acc && (a > b))
    }

    fn all_le(&self, other: &Self) -> bool {
        self.zip_fold(other, true, |acc, a, b| acc && (a <= b))
    }

    fn all_ge(&self, other: &Self) -> bool {
        self.zip_fold(other, true, |acc, a, b| acc && (a >= b))
    }

    fn component_max(&self, other: &Self) -> Self {
        self.zip_map(other, |a, b| {
            na::partial_max(&a, &b)
                .expect("Float compare failed")
                .clone()
        })
    }

    fn component_min(&self, other: &Self) -> Self {
        self.zip_map(other, |a, b| {
            na::partial_min(&a, &b)
                .expect("Float compare failed")
                .clone()
        })
    }
}

/// This trait exists because _certain_ libraries still use tuples for representing vectors, rather
/// than a special struct or even constant-length arrays.
pub trait IntoVec<T, const N: usize> {
    fn into_vec(self) -> na::SVector<T, N>;
}

impl<T> IntoVec<T, 2> for (T, T) {
    fn into_vec(self) -> na::Vector2<T> {
        na::Vector2::new(self.0, self.1)
    }
}

impl<T> IntoVec<T, 3> for (T, T, T) {
    fn into_vec(self) -> na::Vector3<T> {
        na::Vector3::new(self.0, self.1, self.2)
    }
}
