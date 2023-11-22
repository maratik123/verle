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
    pub fn flush<D: HasDisplayHandle, W: HasWindowHandle>(
        &self,
        buffer: &mut Buffer<'_, D, W>,
    ) -> Result<(), Error> {
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

        for x in -radius_i..=radius_i {
            if (center_x_i + x < rect.left() as i32) || (center_x_i + x >= rect.right() as i32) {
                continue;
            }
            for y in -radius_i..=radius_i {
                if (center_y_i + y < rect.top() as i32) || (center_y_i + y >= rect.bottom() as i32)
                {
                    continue;
                }
                let dist: u32 = (x * x) as u32 + (y * y) as u32;
                if dist < r2 + radius {
                    buffer[((center_y_i + y) as usize, (center_x_i + x) as usize)] =
                        if dist <= r2 - radius {
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
}

#[derive(Error, Debug, Copy, Clone, Eq, PartialEq)]
pub enum Error {
    #[error("Invalid buffer layout")]
    InvalidBufferLayout,
}
