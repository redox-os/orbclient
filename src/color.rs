// SPDX-License-Identifier: MIT

use core::fmt;
use serde::de::{self, Deserializer};
use serde::Deserialize;

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

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Color, D::Error>
        where
            D: Deserializer<'de>,
    {
        deserializer.deserialize_i32(ColorVisitor)
    }
}

struct ColorVisitor;

impl<'de> de::Visitor<'de> for ColorVisitor {
    type Value = Color;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Color specification in HEX format '#ARGB'")
    }

    fn visit_str<E>(self, color_spec: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
    {
        let chars: Vec<char> = color_spec.chars().collect();

        if chars.len() == 9 && chars[0] == '#' {
            // TODO: Use slices or ranges in original String and avoid 4 String allocations here?
            let channels: &[String; 4] = &[
                chars[1..3].iter().collect(),
                chars[3..5].iter().collect(),
                chars[5..7].iter().collect(),
                chars[7..9].iter().collect(),
            ];

            let a = u8::from_str_radix(&channels[0], 16)
                .map_err(|e| E::custom(e))?;
            let r = u8::from_str_radix(&channels[1], 16)
                .map_err(|e| E::custom(e.to_string()))?;
            let g = u8::from_str_radix(&channels[2], 16)
                .map_err(|e| E::custom(e.to_string()))?;
            let b = u8::from_str_radix(&channels[3], 16)
                .map_err(|e| E::custom(e.to_string()))?;

            Ok(Color::rgba(r, g, b, a))
        } else {
            Err(E::custom(format!("Invalid Color spec: '{}'", color_spec)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_derive::Deserialize;
    use toml;

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

    #[derive(Deserialize)]
    struct TestColor {
        color: Color,
    }

    #[test]
    fn deserialize_ok() {
        let color_spec = r##"color = "#00010203""##;
        let test_color: TestColor = toml::from_str(color_spec).expect("Color spec did not parse correctly");
        assert_eq!(test_color.color.a(), 0, "Alpha channel incorrect");
        assert_eq!(test_color.color.r(), 1, "Red channel incorrect");
        assert_eq!(test_color.color.g(), 2, "Green channel incorrect");
        assert_eq!(test_color.color.b(), 3, "Blue channel incorrect");
    }

    #[test]
    fn deserialize_hex() {
        let color_spec = r##"color = "#AABBCCDD""##;
        let _: TestColor = toml::from_str(color_spec).expect("Color spec did not parse HEX correctly");
    }

    #[test]
    fn deserialize_no_hash() {
        let color_spec = r##"color = "00010203""##;
        let test_color: Result<TestColor, _> = toml::from_str(color_spec);
        assert!(test_color.is_err(), "Color spec should not parse correctly without leading '#'");
    }

    #[test]
    fn deserialize_not_hex() {
        let color_spec = r##"color = "#GG010203""##;
        let test_color: Result<TestColor, _> = toml::from_str(color_spec);
        assert!(test_color.is_err(), "Color spec should not parse invalid HEX correctly");
    }

    #[test]
    fn deserialize_str_too_long() {
        let color_spec = r##"color = "#0001020304""##;
        let test_color: Result<TestColor, _> = toml::from_str(color_spec);
        assert!(test_color.is_err(), "Color spec should not parse invalid spec correctly");
    }

    #[test]
    fn deserialize_str_too_short() {
        let color_spec = r##"color = "#000102""##;
        let test_color: Result<TestColor, _> = toml::from_str(color_spec);
        assert!(test_color.is_err(), "Color spec should not parse invalid spec correctly");
    }
}
