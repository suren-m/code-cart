use std::env;
use std::fs;

/// also see: https://github.com/rust-lang/glob
fn ls(path: &str) {
    let paths = fs::read_dir(path).unwrap();

    for path in paths {
        println!("{}", path.unwrap().path().display());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        ls("./");

        // prints project dir
        println!("{:?}", env::current_dir().unwrap());
        let x = env::var("TEST_VAR").unwrap_or("hello world".to_string());
        println!("{}", x);
    }
}
