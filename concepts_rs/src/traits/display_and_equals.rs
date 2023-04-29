// all userinfos must also implement debug
trait UserInfo: std::fmt::Debug {
}


// deriving Eq ensures that == has expected behaviour
// with respect to transitivity, symmetry & reflexivity
// must also implement (or derive) PartialEq
#[derive(Debug)]
struct WebUserInfo {
    age: u32,
    firstname: String,
    lastname: String,
}
impl UserInfo for WebUserInfo {}
/// Display
impl std::fmt::Display for WebUserInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "user: {} {}", self.firstname, self.lastname)
    }
}

// unit structs example when using constraints on partialEq
#[derive(Eq, PartialEq, Debug)]
struct DefaultUserInfo;
impl UserInfo for DefaultUserInfo {}
#[derive(Debug)]
struct TestUserInfo;
impl UserInfo for TestUserInfo {}

// here username must be unique during equals comparison
// even if ids are different
// thus, PartialEq implemented separately
#[derive(Eq, PartialEq)]
struct UniqueUser<T> {
    id: u32,
    username: String,
    user_info: T,
}

/// Override ==
/// but only for WebUsers
impl PartialEq for UniqueUser<WebUserInfo> {
    fn eq(&self, other: &Self) -> bool {
        self.username == other.username
    }
}

/// Override ==
/// but only for TestUsers
impl PartialEq for UniqueUser<TestUserInfo> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.username == other.username
    }
}

fn printer(info: &impl std::fmt::Display) {
    println!("{}", info);
}

/// cargo test traits -- --nocapture
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let u1 = WebUserInfo {
            age: 32,
            firstname: String::from("John"),
            lastname: "Doe".to_string(),
        };
        let u2 = WebUserInfo {
            age: 30,
            firstname: String::from("Jane"),
            lastname: "Doe".to_string(),
        };

        // display
        assert_eq!(format!("{}", u2), "user: Jane Doe");

        printer(&u1);
        println!("{:?}", &u2);

        // create unique web user
        let wu1 = UniqueUser::<WebUserInfo> { id: 1, username: "Doe".to_string(), user_info: u1 };
        let wu2 = UniqueUser::<WebUserInfo> { id: 2, username: "Doe".to_string(), user_info: u2 };

        // uses partialeq comp implemented for WebUserInfo. (username should be same)
        assert!(wu1 == wu2);

        // test user info id && username equals
        let tu1 = UniqueUser::<TestUserInfo> {
            id: 1,
            username: "John".to_string(),
            user_info: TestUserInfo {},
        }; // {} optional for unit struct
        let tu2 = UniqueUser::<TestUserInfo> {
            id: 2,
            username: "John".to_string(),
            user_info: TestUserInfo {},
        };

        // uses partialeq for testuser (id should also be same)
        assert!(tu1 != tu2);
        
        // unique user with default user info (default equals comp)
        let du1 = UniqueUser::<DefaultUserInfo> {
            id: 1,
            username: "John".to_string(),
            user_info: DefaultUserInfo,
        };
        let du2 = UniqueUser::<DefaultUserInfo> {
            id: 2,
            username: "John".to_string(),
            user_info: DefaultUserInfo,
        };
        
        // default comparison for demo user
        // id different
        assert!(du1 != du2);

        // can't do this
        // assert!(wu1 == tu1);


    }
}
