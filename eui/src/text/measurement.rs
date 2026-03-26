use fontdue::Font;

pub struct TextMeasurer {
    font: Font,
    stb_correction: f32,
}

impl TextMeasurer {
    pub fn new(font_data: &[u8]) -> Option<Self> {
        let settings = fontdue::FontSettings {
            collection_index: 0,
            scale: 40.0,
            load_substitutions: true,
        };
        let font = Font::from_bytes(font_data, settings).ok()?;
        let stb_correction = compute_stb_correction(&font);
        Some(Self { font, stb_correction })
    }

    pub fn from_font(font: Font) -> Self {
        let stb_correction = compute_stb_correction(&font);
        Self { font, stb_correction }
    }

    /// Measure character advance matching C++ STB Truetype scaling.
    /// C++ measures at round(font_size * 1.20) then uses STB's
    /// scale = size / (ascent - descent) instead of fontdue's size / units_per_em.
    pub fn measure_char_advance(&self, ch: char, font_size: f32) -> f32 {
        let px = (font_size * 1.20).round();
        self.font.metrics(ch, px).advance_width * self.stb_correction
    }

    pub fn measure_width(&self, text: &str, font_size: f32) -> f32 {
        let px = (font_size * 1.20).round();
        let mut width = 0.0;
        for ch in text.chars() {
            width += self.font.metrics(ch, px).advance_width * self.stb_correction;
        }
        width
    }

    pub fn measure_height(&self, font_size: f32) -> f32 {
        let metrics = self.font.horizontal_line_metrics(font_size);
        metrics.map(|m| m.ascent - m.descent + m.line_gap).unwrap_or(font_size * 1.2)
    }

    pub fn line_height(&self, font_size: f32) -> f32 {
        self.measure_height(font_size)
    }

    pub fn font(&self) -> &Font {
        &self.font
    }

    pub fn rasterize(&self, ch: char, font_size: f32) -> (fontdue::Metrics, Vec<u8>) {
        self.font.rasterize(ch, font_size)
    }
}

/// Compute STB correction factor: stb_truetype scales advance by
/// size / (ascent - descent), while fontdue scales by size / units_per_em.
/// The ratio = units_per_em / (ascent - descent).
fn compute_stb_correction(font: &Font) -> f32 {
    let metrics = font.horizontal_line_metrics(1.0);
    if let Some(m) = metrics {
        let ascent_descent = m.ascent - m.descent;
        if ascent_descent > 0.0001 {
            return 1.0 / ascent_descent;
        }
    }
    1.0
}
