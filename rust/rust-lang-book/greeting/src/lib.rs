pub fn greeting(name: &str) -> String {
    //format!("Hello, {}!", name)
    format!("Hello")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn greeting_contains_name() {
        let name = "Carol";
        let result = greeting(&name);
        assert!(result.contains(&name),
        "Greeting did not contain name, value was `{}`", result
        );
    }
}
