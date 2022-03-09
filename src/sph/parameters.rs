use crate::base::Range;
use crate::math::*;

/// A struct containing all of the user-tunable parameters for the SPH simulation
///
/// These must be set before starting a simulation.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SphParamaters {
    pub num_particles: usize,
    /// The time step
    pub delta_time: T,
    /// The radius of the smoothing kernel,
    pub h: T,
    /// The density of the fluid without any forces
    pub rest_density: T,
    /// The ideal gas constant used in the state equation pressure solver
    pub k: T,
    /// The viscosity constant
    pub mu: T,
    /// The force of gravity
    pub gravity: TV,
    /// The velocity damping at the boundary for the reflection boundary conditions
    pub velocity_damping: T,
    /// The simulation domain
    pub domain: Range<TV>,
}

impl Default for SphParamaters {
    fn default() -> Self {
        Self {
            num_particles: 0,
            delta_time: 0.01,
            h: 0.04,
            rest_density: 1000.,
            k: 4.,
            mu: 8.,
            gravity: TV::ith(1, -1.),
            velocity_damping: 0.8,
            domain: Range::new(TV::zeros(), TV::from_element(3.)),
        }
    }
}
