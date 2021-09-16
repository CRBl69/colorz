use crate::rgb::ColorRGB;

/// A struct that represents an HSV color
pub struct ColorHSV {
    pub hue: f32,
    pub saturation: f32,
    pub value: f32,
}

impl ColorHSV {
    /// Converts an hsv color to an rgb color
    /// # Example
    /// ```
    /// let hsv = ColorHSV::new(0.0, 0.0, 0.0);
    ///
    /// let color = ColorRBG::new(0, 0, 0);
    ///
    /// let rgb = hsv.to_rgb();
    ///
    /// assert_eq!(color, rgb);
    /// ```
    pub fn to_rgb(&self) -> ColorRGB {
        let chroma = self.value * self.saturation;

        let m: f32 = self.value - chroma;

        let x: f32 = (chroma as f32 * (1.0 - ((self.hue / 60_f32) % 2_f32 - 1.0).abs())) as f32;

        let (x, y, z) =
            (((chroma + m) * 255.0) as u8, ((x + m) * 255.0) as u8, ((0.0 + m) * 255.0) as u8);

        match (self.hue as i32 / 60) as u8 {
            1 => ColorRGB::new(y, x, z),
            2 => ColorRGB::new(z, x, y),
            3 => ColorRGB::new(z, y, x),
            4 => ColorRGB::new(y, z, x),
            5 => ColorRGB::new(x, z, y),
            _ => ColorRGB::new(x, y, z),
        }
    }

    /// Returns a new ColorHSV struct with the given values
    /// # Example
    /// ```
    /// let color = ColorHSV {
    ///     hue: 0.0,
    ///     saturation: 0.0,
    ///     value: 0.0,
    /// }
    ///
    /// let hsv = ColorHSV::new(0.0, 0.0, 0.0);
    ///
    /// assert_eq!(color, hsv);
    /// ```
    pub fn new(hue: f32, saturation: f32, value: f32) -> ColorHSV {
        ColorHSV {
            hue,
            saturation,
            value,
        }
    }
}