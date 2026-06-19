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