use std::hash::{Hash, Hasher};

#[derive(Hash)]
struct User {
    id: u32,
    name: String,
    // add more fields as needed
}

/// default hasher is non-cryptographic
/// uses SipHash algorithm as a hash function
/// Fixed key of default hasher generated at compile time and embedded in the stdlib
/// this key is a random value and guaranteed to be different for each rs version
/// designed to prevent hash flooding
/// can't be used across machines
/// hash comparisons here are not always 100% accurate and can cause collisions
/// use it as a first-level filter for == checks, and fall back to precise comparisons if collisions detected
/// two objects with different values can still produce same hash value
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let obj1 = User {
            id: 42,
            name: "Jane".to_string(),
        };
        let mut obj2 = User {
            id: 42,
            name: "Jane".to_string(),
        };

        let mut hasher1 = std::collections::hash_map::DefaultHasher::new();
        let mut hasher2 = std::collections::hash_map::DefaultHasher::new();

        obj1.hash(&mut hasher1);
        obj2.hash(&mut hasher2);

        let hash_value1 = hasher1.finish();
        let hash_value2 = hasher2.finish();

        assert_eq!(hash_value1, hash_value2);

        // Notice this assertion shows even if values of both objects are now same
        // need to run hasher again
        obj2.name = "John".to_string();
        assert_eq!(hash_value1, hash_value2);

        obj2.hash(&mut hasher2);
        let hash_value2 = hasher2.finish();
        // upon re-running the hasher
        assert_ne!(hash_value1, hash_value2);
    }
}
