use fontdue::Font;
use stb_truetype::FontInfo;

pub struct TextMeasurer {
    font: Font,
    stb_font: FontInfo<Vec<u8>>,
}

impl TextMeasurer {
    pub fn new(font_data: &[u8]) -> Option<Self> {
        let settings = fontdue::FontSettings {
            collection_index: 0,
            scale: 40.0,
            load_substitutions: true,
        };
        let font = Font::from_bytes(font_data, settings).ok()?;

        let offset = stb_truetype::get_font_offset_for_index(font_data, 0)?;
        let stb_font = FontInfo::new(font_data.to_vec(), offset as usize)?;

        Some(Self { font, stb_font })
    }

    pub fn from_font_with_data(font: Font, font_data: &[u8]) -> Option<Self> {
        let offset = stb_truetype::get_font_offset_for_index(font_data, 0)?;
        let stb_font = FontInfo::new(font_data.to_vec(), offset as usize)?;
        Some(Self { font, stb_font })
    }

    /// Measure character advance matching C++ STB Truetype:
    /// px = round(font_size * 1.20)
    /// scale = stbtt_ScaleForPixelHeight(px)
    /// advance = stbtt_GetGlyphHMetrics(glyph).advance_width * scale
    pub fn measure_char_advance(&self, ch: char, font_size: f32) -> f32 {
        let px = (font_size * 1.20_f32).round();
        let scale = self.stb_font.scale_for_pixel_height(px);
        let cp = ch as u32;
        let glyph = self.stb_font.find_glyph_index(cp);
        let glyph = if glyph == 0 && cp != 0 {
            self.stb_font.find_glyph_index('?' as u32)
        } else {
            glyph
        };
        if glyph == 0 {
            return 0.0;
        }
        let hm = self.stb_font.get_glyph_h_metrics(glyph);
        (hm.advance_width as f32 * scale).max(0.0)
    }

    pub fn measure_width(&self, text: &str, font_size: f32) -> f32 {
        let px = (font_size * 1.20_f32).round();
        let scale = self.stb_font.scale_for_pixel_height(px);
        let mut width = 0.0;
        for ch in text.chars() {
            let cp = ch as u32;
            let glyph = self.stb_font.find_glyph_index(cp);
            let glyph = if glyph == 0 && cp != 0 {
                self.stb_font.find_glyph_index('?' as u32)
            } else {
                glyph
            };
            if glyph == 0 {
                continue;
            }
            let hm = self.stb_font.get_glyph_h_metrics(glyph);
            width += (hm.advance_width as f32 * scale).max(0.0);
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
