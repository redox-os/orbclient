// SPDX-License-Identifier: MIT

use core::fmt;

/// A color
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Color {
    pub data: u32,
}

impl Color {
    /// Create a new color from RGB
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color {
            #[cfg(not(target_arch = "wasm32"))]
            data: 0xFF000000 | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32),
            #[cfg(target_arch = "wasm32")]
            data: 0xFF000000 | ((b as u32) << 16) | ((g as u32) << 8) | (r as u32),
        }
    }

    /// Set the alpha
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color {
            #[cfg(not(target_arch = "wasm32"))]
            data: ((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32),
            #[cfg(target_arch = "wasm32")]
            data: ((a as u32) << 24) | ((b as u32) << 16) | ((g as u32) << 8) | (r as u32),
        }
    }

    /// Get the r value
    #[cfg(not(target_arch = "wasm32"))]
    pub fn r(&self) -> u8 {
        ((self.data & 0x00FF0000) >> 16) as u8
    }

    /// Get the r value
    #[cfg(target_arch = "wasm32")]
    pub fn r(&self) -> u8 {
        (self.data & 0x000000FF) as u8
    }

    /// Get the g value
    pub fn g(&self) -> u8 {
        ((self.data & 0x0000FF00) >> 8) as u8
    }

    /// Get the b value
    #[cfg(not(target_arch = "wasm32"))]
    pub fn b(&self) -> u8 {
        (self.data & 0x000000FF) as u8
    }

    #[cfg(target_arch = "wasm32")]
    pub fn b(&self) -> u8 {
        ((self.data & 0x00FF0000) >> 16) as u8
    }

    /// Get the alpha value
    pub fn a(&self) -> u8 {
        ((self.data & 0xFF000000) >> 24) as u8
    }

    /// Interpolate between two colors
    pub fn interpolate(start_color: Color, end_color: Color, scale: f64) -> Color {
        let r = Color::interp(start_color.r(), end_color.r(), scale);
        let g = Color::interp(start_color.g(), end_color.g(), scale);
        let b = Color::interp(start_color.b(), end_color.b(), scale);
        let a = Color::interp(start_color.a(), end_color.a(), scale);
        Color::rgba(r, g, b, a)
    }

    fn interp(start_color: u8, end_color: u8, scale: f64) -> u8 {
        ((end_color as f64 - start_color as f64) * scale + start_color as f64) as u8
    }
}

/// Compare two colors (Do not take care of alpha)
impl PartialEq for Color {
    fn eq(&self, other: &Color) -> bool {
        self.r() == other.r() && self.g() == other.g() && self.b() == other.b()
    }
}

impl fmt::Debug for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{:#010X}", { self.data })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partial_eq() {
        assert_eq!(Color::rgb(1, 2, 3), Color::rgba(1, 2, 3, 200));
        assert_ne!(Color::rgb(1, 2, 3), Color::rgba(11, 2, 3, 200));
        assert_eq!(Color::rgba(1, 2, 3, 200), Color::rgba(1, 2, 3, 200));
    }

    #[test]
    fn alignment() {
        assert_eq!(4, core::mem::size_of::<Color>());
        assert_eq!(8, core::mem::size_of::<[Color; 2]>());
        assert_eq!(12, core::mem::size_of::<[Color; 3]>());
        assert_eq!(16, core::mem::size_of::<[Color; 4]>());
        assert_eq!(20, core::mem::size_of::<[Color; 5]>());
    }
}
