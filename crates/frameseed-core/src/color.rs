#[derive(Debug, PartialEq)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Rgba {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub fn black() -> Self {
        Self::new(0, 0, 0, 255)
    }

    pub fn white() -> Self {
        Self::new(255, 255, 255, 255)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(
            Rgba::new(12, 13, 14, 15),
            Rgba {
                r: 12,
                g: 13,
                b: 14,
                a: 15
            }
        );
    }

    #[test]
    fn test_black() {
        assert_eq!(Rgba::black(), Rgba::new(0, 0, 0, 255));
    }

    #[test]
    fn test_white() {
        assert_eq!(Rgba::white(), Rgba::new(255, 255, 255, 255));
    }
}
