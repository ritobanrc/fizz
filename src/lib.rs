extern crate nalgebra as na;

pub mod math {
    #[cfg(feature = "3d")]
    pub const DIM: usize = 3;

    #[cfg(feature = "2d")]
    pub const DIM: usize = 2;

    pub type Dim = na::Const<DIM>;

    pub type T = f64;
    pub type TV = na::SVector<T, DIM>;
    pub type IV = na::SVector<isize, DIM>;
    pub type UV = na::SVector<usize, DIM>;

    pub type Mat = na::SMatrix<T, DIM, DIM>;
}

pub mod base;
pub mod sph;
pub mod util;
