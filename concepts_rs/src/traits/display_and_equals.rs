// deriving Eq ensures that == has expected behaviour
// with respect to transitivity, symmetry & reflexivity
// must also implement (or derive) PartialEq
#[derive(Eq, PartialEq)]
struct User {
    id : u32,
    firstname: String,
    lastname: String
}

/// Display
impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "user: {} {}", self.firstname, self.lastname)
    }
}

// here username must be unique during equals comparison
// even if ids are different
// thus, PartialEq implemented separately
#[derive(Eq)]
struct UniqueUser {
    id: u32,
    username: String,
}

/// Override == 
impl PartialEq for UniqueUser {
    fn eq(&self, other: &Self) -> bool {
        self.username == other.username
    }
}

/// cargo test traits -- --nocapture
#[cfg(test)]
mod tests {
   use super::*;
   #[test]
   fn test() {
       let u1 = User { id: 1, firstname: String::from("Jon"), lastname: "Doe".to_string() };
       let u2 = User { id: 2, firstname: String::from("Jon"), lastname: "Doe".to_string() };

       // display 
       assert_eq!(format!("{}", u2), "user: Jon Doe");
    
       // default equals (different ids)
       assert!(u1 != u2);

       // unique user
       let uu1 = UniqueUser { id: 1, username: "Jon".to_string() };
       let uu2 = UniqueUser { id: 2, username: "Jon".to_string() };
       // ids different, but username same, so duplicate user
       assert!(uu1 == uu2);
   }
}