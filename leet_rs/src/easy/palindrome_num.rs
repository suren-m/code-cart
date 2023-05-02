// https://leetcode.com/problems/palindrome-number/
// all negative nums are not palindrome

// convert the number into string and check if the string is a palindrome,
// but this would require extra non-constant space for creating the string
fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }
    let x = x.to_string();
    let x_rev = x.chars().rev().collect::<String>();
    x == x_rev
}

// digit separation and array equality compare. much slower
fn is_palindrome_v2(x: i32) -> bool {
    if x < 0 {
        return false;
    }
    let mut num = x;
    let mut items = Vec::new();
    let mut stop = false;
    while !stop {
        items.push(num % 10);
        num = num / 10;
        if num == 0 {
            stop = true;
        }
    }
    let mut rev = items.clone();
    rev.reverse();
    items == rev
}

// Revert half the num
// e.g: 1221 = revert 21 and compare it with first two digits.
// 12 == 12 (reversed)
// e.g: 12321 = leave midpoint as such. do the same as above
fn is_palindrome_v3(x: i32) -> bool {
    if x < 0 {
        return false;
    }
    let mut reversed = 0;
    let mut remainder = x;
    let mut num = x;

    while num != 0 {
        remainder = num % 10; // last int store
        reversed = reversed * 10 + remainder;
        num = num / 10;
    }
    x == reversed
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn palindrome_num_test() {
        assert_eq!(true, is_palindrome(121));
        assert_eq!(false, is_palindrome(-121));
        assert_eq!(false, is_palindrome(123));

        assert_eq!(true, is_palindrome_v2(121));
        assert_eq!(false, is_palindrome_v2(-121));
        assert_eq!(false, is_palindrome_v2(123));

        assert_eq!(true, is_palindrome_v3(121));
        assert_eq!(false, is_palindrome_v3(-121));
        assert_eq!(false, is_palindrome_v3(123));
    }
}
