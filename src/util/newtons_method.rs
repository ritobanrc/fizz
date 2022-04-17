//! This module implements [Newton's Method](https://en.wikipedia.org/wiki/Newton%27s_method) for
//! solving nonlinear equations and systems of equations, when you know the derivative/Jacobian.
//!
//! The core idea is very simple:
//! 1. Start with a guess called `x`.
//! 2. Find the tangent line at `x` using the Jacobian.
//! 3. Find where the tangent line intersects the x-axis. That is your new guess.
//! 4. Repeat, starting from step 1, until convergence.
//!
//! Note that in the multivariable case, step 3 requires solving a linear system. For large numbers
//! of variables, it is common to use a Krylov Solver like Conjugate Gradient or GMRES (see Matrix
//! Computations by Golub and Van Loan, Ch. 11 or Numerical Linear Algebra by Trefethen and Bau,
//! Part VI). However, situations with fewer variables, we may simply use Gaussian Elimination.
use crate::math::*;

pub fn solve_linear_system(a: &Mat, b: &TV) -> TV {
    let lu = a.lu();
    lu.solve(b).expect(&format!(
        "Unable to solve linear system. A = {:?}, b = {:?}",
        a, b
    ))
}

pub fn newtons_method<F, G>(func: F, grad: G, initial_guess: TV) -> TV
where
    F: Fn(&TV) -> TV,
    G: Fn(&TV) -> Mat,
{
    let mut x = initial_guess;
    loop {
        let jacobian = grad(&x);
        let f = -func(&x);
        let delta = solve_linear_system(&jacobian, &f);
        x += delta;

        if delta.norm_squared() < 1e-12 {
            break;
        }
    }
    x
}

#[cfg(test)]
mod tests {
    #[test]
    #[cfg(feature = "3d")]
    fn test_newtons_method_3d() {
        use super::*;

        let x = newtons_method(
            |v| {
                let x = v.x;
                let y = v.y;
                let z = v.z;
                TV::new(
                    2. * x * x + y * y - z - 12.,
                    y * y + z - 10.,
                    x * x - 2. * z * z + 3. * y * y - 5.,
                )
            },
            |v| {
                let x = v.x;
                let y = v.y;
                let z = v.z;
                Mat::new(4. * x, 0., 2. * x, 2. * y, 2. * y, 6. * y, -1., 1., -4. * z).transpose()
            },
            TV::from_element(1.),
        );

        let actual = TV::new(2.0347124968015162, 2.6191496817401907, 3.1400549446402595);
        assert!((x - actual).norm_squared() < 1e-5)
    }
}
