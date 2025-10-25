pub fn get_hello() -> &'static str {
    "Hello, world!"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_hello_correct_output() {
        assert_eq!(get_hello(), "Hello, world!");
    }
}
