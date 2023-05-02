use std::collections::HashMap;

fn how_sum(nums: &Vec<i64>, target: i64) -> Option<Vec<i64>> {
    match target {
        0 => Some(Vec::new()),
        x if x < 0 => None,
        _ => {
            for num in nums {
                let diff = target - num;
                if let Some(mut val) = how_sum(nums, diff) {
                    val.push(*num);
                    return Some(val);
                }
            }
            None
        }
    }
}

fn how_sum_memo(
    nums: &Vec<i64>,
    target: i64,
    memo: &mut HashMap<i64, Option<Vec<i64>>>,
) -> Option<Vec<i64>> {
    match target {
        0 => Some(Vec::<i64>::new()),
        x if x < 0 => None,
        _ => {
            if let Some(val) = memo.get(&target) {
                return val.clone();
            }
            for num in nums {
                let diff = target - num;
                if let Some(mut cached_val) = how_sum_memo(nums, diff, memo) {
                    cached_val.push(*num);
                    memo.insert(target, Some(cached_val.clone()));
                    return Some(cached_val);
                }
            }
            memo.insert(target, None);
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(how_sum(&vec![2, 3], 7).unwrap(), vec![3, 2, 2]);
        assert_eq!(how_sum(&vec![5, 3, 4, 7], 7).unwrap(), vec![4, 3]);
        assert_eq!(how_sum(&vec![5, 3, 7, 4], 7).unwrap(), vec![4, 3]);

        assert_eq!(
            how_sum_memo(&vec![2, 3], 7, &mut HashMap::new()).unwrap(),
            vec![3, 2, 2]
        );
        assert_eq!(
            how_sum_memo(&vec![5, 3, 4, 7], 7, &mut HashMap::new()).unwrap(),
            vec![4, 3]
        );

        // no combinations possible. is_none is true
        assert_eq!(
            how_sum_memo(&vec![2, 4], 7, &mut HashMap::new()).is_none(),
            true
        );

        // same number reuse
        assert_eq!(
            how_sum_memo(&vec![2, 3, 5], 8, &mut HashMap::new()).unwrap(),
            vec![2, 2, 2, 2]
        );

        // long runner
        assert_eq!(
            how_sum_memo(&vec![7, 14], 300, &mut HashMap::new()).is_none(),
            true
        );
        println!("done");
    }
}
