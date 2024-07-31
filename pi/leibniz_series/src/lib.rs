/// https://en.wikipedia.org/wiki/Approximations_of_n
pub fn approximate_pi(n_samples: usize) -> f64 {
    // TODO could chunk the sample size into parallel ones
    let sum = (0..n_samples)
        .into_iter()
        .fold(0.0, |acc: f64, idx: usize| {
            let pow: i32 = (idx % 2) as i32;
            let fraction = (-1.0 as f64).powi(pow) / ((2 * idx + 1) as f64);
            acc + fraction
        });
    4.0 * sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convergence_speed() {
        for n_samples in [10, 100, 1_000, 10_000, 100_000, 1_000_000, 10_000_000, 100_000_000] {
            let approx = approximate_pi(n_samples);
            assert_eq!(approx as usize, 3);
            println!("π is approximately {approx} [nr_samples = {n_samples}]");
        }
    }
}
