#![allow(dead_code)]

use std::collections::HashMap;

fn main() {}

// // Can't reuse same values
// fn can_sum_brute(nums: &Vec<isize>, target: isize) -> bool {
//     for (i, num) in nums.iter().enumerate() {
//         if *num == target {
//             return true;
//         }
//         for j in i + 1..nums.len() {
//             if nums[j] + *num == target {
//                 return true;
//             }
//         }
//     }
//     false
// }

/// Recursive versions can reuse same values
/// See unit tests
///                 [2, 3, 4, 7]
///
fn can_sum_rec(nums: &Vec<isize>, target: isize) -> bool {
    match target {
        0 => true,           // base case
        x if x < 0 => false, // invalid case
        _ => {
            for i in nums {
                let diff = target - *i;
                if can_sum_rec(nums, diff) == true {
                    return true; // stack build up from base [true, true, true, etc.]
                }
            }
            false
        }
    }
}

fn can_sum_rec_memo(nums: &Vec<isize>, target: isize, memo: &mut HashMap<isize, bool>) -> bool {
    println!("entering rec with: {:?} {} {:?}", nums, target, &memo);
    match target {
        0 => {
            println!("base case hit. return true");
            true
        }
        x if x < 0 => {
            println!("Invalid case hit. return false");
            false
        }
        _ => {
            if let Some(val) = memo.get(&target) {
                println!("Cache hit for {:?}. Returning {}", target, *val);
                return *val;
            }
            for i in nums {
                let diff = target - *i;
                if can_sum_rec_memo(nums, diff, memo) == true {
                    println!("Inserting {} as true", target);
                    memo.insert(target, true);
                    return true;
                }
            }
            println!("Inserting {} as false", target);
            memo.insert(target, false);
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_sum_test() {
        dbg!("Can sum");
        let nums = vec![2, 3, 4, 7];
        // assert_eq!(true, can_sum_brute(&nums, 9));
        // assert_eq!(true, can_sum_brute(&nums, 5));
        // assert_eq!(false, can_sum_brute(&nums, 8));

        // assert_eq!(true, can_sum_iter(&nums, 9));
        // assert_eq!(true, can_sum_iter(&nums, 5));
        // assert_eq!(false, can_sum_iter(&nums, 8));

        assert_eq!(true, can_sum_rec(&nums, 9));
        assert_eq!(true, can_sum_rec(&nums, 5));
        // Recursive versions can reuse same values (like 2 times 4)
        assert_eq!(true, can_sum_rec(&nums, 8));

        //assert_eq!(true, can_sum_rec_memo(&nums, 9, &mut HashMap::new()));
        assert_eq!(true, can_sum_rec_memo(&nums, 5, &mut HashMap::new()));
        // assert_eq!(true, can_sum_rec_memo(&nums, 8, &mut HashMap::new()));

        dbg!("done");
    }

    // #[test]
    // fn can_sum_long_runner() {
    //     // long runner
    //     let nums = vec![7, 14];
    //     assert_eq!(false, can_sum_rec(&nums, 300));
    // }

    // #[test]
    // fn can_sum_long_runner_with_memo() {
    //     // long runner
    //     let nums = vec![7, 14];
    //     assert_eq!(false, can_sum_rec_memo(&nums, 300, &mut HashMap::new()));
    // }
}
