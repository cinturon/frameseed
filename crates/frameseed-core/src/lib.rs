pub fn welcome_message() -> &'static str {
    "Welcome to the frameseed-core crate."
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_welcome_message() {
        assert_eq!(welcome_message(), "Welcome to the frameseed-core crate.");
    }
}
