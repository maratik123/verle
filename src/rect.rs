use crate::Pos;
use crate::Size;
use glam::UVec2;
use std::fmt::{Display, Formatter};
use std::ops::BitAnd;
use thiserror::Error;

#[derive(Eq, PartialEq, Debug, Copy, Clone, Default)]
pub struct Rect {
    top_left: Pos,
    bottom_right: Pos,
}

impl Rect {
    #[inline]
    pub fn try_new(top_left: Pos, bottom_right: Pos) -> Result<Self, Error> {
        if top_left.x > bottom_right.x {
            Err(Error::Invalid(
                Dimension::X,
                InvalidActual {
                    lower: top_left.x,
                    higher: bottom_right.x,
                },
            ))
        } else if top_left.y > bottom_right.y {
            Err(Error::Invalid(
                Dimension::Y,
                InvalidActual {
                    lower: top_left.y,
                    higher: bottom_right.y,
                },
            ))
        } else {
            Ok(Self {
                top_left,
                bottom_right,
            })
        }
    }

    #[inline]
    pub fn new(top_left: Pos, size: Size) -> Self {
        Self {
            top_left,
            bottom_right: (UVec2::from(top_left) + UVec2::from(size)).into(),
        }
    }

    #[inline]
    pub fn top_left(&self) -> Pos {
        self.top_left
    }

    #[inline]
    pub fn bottom_right(&self) -> Pos {
        self.bottom_right
    }

    #[inline]
    pub fn top_right(&self) -> Pos {
        Pos::new(self.right(), self.top())
    }

    #[inline]
    pub fn bottom_left(&self) -> Pos {
        Pos::new(self.left(), self.bottom())
    }

    #[inline]
    pub fn left(&self) -> u32 {
        self.top_left.x
    }

    #[inline]
    pub fn right(&self) -> u32 {
        self.bottom_right.x
    }

    #[inline]
    pub fn top(&self) -> u32 {
        self.top_left.y
    }

    #[inline]
    pub fn bottom(&self) -> u32 {
        self.bottom_right.y
    }

    #[inline]
    pub fn width(&self) -> u32 {
        self.right() - self.left()
    }

    #[inline]
    pub fn height(&self) -> u32 {
        self.bottom() - self.top()
    }

    #[inline]
    pub fn size(&self) -> Size {
        Size::new(self.width(), self.height())
    }

    #[inline]
    pub fn is_zero_size(&self) -> bool {
        (self.left()..self.right()).is_empty() || (self.top()..self.bottom()).is_empty()
    }

    #[inline]
    pub fn contains(&self, pos: Pos) -> bool {
        (self.left()..self.right()).contains(&pos.x) && (self.top()..self.bottom()).contains(&pos.y)
    }
}

impl<'a, 'b> BitAnd<&'a Rect> for &'b Rect {
    type Output = Option<Rect>;

    #[inline]
    fn bitand(self, rhs: &Rect) -> Self::Output {
        Rect::try_new(
            UVec2::from(self.top_left).max(rhs.top_left.into()).into(),
            UVec2::from(self.bottom_right)
                .min(rhs.bottom_right.into())
                .into(),
        )
        .ok()
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Dimension {
    X,
    Y,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct InvalidActual {
    pub lower: u32,
    pub higher: u32,
}

impl Display for Dimension {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Dimension::X => 'X',
                Dimension::Y => 'Y',
            }
        )
    }
}

#[derive(Error, Debug, Eq, PartialEq, Copy, Clone)]
pub enum Error {
    #[error("Invalid dimension {0}: lower: {} is expected not higher than upper: {}", .1.lower, .1.higher)]
    Invalid(Dimension, InvalidActual),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_success() {
        assert_eq!(
            Rect::try_new(Pos::new(1, 2), Pos::new(3, 4)),
            Ok(Rect {
                top_left: Pos { x: 1, y: 2 },
                bottom_right: Pos { x: 3, y: 4 }
            })
        );
    }

    #[test]
    fn new_x_err() {
        let err = Rect::try_new(Pos::new(3, 2), Pos::new(1, 4)).unwrap_err();
        assert_eq!(
            err,
            Error::Invalid(
                Dimension::X,
                InvalidActual {
                    lower: 3,
                    higher: 1
                }
            )
        );
        assert_eq!(
            err.to_string(),
            "Invalid dimension X: lower: 3 is expected not higher than upper: 1"
        )
    }

    #[test]
    fn new_y_err() {
        let err = Rect::try_new(Pos::new(1, 4), Pos::new(3, 2)).unwrap_err();
        assert_eq!(
            err,
            Error::Invalid(
                Dimension::Y,
                InvalidActual {
                    lower: 4,
                    higher: 2
                }
            )
        );
        assert_eq!(
            err.to_string(),
            "Invalid dimension Y: lower: 4 is expected not higher than upper: 2"
        )
    }

    #[test]
    fn new() {
        assert_eq!(
            Rect::new(Pos::new(1, 2), Size::new(3, 4)),
            Rect {
                top_left: Pos::new(1, 2),
                bottom_right: Pos::new(4, 6)
            }
        )
    }

