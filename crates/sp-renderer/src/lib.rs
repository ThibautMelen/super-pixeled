//! Framebuffer and rendering utilities.
//!
//! Provides a zero-copy framebuffer for 64x32 LED panel rendering.

use sp_core::{Color, Point};

/// Framebuffer for LED panel rendering.
///
/// Stores pixel data in RGB format, row-major order.
/// Designed for zero-copy transfer to HUB75 driver.
#[derive(Clone)]
pub struct Framebuffer {
    width: u32,
    height: u32,
    data: Vec<Color>,
}

impl Framebuffer {
    /// Create a new framebuffer with given dimensions.
    pub fn new(width: u32, height: u32) -> Self {
        let size = (width * height) as usize;
        Self {
            width,
            height,
            data: vec![Color::BLACK; size],
        }
    }

    /// Get framebuffer width.
    #[inline]
    pub const fn width(&self) -> u32 {
        self.width
    }

    /// Get framebuffer height.
    #[inline]
    pub const fn height(&self) -> u32 {
        self.height
    }

    /// Get total pixel count.
    #[inline]
    pub fn pixel_count(&self) -> usize {
        self.data.len()
    }

    /// Clear framebuffer to black.
    pub fn clear(&mut self) {
        self.data.fill(Color::BLACK);
    }

    /// Fill framebuffer with a color.
    pub fn fill(&mut self, color: Color) {
        self.data.fill(color);
    }

    /// Get pixel color at position.
    #[inline]
    pub fn get(&self, point: Point) -> Option<Color> {
        if point.in_bounds(self.width, self.height) {
            let idx = (point.y as usize) * (self.width as usize) + (point.x as usize);
            Some(self.data[idx])
        } else {
            None
        }
    }

    /// Set pixel color at position.
    #[inline]
    pub fn set(&mut self, point: Point, color: Color) {
        if point.in_bounds(self.width, self.height) {
            let idx = (point.y as usize) * (self.width as usize) + (point.x as usize);
            self.data[idx] = color;
        }
    }

    /// Get raw pixel data (for driver transfer).
    #[inline]
    pub fn data(&self) -> &[Color] {
        &self.data
    }

    /// Get mutable raw pixel data.
    #[inline]
    pub fn data_mut(&mut self) -> &mut [Color] {
        &mut self.data
    }

    /// Draw a horizontal line.
    pub fn draw_hline(&mut self, y: i32, x1: i32, x2: i32, color: Color) {
        let (start, end) = if x1 <= x2 { (x1, x2) } else { (x2, x1) };
        for x in start..=end {
            self.set(Point::new(x, y), color);
        }
    }

    /// Draw a vertical line.
    pub fn draw_vline(&mut self, x: i32, y1: i32, y2: i32, color: Color) {
        let (start, end) = if y1 <= y2 { (y1, y2) } else { (y2, y1) };
        for y in start..=end {
            self.set(Point::new(x, y), color);
        }
    }

    /// Draw a filled rectangle.
    pub fn fill_rect(&mut self, x: i32, y: i32, w: u32, h: u32, color: Color) {
        for dy in 0..h as i32 {
            for dx in 0..w as i32 {
                self.set(Point::new(x + dx, y + dy), color);
            }
        }
    }
}

impl Default for Framebuffer {
    fn default() -> Self {
        Self::new(64, 32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let fb = Framebuffer::new(64, 32);
        assert_eq!(fb.width(), 64);
        assert_eq!(fb.height(), 32);
        assert_eq!(fb.pixel_count(), 2048);
    }

    #[test]
    fn test_set_get() {
        let mut fb = Framebuffer::new(64, 32);
        let point = Point::new(10, 5);
        let color = Color::RED;

        fb.set(point, color);
        assert_eq!(fb.get(point), Some(color));
    }

    #[test]
    fn test_out_of_bounds() {
        let mut fb = Framebuffer::new(64, 32);
        let out = Point::new(100, 100);

        fb.set(out, Color::RED); // Should not panic
        assert_eq!(fb.get(out), None);
    }

    #[test]
    fn test_fill() {
        let mut fb = Framebuffer::new(8, 8);
        fb.fill(Color::BLUE);

        for y in 0..8 {
            for x in 0..8 {
                assert_eq!(fb.get(Point::new(x, y)), Some(Color::BLUE));
            }
        }
    }
}
