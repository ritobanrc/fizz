//! Contains implementations of several commonly used smoothing kernels in SPH.

use crate::math::*;

pub trait SmoothingKernel {
    fn value(_r: TV, _h: T) -> T {
        0.
    }

    fn gradient_mag(_r: TV, _h: T) -> T {
        0.
    }

    fn gradient(r: TV, h: T) -> TV {
        r.normalize() * Self::gradient_mag(r, h)
    }

    fn laplacian(_r: TV, _h: T) -> T {
        0.
    }
}

pub struct SpikyKernel;

impl SmoothingKernel for SpikyKernel {
    fn value(r: TV, h: T) -> T {
        let r_mag = r.magnitude();
        if r_mag >= 0. && r_mag <= h {
            let c = 15. / (std::f64::consts::PI * h.powi(6));
            let h_sub_r = h - r_mag;
            c * h_sub_r * h_sub_r * h_sub_r
        } else {
            0.
        }
    }

    fn gradient_mag(r: TV, h: T) -> T {
        let r_mag = r.magnitude();
        if r_mag >= 0. && r_mag <= h {
            let c = 15. * -3. / (std::f64::consts::PI * h.powi(6));
            let h_sub_r = h - r_mag;
            c * h_sub_r * h_sub_r
        } else {
            0.
        }
    }
}

pub struct Poly6Kernel;

impl SmoothingKernel for Poly6Kernel {
    fn value(r: TV, h: T) -> T {
        let c = 315. / (64. * std::f64::consts::PI * h.powi(9));
        let mag2 = r.magnitude_squared();

        if mag2 <= h * h && mag2 >= 0. {
            c * (h * h - mag2).powi(3)
        } else {
            0.
        }
    }

    fn gradient_mag(r: TV, h: T) -> T {
        let c = 315. / (64. * std::f64::consts::PI * h.powi(9));
        let mag2 = r.magnitude_squared();
        if mag2 <= h * h && mag2 > 0. {
            c * 3. * -2. * mag2.sqrt() * (h * h - mag2) * (h * h - mag2)
        } else {
            0.
        }
    }
}

pub struct ViscosityKernel;

impl SmoothingKernel for ViscosityKernel {
    fn laplacian(r: TV, h: T) -> T {
        let c = 45. / (std::f64::consts::PI * h.powi(6));

        let mag = r.magnitude();
        if mag <= h {
            c * (h - mag)
        } else {
            0.
        }
    }
}
