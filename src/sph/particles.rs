use crate::math::*;

/// Contains all SPH particle data
pub struct SphParticles {
    pub mass: Vec<T>,
    pub density: Vec<T>,
    pub pressure: Vec<T>,

    pub position: Vec<TV>,
    pub velocity: Vec<TV>,
    pub force: Vec<TV>,
}
