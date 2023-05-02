// In py
// max(sum(acc) for acc in accounts)
// max(map(sum, accounts))
pub fn maximum_wealth_func(accounts: &Vec<Vec<i32>>) -> i32 {
    let sum: Vec<i32> = accounts
        .iter()
        .map(|x| x.iter().fold(0, |acc, x| acc + x))
        .collect();
    *sum.iter().max().unwrap()

    // let mut max = 0;
    // for acc in accounts {
    //     let sum = acc.iter().fold(0, |acc, x| acc + x);

    //     max = std::cmp::max(sum, max);
    // }
    //max
}

pub fn maximum_wealth(accounts: &Vec<Vec<i32>>) -> i32 {
    let mut max = 0;
    for acc in accounts {
        let mut total = 0;
        for bal in acc {
            total = total + bal;
        }
        if total > max {
            max = total;
        }
    }
    return max;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maximum_wealth_test() {
        let accounts = vec![vec![1, 5], vec![7, 3], vec![3, 5]];
        assert_eq!(10, maximum_wealth(&accounts));
        assert_eq!(10, maximum_wealth_func(&accounts));
    }
}
