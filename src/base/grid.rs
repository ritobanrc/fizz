use super::{FaceIndex, RangeIterator};
use crate::base::Range;
use crate::math::*;

/// Represents a Grid (Co-located or Staggered) in world space.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct Grid {
    /// Represents the world-space domain of the grid.
    pub domain: Range<TV>,
    /// The number of cells in each direction.
    /// Should always by domain.size() / dx, componentwise.
    pub cells: IV,
    /// Size of each grid cell
    /// Not serialized, since it can be calculated from `domain` and `cells`
    #[serde(skip)]
    pub dx: TV,
    /// Reciprocal of each component of `dx`
    /// Also not serialized, since it can be easily calculated from `dx`.
    #[serde(skip)]
    pub one_over_dx: TV,
}

impl Grid {
    /// Creates a new grid given the number of cells in each direction and the size of the domain.
    pub fn new(cells: IV, domain: Range<TV>) -> Self {
        let dx = domain.size().component_div(&na::convert::<_, TV>(cells));
        let one_over_dx = TV::from_element(1.).component_div(&dx);
        Self {
            domain,
            dx,
            one_over_dx,
            cells,
        }
    }

    /// Recalculates dx and one_over_dx using `domain` and `cells`.
    pub fn recalculate_dx(&mut self) {
        self.dx = self
            .domain
            .size()
            .component_div(&na::convert::<_, TV>(self.cells));
        self.one_over_dx = TV::from_element(1.).component_div(&self.dx);
    }

    /// Returns the number of nodes in each direction
    /// This is one greater than the number of cells.
    pub fn num_nodes(&self) -> IV {
        self.cells + IV::from_element(1)
    }

    /// Returns the number of nodes in each direction
    /// This is one greater than the number of cells.
    pub fn num_cells(&self) -> IV {
        self.cells
    }

    /// Returns the volume (in 3d) or area (in 2d) of a cell
    pub fn cell_size(&self) -> T {
        self.dx.iter().product()
    }

    /// Returns the area (in 3d) or length (in 2d) of each face
    pub fn face_areas(&self) -> TV {
        self.cell_size() * self.one_over_dx
    }

    /// Returns the area (in 3d) or length (in 2d) of faces in a particular direction.
    pub fn face_area(&self, axis: usize) -> T {
        self.cell_size() * self.one_over_dx[axis]
    }

    /// Location of the center of the cell.
    pub fn cell_center(&self, idx: IV) -> TV {
        self.domain.min
            + (na::convert::<_, TV>(idx) + TV::from_element(0.5)).component_mul(&self.dx)
    }

    /// Index of the nearest cell center
    pub fn cell_index(&self, x: TV) -> IV {
        na::try_convert::<_, IV>(
            (x - self.domain.min)
                .component_mul(&self.one_over_dx)
                .map(T::floor),
        )
        .expect(&format!("Failed to get cell index for {:?}", x))
    }

    /// Index of the node to the lower-left
    pub fn node_lower(&self, x: TV) -> IV {
        self.cell_index(x)
    }

    /// Position of a particular node
    pub fn node_x(&self, node: IV) -> TV {
        self.domain.min + node.cast::<T>().component_mul(&self.dx)
    }

    /// Get the location for a particular cell center
    pub fn cell_x(&self, cell: IV) -> TV {
        self.domain.min + (cell.cast::<T>() + TV::from_element(0.5)).component_mul(&self.dx)
    }

    /// Returns an iterator over all of the nodes in the grid.
    pub fn nodes<'a>(&self) -> impl Iterator<Item = IV> + 'a {
        RangeIterator::new(Range::new(IV::zeros(), self.num_nodes()))
    }

    /// Returns an iterator over all of the cells in the grid.
    pub fn cells<'a>(&self) -> impl Iterator<Item = IV> + 'a {
        RangeIterator::new(Range::new(IV::zeros(), self.num_cells()))
    }

    /// Returns an iterator over all of the faces in the grid.
    ///
    /// Iterates over the faces for each dimension, one at a time (i.e. x-faces, then y-faces, then
    /// z-faces).
    pub fn faces<'a>(&'a self) -> impl Iterator<Item = FaceIndex> + 'a {
        (0..DIM).flat_map(move |axis| {
            let start = IV::zeros();
            let end = self.num_cells() + IV::ith_axis(axis).into_inner();
            RangeIterator::new(Range::new(start, end)).map(move |cell| FaceIndex { cell, axis })
        })
    }
}