    #[test]
    fn top_left() {
        assert_eq!(
            Rect {
                top_left: Pos::new(1, 2),
                bottom_right: Pos::new(3, 4)
            }
            .top_left(),
            Pos::new(1, 2)
        );
    }

    #[test]
    fn bottom_right() {
        assert_eq!(
            Rect {
                top_left: Pos::new(1, 2),
                bottom_right: Pos::new(3, 4)
            }
            .bottom_right(),
            Pos::new(3, 4)
        );
    }

    #[test]
    fn new_xy_err() {
        let err = Rect::try_new(Pos::new(3, 4), Pos::new(1, 2)).unwrap_err();
        assert_eq!(
            err,
            Error::Invalid(
                Dimension::X,
                InvalidActual {
                    lower: 3,
                    higher: 1
                }
            )
        );
        assert_eq!(
            err.to_string(),
            "Invalid dimension X: lower: 3 is expected not higher than upper: 1"
        )
    }

    #[test]
    fn intersection() {
        let intersection = &Rect::new(Pos::new(2, 2), Size::new(2, 1))
            & &Rect::new(Pos::new(3, 1), Size::new(3, 3));
        assert_eq!(
            intersection,
            Some(Rect::new(Pos::new(3, 2), Size::new(1, 1)))
        );
        assert!(!intersection.unwrap().is_zero_size());
    }

    #[test]
    fn empty_intersection() {
        let intersection = &Rect::new(Pos::new(1, 1), Size::new(1, 1))
            & &Rect::new(Pos::new(3, 3), Size::new(1, 1));
        assert_eq!(intersection, None);
    }

    #[test]
    fn zero_size_intersection_in_dot() {
        let intersection = &Rect::new(Pos::new(1, 1), Size::new(1, 1))
            & &Rect::new(Pos::new(2, 2), Size::new(1, 1));
        assert_eq!(
            intersection,
            Some(Rect::new(Pos::new(2, 2), Size::default()))
        );
        assert!(intersection.unwrap().is_zero_size());
    }

    #[test]
    fn zero_size_intersection_in_line() {
        let intersection = &Rect::new(Pos::new(1, 1), Size::new(1, 1))
            & &Rect::new(Pos::new(1, 2), Size::new(1, 1));
        assert_eq!(
            intersection,
            Some(Rect::new(Pos::new(1, 2), Size::new(1, 0)))
        );
        assert!(intersection.unwrap().is_zero_size());
    }

    #[test]
    fn top_right() {
        assert_eq!(
            Rect {
                top_left: Pos::new(1, 2),
                bottom_right: Pos::new(3, 4)
            }
            .top_right(),
            Pos::new(3, 2)
        );
    }

    #[test]
    fn bottom_left() {
        assert_eq!(
            Rect {
                top_left: Pos::new(1, 2),
                bottom_right: Pos::new(3, 4)
            }
            .bottom_left(),
            Pos::new(1, 4)
        );
    }

    #[test]
    fn left() {
        assert_eq!(
            Rect {
                top_left: Pos::new(1, 2),
                bottom_right: Pos::new(3, 4)
            }
            .left(),
            1
        );
    }

    #[test]
    fn right() {
        assert_eq!(
            Rect {
                top_left: Pos::new(1, 2),
                bottom_right: Pos::new(3, 4)
            }
            .right(),
            3
        );
    }

    #[test]
    fn top() {
        assert_eq!(
            Rect {
                top_left: Pos::new(1, 2),
                bottom_right: Pos::new(3, 4)
            }
            .top(),
            2
        );
    }

    #[test]
    fn bottom() {
        assert_eq!(
            Rect {
                top_left: Pos::new(1, 2),
                bottom_right: Pos::new(3, 4)
            }
            .bottom(),
            4
        );
    }

    #[test]
    fn width() {
        assert_eq!(
            Rect {
                top_left: Pos::new(1, 2),
                bottom_right: Pos::new(4, 9)
            }
            .width(),
            3
        );
    }

    #[test]
    fn height() {
        assert_eq!(
            Rect {
                top_left: Pos::new(1, 2),
                bottom_right: Pos::new(4, 9)
            }
            .height(),
            7
        );
    }

    #[test]
    fn size() {
        assert_eq!(
            Rect {
                top_left: Pos::new(1, 2),
                bottom_right: Pos::new(4, 9)
            }
            .size(),
            Size::new(3, 7)
        );
    }

    #[test]
    fn contains() {
        assert!(Rect {
            top_left: Pos::new(1, 1),
            bottom_right: Pos::new(3, 3)
        }
        .contains(Pos::new(2, 2)));
    }

    #[test]
    fn not_contains() {
        assert!(!Rect {
            top_left: Pos::new(1, 1),
            bottom_right: Pos::new(3, 3)
        }
        .contains(Pos::new(4, 4)));
    }

    #[test]
    fn not_contains_on_empty() {
        assert!(!Rect::default().contains(Pos::default()));
    }

    #[test]
    fn not_contains_on_bottom_right() {
        assert!(!Rect {
            top_left: Pos::new(1, 1),
            bottom_right: Pos::new(3, 3)
        }
        .contains(Pos::new(3, 3)));
    }
}
