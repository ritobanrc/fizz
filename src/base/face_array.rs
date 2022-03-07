use super::ArrayNd;
use crate::math::*;

/// Represents data stored at faces of grid.
///
/// It is very common to use "Marker-and-cell" (MAC) or Staggered Grids in numerical simulation.
/// In these grids, some data is stored at the faces of cells (rather than at cell centers). For
/// example, velocity is commonly stored on a MAC grid, where x-faces store the x-component,
/// y-faces store the y-component, and z-faces store the z-component. See [Harlow and Welch 1965]
/// for the original paper which introduced the idea of MAC grids for incompressible flow
/// simulation.
///
/// This allows for second-order finite difference approximations of the derivatives to be
/// evaluated at cell centers, which can be used to update the quantities stored there, and vice
/// versa (for example, the divergence of velocity can be calculated in each cell, which can be
/// used to solve for the pressures, stored at cell centers. The gradient of those pressures, in
/// turn, can be used to update the velocity components located at cell faces). A MAC grid
/// discretization also avoids checkerboarding in the solution (where using a central-difference
/// stencil causes every-other cell to be updated).
pub struct FaceArray<T>(pub [ArrayNd<T>; DIM]);

impl<T> std::ops::Index<FaceIndex> for FaceArray<T> {
    type Output = T;
    fn index(&self, fi: FaceIndex) -> &Self::Output {
        &self.0[fi.axis][fi.cell]
    }
}

impl<T> std::ops::IndexMut<FaceIndex> for FaceArray<T> {
    fn index_mut(&mut self, fi: FaceIndex) -> &mut Self::Output {
        &mut self.0[fi.axis][fi.cell]
    }
}

/// An index used to index into a `FaceArray`. Contains a `cell` and an associated `axis`. The index
/// represeted is the lower/left face of that cell.
pub struct FaceIndex {
    pub cell: IV,
    pub axis: usize,
}

impl FaceIndex {
    pub fn new(cell: IV, axis: usize) -> Self {
        Self { cell, axis }
    }
}
