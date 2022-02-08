use super::Range;
use crate::math::*;
use thiserror::Error;

/// A multi-dimensional array.
///
/// We take a page from FORTRAN and allow for negative indicies into multi-dimensional arrays. We
/// do this by storing a `Range<IV>` representing the `domain` of the array (rather than `UV` for
/// the `dimension`), because it allows for substantially simpler code when dealing with boundary
/// conditions. In particular, code that would otherwise have to include special cases to avoid
/// unsigned integer underflow or Index out of bounds errors can simply ignore this possibility as
/// long as the caller passes in an `ArrayNd` with a larger domain (with additional "ghost cells"
/// filled in).
#[derive(Default)]
pub struct ArrayNd<T> {
    data: Vec<T>,
    domain: Range<IV>,
    stride: IV,
    offset: isize,
}

// TODO: figure out a vaguely consistent error handling strategy
#[derive(Error, Debug)]
pub enum ArrayNdCreationError {
    #[error("The domain specified for the array {0} is invalid.")]
    InvalidDomain(Range<IV>, #[source] std::num::TryFromIntError),
}

impl<T> ArrayNd<T>
where
    T: num::Zero + Clone,
{
    /// Creates a new multi-dimensional array of zeros.
    ///
    /// Returns an error if the domain is invalid.
    pub fn zeros(domain: Range<IV>) -> Result<Self, ArrayNdCreationError> {
        let dim = domain.size();
        let size = dim
            .iter()
            .product::<isize>()
            .try_into()
            .map_err(|err| ArrayNdCreationError::InvalidDomain(domain, err))?;

        let stride = calculate_strides(dim);
        Ok(Self {
            data: vec![num::Zero::zero(); size],
            domain,
            stride,
            offset: -domain.min.dot(&stride),
        })
    }

    /// Creates a new multi-dimensional array of zeros with the same domain (and other associated
    /// constants) as another `ArrayNd`.
    pub fn zeros_like<U>(other: &ArrayNd<U>) -> Self {
        Self {
            data: vec![num::Zero::zero(); other.data.len()],
            domain: other.domain,
            stride: other.stride,
            offset: other.offset,
        }
    }
}

/// Calculates the strides for a particular domain size.
/// The strides are a set of numbers such that the dot product of an index with the stride gives
/// index into the flattened array.
///
/// You may be more familiar with the formula `idx = x + y * width` (in 2 dimensions). This is
/// essentially taking a dot product between the index `[x, y]` and the stride vector `[1, width]`.
/// this function generalizes that idea to multiple dimensions.
fn calculate_strides(cells: IV) -> IV {
    let mut strides = IV::zeros();
    strides[DIM - 1] = 1;
    for i in (1..=DIM - 1).rev() {
        strides[i - 1] = strides[i] * cells[i];
    }
    strides
}

impl<T> std::ops::Index<IV> for ArrayNd<T> {
    type Output = T;
    fn index(&self, idx: IV) -> &Self::Output {
        debug_assert!(self.domain.contains_half_open(idx));
        &self.data[idx.dot(&self.stride) as usize]
    }
}

impl<T> std::ops::IndexMut<IV> for ArrayNd<T> {
    fn index_mut(&mut self, idx: IV) -> &mut Self::Output {
        debug_assert!(self.domain.contains_half_open(idx));
        &mut self.data[idx.dot(&self.stride) as usize]
    }
}
