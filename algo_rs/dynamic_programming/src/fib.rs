#![allow(dead_code)]
use std::collections::HashMap;

/// fact(5) = 5 x 4 x 3 x 2 x 1
/// fact(4) = 4 x 3 x 2 x 1
/// fact(5) = 5 x fact(4)

fn fact_brute(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => n * fact_brute(n - 1),
    }
}

fn fact(n: u64, memo: &mut HashMap<u64, u64>) -> u64 {
    match n {
        0 | 1 => 1,
        _ => {
            if let Some(val) = memo.get(&n) {
                *val
            } else {
                let res = n * fact(n - 1, memo);
                memo.insert(n, res);
                println!("Cache insert. contents: {:?}", memo);
                res
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let mut cache: HashMap<u64, u64> = HashMap::new();
        println!("computing fact of 8...");
        println!("fact 8 res {}", fact(8, &mut cache));
        println!("computing fact of 5..");
        println!("fact 5 res {}", fact(5, &mut cache));
        println!("computing fact of 10..");
        println!("fact 10 res: {}", fact(10, &mut cache));
    }
}
