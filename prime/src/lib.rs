mod plot;
mod prime_generator;

#[cfg(test)]
mod tests {
    use crate::plot::plot_spiral;
    use crate::prime_generator::PrimeSieve;

    #[test]
    fn generate_prime_plot() {
        let mut prime_sieve: PrimeSieve<100_000> = PrimeSieve::init();
        prime_sieve.sieve();
        let primes: Vec<f64> = prime_sieve
            .to_ints()
            .into_iter()
            .map(|f| f as f64)
            .collect::<Vec<_>>();

        plot_spiral(&primes);
    }
}
