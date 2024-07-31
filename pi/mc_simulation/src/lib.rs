use rand::distributions::Uniform;
use rand::prelude::Distribution;
use rand_chacha::{rand_core::SeedableRng, ChaCha8Rng};

const SEED: u64 = 42;

pub fn approximate_pi(nr_samples: usize, n_threads: usize) -> f64 {
    // TODO check that nr_samples > n_threads

    let num_circle_samples: Vec<usize> = (0..n_threads)
        .into_iter()
        .map(|thread_idx| {
            std::thread::spawn(move || {
                let range = Uniform::new(-1.0, 1.0);

                let mut rng = ChaCha8Rng::seed_from_u64(SEED);
                rng.set_stream(thread_idx as u64);
                let thread_samples: Vec<f64> =
                    range.sample_iter(rng).take(2 * nr_samples).collect();

                let xy_points = thread_samples.chunks(2);

                let num_circle: usize = xy_points.fold(0, |n, xy| {
                    if xy[0].powi(2) + xy[1].powi(2) <= 1.0 {
                        n + 1
                    } else {
                        n
                    }
                });

                num_circle
            })
        })
        .map(|handle| handle.join().unwrap())
        .collect();

    let num_circle: usize = num_circle_samples.iter().sum();
    let approx = 4. * (num_circle as f64) / ((nr_samples * n_threads) as f64);
    println!(
        "π is approximately {approx} [nr_samples = {}]",
        nr_samples * n_threads
    );

    approx
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convergence_speed() {
        for n_samples in [10, 100, 1_000, 10_000, 100_000, 1_000_000, 10_000_000] {
            let approx = approximate_pi(n_samples, 8);
            assert_eq!(approx as usize, 3);
        }
    }
}
