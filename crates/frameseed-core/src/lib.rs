mod color;
pub use color::{lerp_rgba, Rgba};

mod frame;
pub use frame::Frame;

pub fn welcome_message() -> &'static str {
    "Welcome to Frameseed."
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_welcome_message() {
        assert_eq!(welcome_message(), "Welcome to Frameseed.");
    }
}
