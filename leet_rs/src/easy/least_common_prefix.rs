use std::collections::HashMap;
#[cfg(test)]
pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut res = "".to_string();
    if strs.is_empty() {
        return res;
    }

    //let shortest_item = strs.iter().min().unwrap();

    // this seems much faster
    let shortest_item = strs.iter().min_by(|x, y| x.len().cmp(&y.len())).unwrap();

    for (i, s) in shortest_item.chars().enumerate() {
        for other in &strs {
            if other[i..i + 1] != s.to_string() {
                return shortest_item[0..i].to_string();
            }
        }
    }

    shortest_item.to_string()
}
mod tests {
    use super::*;
    #[test]
    fn longest_common_prefix_test() {
        //write test here
        let strs = vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string(),
        ];
        assert_eq!("fl".to_string(), longest_common_prefix(strs));
        let strs = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];
        assert_eq!("".to_string(), longest_common_prefix(strs));
    }
}
