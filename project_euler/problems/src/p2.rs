#![allow(dead_code)]
use std::usize;

struct FibonacciSeqElem {
    curr: usize,
    prev: usize,
}

impl FibonacciSeqElem {
    fn value(&self) -> usize {
        self.curr
    }
}

/// Infinite Fibonacci sequence
impl std::iter::Iterator for FibonacciSeqElem {
    type Item = FibonacciSeqElem;
    fn next(&mut self) -> Option<Self::Item> {
        Some(Self {
            curr: self.curr + self.prev,
            prev: self.curr,
        })
    }
}

struct FiniteFibonacciSeqGenerator {
    max_iter: usize,
    max_value: usize,
    n_curr: usize,
    cursor: FibonacciSeqElem,
}

impl FiniteFibonacciSeqGenerator {
    fn init(max_iter: usize, max_value: usize) -> Self {
        Self {
            max_iter,
            max_value,
            n_curr: 1,
            cursor: FibonacciSeqElem { curr: 1, prev: 1 },
        }
    }
}

impl Iterator for FiniteFibonacciSeqGenerator {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        if self.n_curr >= self.max_iter || self.cursor.value() >= self.max_value {
            return None;
        }
        if self.n_curr <= 2 {
            self.n_curr += 1;
            return Some(self.cursor.value());
        }

        self.n_curr += 1;
        self.cursor = self.cursor.next()?;
        let v = self.cursor.value();

        if v > self.max_value {
            return None;
        }
        return Some(self.cursor.value());
    }
}

fn solution(n_max: usize) -> usize {
    let generator = FiniteFibonacciSeqGenerator::init(n_max, n_max);

    let sum = generator
        .into_iter()
        .fold(0, |acc, v| if v % 2 == 0 { acc + v } else { acc });

    sum
}

#[cfg(test)]
mod tests {

    #[test]
    fn solution_test() {
        assert_eq!(super::solution(100), 44);
        assert_eq!(super::solution(145), 188);
    }

    // NB: actuall solution is ommited due to the policy of PE (don't spoil solutions)
    #[test]
    #[ignore]
    fn solution_question() {
        assert_eq!(super::solution(4_000_000), 0);
    }
}
