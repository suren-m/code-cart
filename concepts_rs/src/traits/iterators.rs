/*
    # generators in py
    # prints squares of each num from 0..n
    def squares(n):
        for i in range(n):
            yield i**2

    for x in squares(5):
        print(x)

    # prints 0 1 4 9 16
*/

// Option 1: Just using a function
// Item is the type of val that gets returned 
// Map implements iterator trait
fn squares(n: u32) -> impl Iterator<Item = u32> {
    let x = (0..n).map(|x| x * x);
    x
}

// Option 2: using a function without map
fn squares_v2(n: u32) -> impl Iterator<Item = u32> {
    let mut curr = 0;
    std::iter::from_fn(move || {
        if curr < n {
            let res = curr * curr;
            curr += 1;
            Some(res)
        } else {
            None
        }
    })
}

// Option 3: Using a dedicated type
struct Squares {
    n: u32,
    curr: u32,
}

impl Squares {
    fn new(n: u32) -> Squares {
        Squares { n: n, curr: 0 }
    }
}

impl Iterator for Squares {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr < self.n {
            let res = Some(self.curr ^ 2); // or self.curr * self.curr
            self.curr = self.curr + 1;
            res
        } else {
            None
        }
    }
}

// cargo test iterators -- --nocapture
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        for x in Squares::new(5) {
            println!("{}", x);
        }

        for x in squares(5) {
            println!("squares func: {}", x);
        }
        
        for x in squares_v2(5) {
            println!("squares_v2 func: {}", x);
        }

    }
}
