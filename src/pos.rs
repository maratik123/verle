use glam::IVec2;

#[derive(Eq, PartialEq, Debug, Copy, Clone, Default)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

impl Pos {
    #[inline]
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl From<Pos> for IVec2 {
    #[inline]
    fn from(pos: Pos) -> Self {
        Self::new(pos.x, pos.y)
    }
}

impl From<IVec2> for Pos {
    #[inline]
    fn from(ivec2: IVec2) -> Self {
        Self::new(ivec2.x, ivec2.y)
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
    fn ivec2_from_pos() {
        assert_eq!(IVec2::new(1, 2), Pos::new(1, 2).into());
    }

    #[test]
    fn pos_from_ivec2() {
        assert_eq!(Pos::new(1, 2), IVec2::new(1, 2).into());
    }
}
