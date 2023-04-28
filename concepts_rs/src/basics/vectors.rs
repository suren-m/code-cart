fn doubles(n: &[i32]) -> Vec<i32> {
    n.iter().map(|x| x * x).collect::<Vec<i32>>()
}

/// filter out even numbers
fn even_filter(n: &[i32]) -> Vec<i32> {
    //n.iter().filter(|x| *x % 2 == 1).copied().collect()
    // or
    // n.iter().filter(|&x| x % 2 == 1).cloned().collect()
    // or even better
    // just pass reference param into the closure fn instead of giving ownership
    n.iter().filter(|&x| x % 2 == 1).copied().collect()
}

/// retain only odd numbers (inplace filtering)
/// input has to be a Vec. Notice the inplace filter
/// had to call .to_vec()
fn even_filter_in_place(n: &mut Vec<i32>) -> Vec<i32> {
    n.retain(|&x| x % 2 == 1);
    n.to_vec()
}

// check every element against a predicate
fn is_all_positive(n: &[i32]) -> bool {
    n.iter().all(|&x| x > 0)
}

fn finder(n: &[i32], val: i32) -> Option<&i32> {
    n.iter().find(|&&x| x == val)
}

// cargo test vectors -- --nocapture
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let mut n = vec![1, 2, 3, 4, 5];
        assert_eq!(doubles(&n), vec![1, 4, 9, 16, 25]);
        assert_eq!(even_filter(&n), vec![1, 3, 5]);
        assert_eq!(even_filter_in_place(&mut n), vec![1, 3, 5]);
        print!("{:?}", &n); // [1,3,5]
        assert!(is_all_positive(&n)); // true
        assert_eq!(n.iter().find(|&&x| x == 3), Some(&3));
        assert_eq!(finder(&n, 3), Some(&3));

        // enumerator
        let enumerator = [10].iter().enumerate();
        for item in enumerator {
            assert!(item == (0, &10));
        }
    }

    #[test]
    fn vec_mutations_test() {
        // append a vec to another vec
        let mut seq1 = vec![-1, 3, 2];
        let mut seq2 = vec![1, 0, -3];
        seq1.append(&mut seq2);
        seq1.sort();
        assert_eq!(seq2, []);
        assert_eq!(seq1, vec![-3, -1, 0, 1, 2, 3]);

        seq1.push(4);
        seq1.insert(1, -2);
        assert_eq!(seq1, vec![-3, -2, -1, 0, 1, 2, 3, 4]);
        assert_eq!(Some(4), seq1.pop());
        // below can throw index out of bounds
        assert_eq!(-2, seq1.remove(1));
        // remove -1 and replace with 3 (last elem)
        assert_eq!(-1, seq1.swap_remove(1));
        assert_eq!(seq1, vec![-3, 3, 0, 1, 2]);
    }
}
