use glam::UVec2;

#[derive(Eq, PartialEq, Debug, Copy, Clone, Default)]
pub struct Pos {
    pub x: u32,
    pub y: u32,
}

impl Pos {
    #[inline]
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

impl From<Pos> for UVec2 {
    #[inline]
    fn from(pos: Pos) -> Self {
        Self::new(pos.x, pos.y)
    }
}

impl From<UVec2> for Pos {
    #[inline]
    fn from(uvec2: UVec2) -> Self {
        Self::new(uvec2.x, uvec2.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        assert_eq!(Pos::new(1, 2), Pos { x: 1, y: 2 });
    }

    #[test]
    fn uvec2_from_pos() {
        assert_eq!(UVec2::new(1, 2), Pos::new(1, 2).into());
    }

    #[test]
    fn pos_from_uvec2() {
        assert_eq!(Pos::new(1, 2), UVec2::new(1, 2).into());
    }
}
