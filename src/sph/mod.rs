//! This module contains an implementation of the Smoothed Particle Hydrodynamics (SPH) Algorithm.
//! For reference, we recommend readers look at
//!
//! * Müller, M., Charypar, D., & Gross, M. (2003, July). Particle-based fluid simulation for interactive applications. In Proceedings of the 2003 ACM SIGGRAPH/Eurographics symposium on Computer animation (pp. 154-159).
//! * Koschier, D., Bender, J., Solenthaler, B., & Teschner, M. (2020). Smoothed particle hydrodynamics techniques for the physics based simulation of fluids and solids. arXiv preprint arXiv:2009.06944.
//!
//! The former is the 2003 paper which first introduced SPH to the computer graphics community for
//! the simulation of fluids, while the latter is a recent tutorial which covers the development of
//! SPH methods in graphics over the past 20 years.

mod kernels;
mod parameters;
mod particles;
mod simulation;

pub use parameters::SphParamaters;
pub use simulation::SphSimulation;
