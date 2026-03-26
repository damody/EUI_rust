use std::collections::HashMap;
use fontdue::Font;
use glow::HasContext;

pub struct GlyphEntry {
    pub u0: f32,
    pub v0: f32,
    pub u1: f32,
    pub v1: f32,
    pub width: f32,
    pub height: f32,
    pub advance_width: f32,
    pub offset_x: f32,
    pub offset_y: f32,
}

pub struct FontAtlas {
    pub texture: glow::Texture,
    pub atlas_w: u32,
    pub atlas_h: u32,
    pub cursor_x: u32,
    pub cursor_y: u32,
    pub row_height: u32,
    pub glyphs: HashMap<(char, u32), GlyphEntry>,
    pub font: Font,
    /// Ratio to convert STB pixel_height to fontdue font_size:
    /// render_fs = round(cmd_fs * 1.20) * stb_to_fontdue_ratio
    /// When 0.0, no STB correction is applied (icon fonts).
    pub stb_to_fontdue_ratio: f32,
}

impl FontAtlas {
    /// Convert a DrawCommand font_size to the fontdue render font_size.
    /// For text fonts: matches the STB measurement scale.
    /// For icon fonts (ratio=0): uses font_size directly.
    pub fn render_font_size(&self, cmd_font_size: f32) -> f32 {
        if self.stb_to_fontdue_ratio > 0.0 {
            (cmd_font_size * 1.20_f32).round() * self.stb_to_fontdue_ratio
        } else {
            cmd_font_size
        }
    }

    pub unsafe fn new(gl: &glow::Context, font: Font, stb_to_fontdue_ratio: f32) -> Self {
        let atlas_w = 1024u32;
        let atlas_h = 1024u32;
        let texture = gl.create_texture().expect("create font atlas texture");
        gl.bind_texture(glow::TEXTURE_2D, Some(texture));
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::LINEAR as i32);
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, glow::LINEAR as i32);
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::CLAMP_TO_EDGE as i32);
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::CLAMP_TO_EDGE as i32);

        let blank = vec![0u8; (atlas_w * atlas_h) as usize];
        gl.tex_image_2d(
            glow::TEXTURE_2D, 0, glow::RED as i32,
            atlas_w as i32, atlas_h as i32, 0,
            glow::RED, glow::UNSIGNED_BYTE, Some(&blank),
        );
        gl.bind_texture(glow::TEXTURE_2D, None);

        Self {
            texture,
            atlas_w,
            atlas_h,
            cursor_x: 1,
            cursor_y: 1,
            row_height: 0,
            glyphs: HashMap::new(),
            font,
            stb_to_fontdue_ratio,
        }
    }

    pub unsafe fn get_or_rasterize(&mut self, gl: &glow::Context, ch: char, font_size: f32) -> &GlyphEntry {
        let key = (ch, (font_size * 10.0) as u32);
        if self.glyphs.contains_key(&key) {
            return &self.glyphs[&key];
        }

        let (metrics, bitmap) = self.font.rasterize(ch, font_size);
        let gw = metrics.width as u32;
        let gh = metrics.height as u32;

        if self.cursor_x + gw + 1 > self.atlas_w {
            self.cursor_x = 1;
            self.cursor_y += self.row_height + 1;
            self.row_height = 0;
        }
        if self.cursor_y + gh + 1 > self.atlas_h {
            // Atlas full - just reuse origin (lossy but won't crash)
            self.cursor_x = 1;
            self.cursor_y = 1;
            self.row_height = 0;
        }

        let px = self.cursor_x;
        let py = self.cursor_y;

        if gw > 0 && gh > 0 {
            gl.bind_texture(glow::TEXTURE_2D, Some(self.texture));
            gl.pixel_store_i32(glow::UNPACK_ALIGNMENT, 1);
            gl.tex_sub_image_2d(
                glow::TEXTURE_2D, 0,
                px as i32, py as i32, gw as i32, gh as i32,
                glow::RED, glow::UNSIGNED_BYTE,
                glow::PixelUnpackData::Slice(&bitmap),
            );
            gl.pixel_store_i32(glow::UNPACK_ALIGNMENT, 4);
            gl.bind_texture(glow::TEXTURE_2D, None);
        }

        self.cursor_x += gw + 1;
        if gh > self.row_height {
            self.row_height = gh;
        }

        let aw = self.atlas_w as f32;
        let ah = self.atlas_h as f32;

        self.glyphs.insert(key, GlyphEntry {
            u0: px as f32 / aw,
            v0: py as f32 / ah,
            u1: (px + gw) as f32 / aw,
            v1: (py + gh) as f32 / ah,
            width: gw as f32,
            height: gh as f32,
            advance_width: metrics.advance_width,
            offset_x: metrics.xmin as f32,
            offset_y: metrics.ymin as f32,
        });

        &self.glyphs[&key]
    }

    pub unsafe fn destroy(&self, gl: &glow::Context) {
        gl.delete_texture(self.texture);
    }
}
