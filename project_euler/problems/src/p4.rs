#![allow(dead_code)]

// Problem 4: Palindrome Product
fn max_palyndrome(n_min: usize, n_max: usize) -> Option<usize> {
    if n_min > n_max {
        return None;
    }

    fn is_palyndrome(p: usize) -> bool {
        let p_str = p.to_string();
        let rev_p = p_str.chars().rev().collect::<String>();
        rev_p == p_str
    }

    let mut max_p = None;

    for n in n_min..n_max {
        for m in n..n_max {
            let p = n * m;
            if is_palyndrome(p) {
                max_p = Some(p)
            }
        }
    }

    max_p
}

#[cfg(test)]
mod tests {

    #[test]
    fn solution_test() {
        assert_eq!(super::max_palyndrome(1, 100), Some(9009));
    }

    // NB: solution is ommited due to the policy of PE (don't spoil solutions)
    #[test]
    #[ignore]
    fn solution_question() {
        assert_eq!(super::max_palyndrome(100, 999), Some(9009));
    }
}
