use std::{fmt, ops};

#[derive(Debug, Clone, Copy)]
struct Point<T> {
    x: T,
    y: T,
}

impl Point<i32> {
    fn val(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

impl ops::Add for Point<i32> {
    type Output = Point<i32>;

    fn add(self, other: Point<i32>) -> Point<i32> {
        Point::<i32> {
            //or just Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: fmt::Display> fmt::Display for Point<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {} y: {}", self.x, self.y)
    }
}

impl ops::Deref for Point<i32> {
    type Target = i32;
    // returning a tuple of (x, y) doesn't work
    // due to temporary value being moved
    fn deref(&self) -> &Self::Target {
        &self.x
    }
}

// fn double<T: Copy + std::ops::Add<Output=T>
// or
fn double<T>(item: T) -> T
where
    T: Copy + std::ops::Add<Output = T>,
{
    item + item // without Copy, item will be moved
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let p1 = Point { x: 5, y: 5 };
        let p2 = Point { x: 5, y: 5 };

        let p3 = p1 + p2;
        let (x, y) = p2.val();
        println!("{}", p3);

        let p3_double = double::<Point<i32>>(p3); // or just double(p3)
        println!("{}", p3_double)
    }
}
