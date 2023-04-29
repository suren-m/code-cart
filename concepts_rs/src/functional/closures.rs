fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let ans = do_twice(add_one, 5);
        assert_eq!(ans, 12);
    }
}