use crate::hsv::ColorHSV;

/// A struct that represents an RGB color
pub struct ColorRGB {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl ColorRGB {
    /// Returns a new ColorRGB struct with the given values
    /// # Example
    /// ```
    /// let color = ColorRGB {
    ///     red: 0,
    ///     green: 0,
    ///     blue: 0,
    /// }
    ///
    /// let rgb = ColorRGB::new(0, 0, 0);
    ///
    /// assert_eq!(color, rgb);
    /// ```
    pub fn new(red: u8, green: u8, blue: u8) -> ColorRGB {
        ColorRGB { red, green, blue }
    }

    pub fn as_hsv(&self) -> ColorHSV {
        // panic!("chat");
        let rgb_vec = vec![self.red as f32 / 255.0, self.green as f32 / 255.0, self.blue as f32 / 255.0];
        let x_max = *rgb_vec
            .iter()
            .reduce(|a,b| {
                if a > b {a} else {b}
            })
            .unwrap();

        let x_min = *rgb_vec
            .iter()
            .reduce(|a,b| {
                if a < b {a} else {b}
            })
            .unwrap();

        let chroma = x_max - x_min;

        let hue = if chroma == 0.0 {
            0.0
        } else if (x_max - rgb_vec[0]).abs() < 0.1 {
            60.0 * (0.0 + (rgb_vec[1] - rgb_vec[2]) / chroma)
        } else if (x_max - rgb_vec[1]).abs() < 0.1 {
            60.0 * (2.0 + (rgb_vec[2] - rgb_vec[0]) / chroma)
        } else {
            60.0 * (4.0 + (rgb_vec[0] - rgb_vec[1]) / chroma)
        };

        let saturation = if x_max == 0.0 { 0.0 } else { chroma / x_max };

        ColorHSV::new(hue, saturation, x_max)
    }
}
