// https://leetcode.com/problems/best-time-to-buy-and-sell-stock/
// max diff between increasing elements
// similar to kadane's algorithm

use std::cmp::{max, min};

pub fn max_profit(prices: &Vec<i32>) -> i32 {
    // max val in the end
    // min val in the front
    // no profit return 0

    if prices.is_empty() {
        return 0;
    }
    let mut max_diff = 0;
    let mut known_min_val = prices[0];

    for item in prices {
        if item < &known_min_val {
            known_min_val = *item;
        }
        // only interested if there is an increasing pattern found
        // no need to calculate for every iteration
        else if item - known_min_val > max_diff {
            max_diff = item - known_min_val;
        }
    }
    max_diff
}

pub fn max_profit_v2(prices: &Vec<i32>) -> i32 {
    let mut known_min = std::i32::MAX;
    let mut mx = 0;
    for p in prices {
        known_min = min(known_min, *p);
        mx = max(mx, p - known_min);
    }
    mx
}

pub fn max_profit_one_line(prices: &Vec<i32>) -> i32 {
    prices
        .iter()
        .fold((std::i32::MAX, 0), |(buy, mx), &curr| {
            (min(buy, curr), max(mx, curr - buy))
        })
        .1
}
#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn max_profit_test() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        assert_eq!(5, max_profit(&prices));
        assert_eq!(5, max_profit_v2(&prices));
        assert_eq!(5, max_profit_one_line(&prices));
    }
}
