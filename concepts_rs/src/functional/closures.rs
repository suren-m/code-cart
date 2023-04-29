fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn closures() {
    let mut name = String::from("john");

    let greet = |msg| format!("{} {}", msg, name);
    println!("{}", greet("hello"));

    // mutable closure that modifies captured variable
    let mut add_title = |title| {
        let name_ref = &mut name;
        // deref to assign new string
        *name_ref = format!("{} {}", title, name_ref).to_string();
    };
    add_title("Mr.");

    // exclusive closure. consumes x
    let consume_name = || {
        let new_name = name;
        println!("name: {}", new_name);
    };
    consume_name();

    // FnOnce can't be called again.
    // consume_name();

    // can't do. name already moved
    // println!("new name: {}", name);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let ans = do_twice(add_one, 5);
        assert_eq!(ans, 12);
        closures();
    }
}
