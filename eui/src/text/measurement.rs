use fontdue::Font;

pub struct TextMeasurer {
    font: Font,
}

impl TextMeasurer {
    pub fn new(font_data: &[u8]) -> Option<Self> {
        let settings = fontdue::FontSettings {
            collection_index: 0,
            scale: 40.0,
            load_substitutions: true,
        };
        let font = Font::from_bytes(font_data, settings).ok()?;
        Some(Self { font })
    }

    pub fn from_font(font: Font) -> Self {
        Self { font }
    }

    pub fn measure_char_advance(&self, ch: char, font_size: f32) -> f32 {
        self.font.metrics(ch, font_size).advance_width
    }

    pub fn measure_width(&self, text: &str, font_size: f32) -> f32 {
        let mut width = 0.0;
        for ch in text.chars() {
            width += self.font.metrics(ch, font_size).advance_width;
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
