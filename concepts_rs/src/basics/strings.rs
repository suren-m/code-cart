// simple version
fn is_palindrome(s: &str) -> bool {
    let s = s.to_string();
    s == s.chars().rev().collect::<String>()
}

// better and works with spaces
fn is_palindrome_v2(s: &str) -> bool {
    let chars: Vec<char> = s
        .to_lowercase()
        .chars()
        .filter(|x| !x.is_whitespace())
        .collect();
    let len = chars.len();

    if len <= 1 {
        return true;
    }

    let midway = len / 2;
    let last_index = len - 1;

    for i in 0..midway {
        if chars[i] != chars[last_index - i] {
            return false;
        }
    }
    true
}

// extra allocation of new string
fn reverse(s: &mut String) -> &mut String {
    let reversed: String = s.chars().rev().collect();
    s.clear();
    s.push_str(&reversed);
    s
}

// using vec
fn reverse_v2(s: String) -> String {
    let mut bytes = s.into_bytes();
    let len = bytes.len();
    for i in 0..len/2 {
        bytes.swap(i, len - 1 - i);
    }
    let s = String::from_utf8(bytes).unwrap();
    s
}

// with references but unsafe
fn reverse_v3(s: &mut String) -> &mut String {
    let bytes = unsafe { s.as_bytes_mut() };
    let len = bytes.len();
    for i in 0..len/2 {
        bytes.swap(i, len - 1 - i);
    }
    *s = String::from_utf8(bytes.to_vec()).unwrap();
    s
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let s1 = "racecar";
        let s2 = "A man a plan a canal Panama";
        let s3 = "";
        let s4 = "deed";

        assert_eq!(is_palindrome_v2(s1), true);
        assert_eq!(is_palindrome_v2(s2), true);
        assert_eq!(is_palindrome_v2(s3), true);
        assert_eq!(is_palindrome_v2(s4), true);

        assert_eq!(reverse(&mut String::from("rev")), &mut String::from("ver"));
        assert_eq!(reverse_v2(String::from("rev")), String::from("ver"));
        assert_eq!(reverse_v3(&mut String::from("rev")), &mut String::from("ver"));

        let s = String::from("hello world");
        println!("{}",s.get(0..5).unwrap());
        let idx1 = s.find('o').unwrap(); // find first occurrence of 'o'
        println!("'o' idx1: {}", idx1);
        let idx2 = s[idx1+1..].find('o').unwrap() + idx1 + 1; // second occurrence of 'o'
        println!("'o' idx2: {}", idx2);
    }
}
