use glam::UVec2;
use std::fmt::{Display, Formatter};

#[derive(Eq, PartialEq, Debug, Copy, Clone, Default)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

impl Size {
    #[inline]
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

impl From<Size> for UVec2 {
    #[inline]
    fn from(size: Size) -> Self {
        Self::new(size.width, size.height)
    }
}

impl From<UVec2> for Size {
    #[inline]
    fn from(uvec2: UVec2) -> Self {
        Self::new(uvec2.x, uvec2.y)
    }
}

impl Display for Size {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Size: width = {}, height = {}", self.width, self.height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        assert_eq!(
            Size::new(1, 2),
            Size {
                width: 1,
                height: 2
            }
        );
    }

    #[test]
    fn ivec2_from_size() {
        assert_eq!(UVec2::new(1, 2), Size::new(1, 2).into());
    }

    #[test]
    fn size_from_ivec2() {
        assert_eq!(Size::new(1, 2), UVec2::new(1, 2).into());
    }
}
