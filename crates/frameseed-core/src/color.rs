#[derive(Debug, PartialEq, Copy, Clone)]
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

    pub fn invert(self) -> Self {
        Self::new(255 - self.r, 255 - self.g, 255 - self.b, self.a)
    }

    /// Parse a hex color string like `"#ff5e4d"` or `"ff5e4d"` (with or without `#`).
    pub fn from_hex(hex: &str) -> Option<Self> {
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 {
            return None;
        }
        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
        Some(Self::new(r, g, b, 255))
    }

    /// Format as a lowercase hex string like `"#ff5e4d"`.
    pub fn to_hex(self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}

pub fn lerp_u8(start: u8, end: u8, t: f32) -> u8 {
    let start = start as f32;
    let end = end as f32;
    (start + t * (end - start)) as u8
}

pub fn lerp_rgba(start: Rgba, end: Rgba, t: f32) -> Rgba {
    Rgba {
        r: lerp_u8(start.r, end.r, t),
        g: lerp_u8(start.g, end.g, t),
        b: lerp_u8(start.b, end.b, t),
        a: lerp_u8(start.a, end.a, t),
    }
}

#[test]
fn from_hex_with_hash() {
    assert_eq!(Rgba::from_hex("#ff5e4d"), Some(Rgba::new(255, 94, 77, 255)));
}

#[test]
fn from_hex_without_hash() {
    assert_eq!(Rgba::from_hex("ffc857"), Some(Rgba::new(255, 200, 87, 255)));
}

#[test]
fn from_hex_invalid_returns_none() {
    assert_eq!(Rgba::from_hex("xyz"), None);
    assert_eq!(Rgba::from_hex("#12345"), None);
}

#[test]
fn to_hex_roundtrip() {
    let c = Rgba::new(255, 94, 77, 255);
    assert_eq!(Rgba::from_hex(&c.to_hex()), Some(c));
}

#[test]
fn test_black() {
    assert_eq!(Rgba::black(), Rgba::new(0, 0, 0, 255));
}

#[test]
fn test_white() {
    assert_eq!(Rgba::white(), Rgba::new(255, 255, 255, 255));
}

#[test]
fn test_lerp_u8() {
    assert_eq!(lerp_u8(0, 255, 0.5), 127);
}

#[test]
fn test_lerp_rgba() {
    assert_eq!(lerp_rgba(Rgba::new(0, 0, 0, 255), Rgba::new(255, 255, 255, 255), 0.5), Rgba::new(127, 127, 127, 255));
}   