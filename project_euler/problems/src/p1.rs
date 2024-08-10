#![allow(dead_code)]

// Problem 1: multiples of 3 and 5
fn solution(divisors: &[usize], n: usize) -> Option<usize> {
    if n == 0 || divisors.is_empty() {
        return None;
    }
    // TODO validations on divisors vs n
    let sum = (1..n).fold(0, |acc, idx| {
        if divisors.iter().any(|d| idx % d == 0) {
            acc + idx
        } else {
            acc
        }
    });
    Some(sum)
}

#[cfg(test)]
mod tests {

    #[test]
    fn solution_test() {
        assert_eq!(super::solution(&[3, 5], 10), Some(23));
    }

    // NB: solution is ommited due to the policy of PE (don't spoil solutions)
    #[test]
    #[ignore]
    fn solution_question() {
        assert_eq!(super::solution(&[3, 5], 1000), Some(0));
    }
}
