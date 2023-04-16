use std::fmt;

trait Animal {
    fn make_noise(&self);
}

pub struct Cat<'a> {
    pub name: &'a str,
}

impl<'a> Cat<'a> {
    pub fn new(name: &'a str) -> Cat<'a> {
        Cat { name: name }
    }
}

impl<'a> fmt::Display for Cat<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "A cat with a lifetime and display called {}", self.name)
    }
}

impl<'a> Animal for Cat<'a> {
    fn make_noise(&self) {
        println!("Meow is the sound of {}..", &self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let cat = Cat { name: "felix" };
        let output = format!("{}", cat);
        assert_eq!(output, "A cat with a lifetime and display called felix");
    }
}
