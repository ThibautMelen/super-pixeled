//! 2D point type for pixel coordinates.

use serde::{Deserialize, Serialize};

/// 2D coordinate point.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    /// Create a new point.
    #[inline]
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Origin point (0, 0).
    pub const ORIGIN: Self = Self::new(0, 0);

    /// Check if point is within bounds (0..width, 0..height).
    #[inline]
    pub const fn in_bounds(self, width: u32, height: u32) -> bool {
        self.x >= 0 && self.y >= 0 && (self.x as u32) < width && (self.y as u32) < height
    }

    /// Convert to linear index for a buffer with given width.
    #[inline]
    pub const fn to_index(self, width: u32) -> Option<usize> {
        if self.x >= 0 && self.y >= 0 {
            Some((self.y as usize) * (width as usize) + (self.x as usize))
        } else {
            None
        }
    }
}

impl From<(i32, i32)> for Point {
    fn from((x, y): (i32, i32)) -> Self {
        Self::new(x, y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in_bounds() {
        let p = Point::new(5, 10);
        assert!(p.in_bounds(64, 32));
        assert!(!p.in_bounds(5, 32)); // x == width
        assert!(!Point::new(-1, 0).in_bounds(64, 32));
    }

    #[test]
    fn test_to_index() {
        let p = Point::new(5, 2);
        assert_eq!(p.to_index(64), Some(2 * 64 + 5));
        assert_eq!(Point::new(-1, 0).to_index(64), None);
    }
}
