//! https://www.quora.com/Is-there-a-mathematical-way-to-find-prime-numbers
//! Sieve of Eratosthenes

pub struct PrimeSieve<const N: usize> {
    pub prime_candidates: [bool; N],
}

impl<const N: usize> PrimeSieve<N> {
    pub fn init() -> PrimeSieve<N> {
        Self {
            prime_candidates: [true; N],
        }
    }

    fn to_idx(&self, p: usize) -> usize {
        (p - 1).div_ceil(2)
    }

    fn to_int(&self, idx: usize) -> usize {
        2 * idx + 1
    }

    fn apply(&mut self, p: usize) {
        let p_idx = self.to_idx(p);
        let mut idx = p_idx + p;
        while idx < N {
            self.prime_candidates[idx] = false;
            idx += p;
        }
    }

    fn next_prime_idx(&self, prev_idx: usize) -> Option<usize> {
        self.prime_candidates
            .iter()
            .enumerate()
            .find(|&(idx, is_candidate)| idx > prev_idx && *is_candidate)
            .map(|(idx, _)| idx)
    }

    pub fn sieve(&mut self) {
        let mut prev = 0;
        while let Some(p_idx) = self.next_prime_idx(prev) {
            prev = p_idx;
            self.apply(self.to_int(p_idx));
        }
    }

    pub fn to_ints(&self) -> Vec<usize> {
        self.prime_candidates
            .iter()
            .enumerate()
            .filter(|(_, &is_candidate)| is_candidate)
            .map(|(idx, _)| self.to_int(idx))
            .collect::<Vec<usize>>()
    }

    pub fn intersect(&mut self, other: &Self) {
        for i in 0..N {
            self.prime_candidates[i] = self.prime_candidates[i] && other.prime_candidates[i];
        }
    }
}

#[cfg(test)]
mod test {
    use super::PrimeSieve;

    #[test]
    fn primes_100() {
        let mut sieve: PrimeSieve<50> = PrimeSieve::init();
        sieve.sieve();

        let res = sieve.to_ints();
        let primes = vec![
            // 2, exclude even prime numbers
            1, // add 1 (although no prime)
            3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97,
        ];

        assert_eq!(primes, res);
    }

    #[test]
    fn primes_1000() {
        let primes = vec![
            // 2, exclude even prime numbers
            1, // add 1 (although no prime)
            3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179,
            181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271,
            277, 281, 283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379,
            383, 389, 397, 401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 463, 467, 479,
            487, 491, 499, 503, 509, 521, 523, 541, 547, 557, 563, 569, 571, 577, 587, 593, 599,
            601, 607, 613, 617, 619, 631, 641, 643, 647, 653, 659, 661, 673, 677, 683, 691, 701,
            709, 719, 727, 733, 739, 743, 751, 757, 761, 769, 773, 787, 797, 809, 811, 821, 823,
            827, 829, 839, 853, 857, 859, 863, 877, 881, 883, 887, 907, 911, 919, 929, 937, 941,
            947, 953, 967, 971, 977, 983, 991, 997,
        ];

        let mut prime_sieve: PrimeSieve<500> = PrimeSieve::init();
        prime_sieve.sieve();

        let res = prime_sieve.to_ints();
        assert_eq!(primes, res);
    }
}
