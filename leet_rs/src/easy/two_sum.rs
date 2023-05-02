// https://leetcode.com/problems/two-sum/
// no repeat of same elements

use std::collections::HashMap;
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut res: Vec<i32> = Vec::new();

    let mut cache = HashMap::<i32, i32>::new();
    for (i, n) in nums.iter().enumerate() {
        let diff = target - n; //tar: 9, [2, 7]
        if let Some(diff_val) = cache.get(&diff) {
            res.push(*diff_val);
            res.push(i as i32);
            return res;
        } else {
            cache.insert(*n, i as i32);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::two_sum;
    #[test]
    fn two_sum_test() {
        let nums = vec![2, 7, 11, 15];
        let res = two_sum(nums, 9);
        assert_eq!(res, vec![0, 1]);
        println!("done")
    }
}
