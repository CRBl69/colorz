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
        ColorRGB {
            red,
            green,
            blue,
        }
    }
}
