//! Brute Force implementation
//! Time: O(n^2)
//! Better version using Kadane's algorithm further below
fn max_sum_subarray_brute(n: &[i32]) -> Vec<i32> {
    let mut max_subarray = Vec::new();
    let mut max_sum = std::i32::MIN;

    for i in 0..n.len() {
        for j in i..n.len() {
            let subarray = &n[i..j + 1];
            let subarray_sum: i32 = subarray.iter().sum();

            if subarray_sum > max_sum {
                max_sum = subarray_sum;
                max_subarray = subarray.to_vec();
            }
        }
    }

    max_subarray
}

fn max_sum_subarray(n: &[i32]) -> Vec<i32> {
    // five vars
    let mut curr_sum = 0;
    let mut curr_start_index = 0;
    let mut max_start_index = 0;
    let mut max_end_index = 0;
    let mut max_sum = std::i32::MIN;

    for (i, num) in n.iter().enumerate() {
        curr_sum = curr_sum + num;

        if curr_sum > max_sum {
            max_sum = curr_sum;
            max_start_index = curr_start_index;
            max_end_index = i;
        }

        // if adding num to curr_sum results in a negative num,
        // then num is negative and no point including it in curr_subarray
        // remember three Cs
        if curr_sum < 0 {
            curr_sum = 0;
            curr_start_index = i + 1;
        }
    }
    n[max_start_index..=max_end_index].to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_kadane() {
        assert_eq!(
            max_sum_subarray(&vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            vec![4, -1, 2, 1]
        );
        assert_eq!(
            max_sum_subarray(&vec![4, -1, -2, 10, -7, -5]),
            vec![4, -1, -2, 10]
        );
    }

    #[test]
    fn test_brute() {
        assert_eq!(
            max_sum_subarray_brute(&vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            vec![4, -1, 2, 1]
        );
        assert_eq!(
            max_sum_subarray(&vec![4, -1, -2, 10, -7, -5]),
            vec![4, -1, -2, 10]
        );
    }
}
