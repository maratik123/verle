use crate::Size;
use crate::{Pos, Rect};
use ndarray::{Array, Array2};
use softbuffer::Buffer;
use thiserror::Error;
use winit::raw_window_handle::{HasDisplayHandle, HasWindowHandle};

pub struct Canvas {
    buffer: Array2<u32>,
    global_rect: Rect,
}

impl Canvas {
    #[inline]
    pub fn new(default_color: u32, size: Size) -> Self {
        Self {
            buffer: Array::from_elem((size.height as usize, size.width as usize), default_color),
            global_rect: Rect::new(Pos::new(0, 0), size),
        }
    }

    #[inline]
    pub fn flush<D, W>(&self, buffer: &mut Buffer<'_, D, W>) -> Result<(), Error>
    where
        D: HasDisplayHandle,
        W: HasWindowHandle,
    {
        buffer.copy_from_slice(self.buffer.as_slice().ok_or(Error::InvalidBufferLayout)?);
        Ok(())
    }

    #[inline]
    pub fn draw_circle(&mut self, center: Pos, radius: u32, outline_color: u32, fill_color: u32) {
        Self::draw_dot_or_circle_in_rect_inner(
            &mut self.buffer,
            center,
            radius,
            outline_color,
            fill_color,
            &self.global_rect,
        );
    }

    #[inline]
    pub fn draw_circle_in_rect(
        &mut self,
        center: Pos,
        radius: u32,
        outline_color: u32,
        fill_color: u32,
        rect: &Rect,
    ) {
        if let Some(rect) = &self.global_rect & rect {
            Self::draw_dot_or_circle_in_rect_inner(
                &mut self.buffer,
                center,
                radius,
                outline_color,
                fill_color,
                &rect,
            );
        }
    }

    #[inline]
    fn draw_dot_or_circle_in_rect_inner(
        buffer: &mut Array2<u32>,
        center: Pos,
        radius: u32,
        outline_color: u32,
        fill_color: u32,
        rect: &Rect,
    ) {
        if rect.is_zero_size() {
            return;
        }
        if radius == 0 {
            Self::draw_dot_in_rect_inner(buffer, center, outline_color, rect);
        } else {
            Self::draw_circle_in_rect_inner(
                buffer,
                center,
                radius,
                outline_color,
                fill_color,
                rect,
            );
        }
    }

    fn draw_circle_in_rect_inner(
        buffer: &mut Array2<u32>,
        center: Pos,
        radius: u32,
        outline_color: u32,
        fill_color: u32,
        rect: &Rect,
    ) {
        debug_assert!(radius != 0);
        debug_assert!(!rect.is_zero_size());

        let radius_i = radius as i32;
        let r2 = radius * radius;
        let center_x_i = center.x as i32;
        let center_y_i = center.y as i32;

        let clamp_x = |x: i32| x.clamp(rect.left() as i32, rect.right() as i32 - 1);
        let x_range = (
            clamp_x(center_x_i - radius_i),
            clamp_x(center_x_i + radius_i),
        );

        let clamp_y = |y: i32| y.clamp(rect.top() as i32, rect.bottom() as i32 - 1);
        let y_range = (
            clamp_y(center_y_i - radius_i),
            clamp_y(center_y_i + radius_i),
        );

        for x in x_range.0..=x_range.1 {
            let offset_x = x - center_x_i;
            let offset_x_2 = (offset_x * offset_x) as u32;
            for y in y_range.0..=y_range.1 {
                let offset_y = y - center_y_i;
                let dist = offset_x_2 + (offset_y * offset_y) as u32;
                if dist < r2 + radius {
                    buffer[(y as usize, x as usize)] = if dist <= r2 - radius {
                        fill_color
                    } else {
                        outline_color
                    }
                }
            }
        }
    }

    #[inline]
    pub fn draw_dot(&mut self, pos: Pos, color: u32) {
        Self::draw_dot_in_rect_inner(&mut self.buffer, pos, color, &self.global_rect);
    }

    #[inline]
    pub fn draw_dot_in_rect(&mut self, pos: Pos, color: u32, rect: &Rect) {
        if let Some(rect) = &self.global_rect & rect {
            Self::draw_dot_in_rect_inner(&mut self.buffer, pos, color, &rect);
        }
    }

    #[inline]
    fn draw_dot_in_rect_inner(buffer: &mut Array2<u32>, pos: Pos, color: u32, rect: &Rect) {
        if rect.contains(pos) {
            buffer[(pos.y as usize, pos.x as usize)] = color;
        }
    }

    #[inline]
    pub fn draw_line(&mut self, from: Pos, to: Pos, color: u32) {
        Self::draw_line_in_rect_inner(&mut self.buffer, from, to, color, &self.global_rect);
    }

    #[inline]
    pub fn draw_line_in_rect(&mut self, from: Pos, to: Pos, color: u32, rect: &Rect) {
        if let Some(rect) = &self.global_rect & rect {
            Self::draw_line_in_rect_inner(&mut self.buffer, from, to, color, &rect);
        }
    }

    fn draw_line_in_rect_inner(
        buffer: &mut Array2<u32>,
        from: Pos,
        to: Pos,
        color: u32,
        rect: &Rect,
    ) {
        if rect.is_zero_size()
            || (from.x < rect.left() && to.x < rect.left())
            || (from.x >= rect.right() && to.x >= rect.right())
            || (from.y < rect.top() && to.y < rect.top())
            || (from.y >= rect.bottom() && to.y >= rect.bottom())
        {
            return;
        }
        let dx = from.x.abs_diff(to.x) as i32;
        let sx = from.x < to.x;
        let dy = -(from.y.abs_diff(to.y) as i32);
        let sy = from.y < to.y;
        let mut error = dx + dy;
        let mut pos = from;

        loop {
            Self::draw_dot_in_rect_inner(buffer, pos, color, rect);
            if pos == to {
                break;
            }
            let e2 = 2 * error;
            if e2 >= dy {
                if pos.x == to.x {
                    break;
                }
                error += dy;
                if sx {
                    pos.x += 1;
                } else {
                    pos.x -= 1;
                }
            }
            if e2 <= dx {
                if pos.y == to.y {
                    break;
                }
                error += dx;
                if sy {
                    pos.y += 1;
                } else {
                    pos.y -= 1;
                }
            }
        }
    }
}

#[derive(Error, Debug, Copy, Clone, Eq, PartialEq)]
pub enum Error {
    #[error("Invalid buffer layout")]
    InvalidBufferLayout,
}
