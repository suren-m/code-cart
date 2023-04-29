fn split_num(n: u32) -> Vec<u32> {
    let mut res = Vec::new();
    let mut n = n;
    while n > 0 {
        res.push(n % 10);
        n /= 10;
    }
    res.reverse();
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(split_num(123), vec![1,2,3]);
    }
}
