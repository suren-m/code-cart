// higher order functions
// https://doc.rust-lang.org/rust-by-example/fn/hof.html

fn is_odd(n: u32) -> bool {
    n % 2 == 1
}

// functional approach
// note: square of an odd num is also always odd
fn sum_of_squared_odd_num(limit: u32) -> u32 {
    (0..)
        .map(|n| n * n)
        .take_while(|&n_sq| n_sq < limit)
        .filter(|&n_sq| is_odd(n_sq))
        .sum()
}

// imperative example
fn sum_of_squared_odd_num_v2(limit: u32) -> u32 {
    let mut acc = 0; // sum
    for n in 0.. {
        let n_sq = n * n;
        if n_sq > limit {
            break;
        } else if is_odd(n_sq) {
            acc += n_sq;
        }
    }
    acc
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(sum_of_squared_odd_num(40), 35); // 1 + 9 + 25
        assert_eq!(sum_of_squared_odd_num_v2(40), 35);
    }
}
