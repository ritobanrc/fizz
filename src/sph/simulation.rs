use super::kernels::{Poly6Kernel, SmoothingKernel, SpikyKernel, ViscosityKernel};
use super::particles::SphParticles;
use super::SphParamaters;
use crate::base::{ArrayNd, Grid, Range, RangeIterator};
use crate::math::*;
use smallvec::SmallVec;
use tracing::instrument;

/// Contains all of the state needed for performing an SPH simulation
pub struct SphSimulation {
    pub particles: SphParticles,
    pub params: SphParamaters,
    pub time: T,

    /// The grid used for efficiently finding particles in the neighborhood.
    grid: Grid,
    /// Contains the indices of the particles located in each cell.
    cells: ArrayNd<SmallVec<[usize; 2]>>,
}

impl SphSimulation {
    #[instrument(skip_all)]
    pub fn advance_timestep(&mut self) {
        self.clear_arrays();
        self.fill_cells();
        self.calculate_densities();
        self.calculate_pressure();
        self.apply_pressure_force();
        self.apply_viscosity_force();
        self.apply_gravity();
        self.move_particles();
        self.enforce_boundaries();

        self.time += self.params.delta_time;
    }

    #[instrument(skip_all)]
    /// Clears arrays for the next timestep
    fn clear_arrays(&mut self) {
        self.particles.density.fill(0.);
        self.particles.force.fill(TV::zeros());
        self.cells.fill(SmallVec::new());
    }

    #[instrument(skip_all)]
    /// Places each particle into the correct cell for efficient neighbor searching.
    fn fill_cells(&mut self) {
        let position = &self.particles.position;

        for p in 0..self.params.num_particles {
            let x = position[p];
            let idx = self.grid.cell_index(x);
            self.cells[idx].push(p);
        }
    }

    fn get_neighbors(&self, x: TV) -> impl Iterator<Item = usize> + '_ {
        let idx = self.grid.cell_index(x);
        let range = Range::new(idx, idx).thickened(1);

        let h2 = self.params.h * self.params.h;

        RangeIterator::new(range)
            .filter_map(|i| self.cells.get(i))
            .flat_map(|cell| cell.iter())
            .filter(move |&&i| (self.particles.position[i] - x).magnitude_squared() < h2)
            .copied()
    }

    #[instrument(skip_all)]
    fn calculate_densities(&mut self) {
        let mass = &self.particles.mass;
        let position = &self.particles.position;

        for p in 0..self.params.num_particles {
            let x = position[p];
            let neighbors = self.get_neighbors(x);
            self.particles.density[p] = neighbors
                .map(|j| mass[j] * Poly6Kernel::value(x - position[j], self.params.h))
                .sum()
        }
    }

    #[instrument(skip_all)]
    fn calculate_pressure(&mut self) {
        let density = &self.particles.density;

        for p in 0..self.params.num_particles {
            let pressure = self.params.k * (density[p] - self.params.rest_density);
            self.particles.pressure[p] = pressure;
        }
    }

    #[instrument(skip_all)]
    fn apply_pressure_force(&mut self) {
        let mass = &self.particles.mass;
        let pressure = &self.particles.pressure;
        let density = &self.particles.density;
        let position = &self.particles.position;

        for i in 0..self.params.num_particles {
            let x = position[i];
            let pressure_i = self.particles.pressure[i];
            let neighbors = self.get_neighbors(x);

            let force_pressure = -neighbors
                .map(|j| {
                    if i == j {
                        return TV::zeros();
                    }
                    let r_ij = position[i] - position[j];

                    let pressure_j = pressure[j];

                    mass[j] * (pressure_i + pressure_j) / (2. * density[j])
                        * SpikyKernel::gradient(r_ij, self.params.h)
                })
                .sum::<TV>();

            self.particles.force[i] += force_pressure;
        }
    }

    #[instrument(skip_all)]
    fn apply_viscosity_force(&mut self) {
        let mass = &self.particles.mass;
        let density = &self.particles.density;
        let position = &self.particles.position;
        let velocity = &self.particles.velocity;

        for i in 0..self.params.num_particles {
            let x = position[i];
            let neighbors = self.get_neighbors(x);

            let force_viscosity = self.params.mu
                * neighbors
                    .map(|j| {
                        if i == j {
                            return TV::zeros();
                        }
                        let vdiff = velocity[j] - velocity[i];
                        let r_ij = position[i] - position[j];

                        mass[j] * vdiff / density[j]
                            * ViscosityKernel::laplacian(r_ij, self.params.h)
                    })
                    .sum::<TV>();

            self.particles.force[i] += force_viscosity;
        }
    }

    #[instrument(skip_all)]
    fn apply_gravity(&mut self) {
        let density = &self.particles.density;
        for i in 0..self.params.num_particles {
            let force_gravity = self.params.gravity * density[i];
            self.particles.force[i] += force_gravity;
        }
    }

    #[instrument(skip_all)]
    fn move_particles(&mut self) {
        let position = &mut self.particles.position;
        let velocity = &mut self.particles.velocity;
        let force = &self.particles.force;
        let mass = &self.particles.mass;

        let dt = self.params.delta_time;

        for p in 0..self.params.num_particles {
            velocity[p] += dt * force[p] / mass[p];
            position[p] += dt * velocity[p];
        }
    }

    #[instrument(skip_all)]
    fn enforce_boundaries(&mut self) {
        let position = &mut self.particles.position;
        let velocity = &mut self.particles.velocity;

        let domain = self.params.domain;

        for p in 0..self.params.num_particles {
            let pos = &mut position[p];
            let vel = &mut velocity[p];

            for a in 0..DIM {
                if pos[p] < domain.min[a] - 0.01 {
                    vel[a] *= -self.params.velocity_damping;
                    pos[a] = domain.min[a];
                }

                if pos[a] > domain.max[a] + 0.01 {
                    vel[a] *= -self.params.velocity_damping;
                    pos[a] = domain.max[a];
                }
            }
        }
    }
}
