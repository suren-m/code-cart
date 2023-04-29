fn sum_of_squares(n: u32) -> u32 {
    (1..=n).map(|x| x * x).sum()
}

// using fold
// sig: fold(init, fn(acc, x))
fn sum_of_squares_fold(n: u32) -> u32 {
    (1..=n).map(|x| x * x).fold(0, |acc, x_sq| acc + x_sq)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(sum_of_squares(3), 14);
        assert_eq!(sum_of_squares_fold(3), 14);
    }
}
