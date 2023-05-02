// https://leetcode.com/problems/roman-to-integer/
// also see, int to roman
use std::collections::HashMap;
//I can be placed before V (5) and X (10) to make 4 and 9.
//X can be placed before L (50) and C (100) to make 40 and 90.
//C can be placed before D (500) and M (1000) to make 400 and 900.

pub fn get_roman_nums() -> HashMap<String, i32> {
    let mut dict = HashMap::new();
    dict.insert("I".to_string(), 1);
    dict.insert("V".to_string(), 5);
    dict.insert("X".to_string(), 10);
    dict.insert("L".to_string(), 50);
    dict.insert("C".to_string(), 100);
    dict.insert("D".to_string(), 500);
    dict.insert("M".to_string(), 1000);
    return dict;
}

// no repeated characters of 4
// todo: check validity
// use biggest symbols when possible.
// For e.g: XV not VVV

// symbols are usually ordered most significant to least.
// here, C is more significant than X
// if they aren't then subtract. DXCI - D + C-X + I
pub fn roman_to_int(s: String) -> i32 {
    let roman_map = get_roman_nums();
    let mut sum = 0;
    let mut i = 0;
    // get current sym
    // check next sym to see if it's higher. if so, subtract and skip.
    // else add to sum and carry on.

    while i < s.len() {
        let curr_sym = &s[i..i + 1];
        let mut curr_sym_val = *roman_map.get(curr_sym).unwrap();
        if i + 1 == s.len() {
            sum = sum + curr_sym_val;
            break;
        }

        let next_sym = &s[i + 1..i + 2];
        let next_sym_val = *roman_map.get(next_sym).unwrap();

        if next_sym_val > curr_sym_val {
            curr_sym_val = next_sym_val - curr_sym_val;
            i = i + 1; // add extra increment to skip next element
        }
        sum = sum + curr_sym_val;
        i = i + 1;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn roman_to_int_test() {
        assert_eq!(3, roman_to_int("III".to_string()));
        assert_eq!(58, roman_to_int("LVIII".to_string()));
        // M = 1000, CM = 900, XC = 90 and IV = 4.
        assert_eq!(1994, roman_to_int("MCMXCIV".to_string()));
    }
}
