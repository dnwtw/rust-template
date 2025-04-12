fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    #[derive(serde::Serialize, serde::Deserialize)]
    struct User {
        name: String,
        age: u32,
    }

    #[test]
    fn test_serialize() {
        let user = User {
            name: String::from("Alice"),
            age: 30,
        };
        let serialized = serde_json::to_string(&user).unwrap();
        assert_eq!(serialized, r#"{"name":"Alice","age":30}"#);
    }

    #[test]
    fn test_deserialize() {
        let data = r#"{"name":"Bob","age":25}"#;
        let user: User = serde_json::from_str(data).unwrap();
        assert_eq!(user.name, "Bob");
        assert_eq!(user.age, 25);
    }
}
