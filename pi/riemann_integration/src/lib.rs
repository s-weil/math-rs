use core::f64;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

/// We have $x^2 + y^2=1$, hence $f(x) = \sqrt{1-x^2}$.
pub fn upper_unit_circle(x: f64) -> Option<f64> {
    if x.abs() > 1.0 {
        return None;
    }
    Some(f64::sqrt(1.0 - x.powi(2)))
}

// TODO extend to variable interval
fn integrate_unit_interval(n_samples: usize) -> f64 {
    let delta = 1.0 / (n_samples as f64);
    let integral = (0..n_samples).into_iter().fold(0.0, |acc, idx| {
        let x = idx as f64 * delta;
        let Some(y) = upper_unit_circle(x) else {
            panic!("x is out of bounds");
        };
        let area = y * delta;
        acc + area
    });
    integral
}

pub fn approximate_pi(n_samples: usize) -> f64 {
    let area_unit_interval = integrate_unit_interval(n_samples);
    // due to symmetry
    4.0 * area_unit_interval
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convergence_speed() {
        for n_samples in [
            10,
            100,
            1_000,
            10_000,
            100_000,
            1_000_000,
            10_000_000,
            100_000_000,
        ] {
            let approx = approximate_pi(n_samples);
            assert_eq!(approx as usize, 3);
            println!("π is approximately {approx} [nr_samples = {n_samples}]");
        }
    }
}

