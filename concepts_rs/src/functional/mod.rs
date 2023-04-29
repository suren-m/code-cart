mod closures;
mod func_basics;
mod hof;    

use rand::prelude::*;
use rand::Rng;

#[derive(Debug)]
struct User<'a> {
    id: i32,
    name: &'a str,
}

impl<'a> User<'a> {
    fn load() -> Vec<User<'a>> {
        let mut users = Vec::new();
        let mut rng = rand::thread_rng();
        for i in 1..=10 {
            users.push(User {
                id: rng.gen_range(-50..=50),
                name: "user",
            });
        }
        users
    }
}

fn sort_abs<'a>(users: &'a mut [User]) -> &'a [User<'a>] {
    users.sort_by_key(|u| u.id.abs());
    users
}

// cargo test <name> -- --nocapture
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let mut users = User::load();
        println!("{:?}", users);
        println!("\n");
        println!("{:?}", sort_abs(&mut users));
    }
}
