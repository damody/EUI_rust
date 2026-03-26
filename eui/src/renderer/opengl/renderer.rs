use std::rc::Rc;

use glow::HasContext;

use crate::core::draw_command::*;
use crate::renderer::contracts::*;
use crate::renderer::opengl::font_renderer::FontAtlas;
use crate::renderer::opengl::image_cache::ImageCache;
use crate::renderer::opengl::shader;
use crate::renderer::opengl::vertex::*;
use crate::runtime::contracts::WindowMetrics;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
enum TextureMode {
    None = 0,
    AlphaMask = 1,
    Rgba = 2,
}

pub struct OpenGlRenderer {
    gl: Rc<glow::Context>,
    program: glow::Program,
    vbo: glow::Buffer,
    viewport_uniform: glow::UniformLocation,
    texture_uniform: glow::UniformLocation,
    texture_mode_uniform: glow::UniformLocation,
    font_atlas: Option<FontAtlas>,
    icon_font_atlas: Option<FontAtlas>,
    image_cache: ImageCache,
}

/// Returns true if the codepoint is in the Unicode Private Use Area
/// (used for icon fonts like Font Awesome).
fn is_private_use_codepoint(cp: u32) -> bool {
    cp >= 0xE000 && cp <= 0xF8FF
}

impl OpenGlRenderer {
    pub unsafe fn new(gl: Rc<glow::Context>) -> Result<Self, String> {
        let program = shader::create_program(&gl)?;
        let vbo = gl.create_buffer().map_err(|e| e.to_string())?;

        let viewport_uniform = gl.get_uniform_location(program, "u_viewport")
            .ok_or("u_viewport not found")?;
        let texture_uniform = gl.get_uniform_location(program, "u_texture")
            .ok_or("u_texture not found")?;
        let texture_mode_uniform = gl.get_uniform_location(program, "u_texture_mode")
            .ok_or("u_texture_mode not found")?;

        Ok(Self {
            gl,
            program,
            vbo,
            viewport_uniform,
            texture_uniform,
            texture_mode_uniform,
            font_atlas: None,
            icon_font_atlas: None,
            image_cache: ImageCache::new(),
        })
    }

    pub unsafe fn set_font(&mut self, font: fontdue::Font, stb_to_fontdue_ratio: f32) {
        if let Some(ref atlas) = self.font_atlas {
            atlas.destroy(&self.gl);
        }
        self.font_atlas = Some(FontAtlas::new(&self.gl, font, stb_to_fontdue_ratio));
    }

    pub unsafe fn set_icon_font(&mut self, font: fontdue::Font, stb_to_fontdue_ratio: f32) {
        if let Some(ref atlas) = self.icon_font_atlas {
            atlas.destroy(&self.gl);
        }
        self.icon_font_atlas = Some(FontAtlas::new(&self.gl, font, stb_to_fontdue_ratio));
    }

    unsafe fn flush_vertices(&self, vertices: &[Vertex], tex_mode: TextureMode, texture: Option<glow::Texture>) {
        if vertices.is_empty() {
            return;
        }

        let gl = &self.gl;
        gl.use_program(Some(self.program));

        gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.vbo));
        let data: &[u8] = std::slice::from_raw_parts(
            vertices.as_ptr() as *const u8,
            vertices.len() * VERTEX_SIZE,
        );
        gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, data, glow::STREAM_DRAW);

        let stride = VERTEX_SIZE as i32;
        gl.enable_vertex_attrib_array(0);
        gl.vertex_attrib_pointer_f32(0, 2, glow::FLOAT, false, stride, 0);
        gl.enable_vertex_attrib_array(1);
        gl.vertex_attrib_pointer_f32(1, 4, glow::FLOAT, false, stride, 8);
        gl.enable_vertex_attrib_array(2);
        gl.vertex_attrib_pointer_f32(2, 2, glow::FLOAT, false, stride, 24);

        gl.uniform_1_i32(Some(&self.texture_mode_uniform), tex_mode as i32);

        if let Some(tex) = texture {
            gl.active_texture(glow::TEXTURE0);
            gl.bind_texture(glow::TEXTURE_2D, Some(tex));
            gl.uniform_1_i32(Some(&self.texture_uniform), 0);
        }

        gl.draw_arrays(glow::TRIANGLES, 0, vertices.len() as i32);

        gl.disable_vertex_attrib_array(0);
        gl.disable_vertex_attrib_array(1);
        gl.disable_vertex_attrib_array(2);
        gl.bind_buffer(glow::ARRAY_BUFFER, None);
    }

    unsafe fn set_scissor(&self, cmd: &DrawCommand, fb_w: i32, fb_h: i32) {
        if cmd.has_clip {
            let cx = cmd.clip_rect.x as i32;
            let cy = fb_h - (cmd.clip_rect.y + cmd.clip_rect.h) as i32;
            let cw = cmd.clip_rect.w as i32;
            let ch = cmd.clip_rect.h as i32;
            self.gl.scissor(cx.max(0), cy.max(0), cw.max(1), ch.max(1));
        } else {
            self.gl.scissor(0, 0, fb_w, fb_h);
        }
    }
}

impl RendererBackend for OpenGlRenderer {
    fn begin_frame(&mut self, metrics: &WindowMetrics, clear_state: &ClearState) {
        unsafe {
            let gl = &self.gl;
            gl.viewport(0, 0, metrics.framebuffer_w, metrics.framebuffer_h);
            if clear_state.clear_color {
                gl.clear_color(clear_state.r, clear_state.g, clear_state.b, clear_state.a);
                gl.clear(glow::COLOR_BUFFER_BIT);
            }
            gl.enable(glow::BLEND);
            gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);
            gl.enable(glow::SCISSOR_TEST);
            gl.scissor(0, 0, metrics.framebuffer_w, metrics.framebuffer_h);

            gl.use_program(Some(self.program));
            gl.uniform_2_f32(
                Some(&self.viewport_uniform),
                metrics.framebuffer_w as f32,
                metrics.framebuffer_h as f32,
            );
        }
    }

    fn render(&mut self, draw_data: &DrawDataView, metrics: &WindowMetrics) {
        unsafe {
            let fb_w = metrics.framebuffer_w;
            let fb_h = metrics.framebuffer_h;

            for cmd in draw_data.commands {
                self.set_scissor(cmd, fb_w, fb_h);

                match cmd.command_type {
                    CommandType::FilledRect => {
                        let mut verts = Vec::new();
                        push_rounded_quad(
                            &mut verts,
                            cmd.rect.x, cmd.rect.y, cmd.rect.w, cmd.rect.h,
                            cmd.color.r, cmd.color.g, cmd.color.b, cmd.color.a,
                            cmd.radius,
                        );
                        self.flush_vertices(&verts, TextureMode::None, None);
                    }
                    CommandType::RectOutline => {
                        let mut verts = Vec::new();
                        push_rounded_outline(
                            &mut verts,
                            cmd.rect.x, cmd.rect.y, cmd.rect.w, cmd.rect.h,
                            cmd.color.r, cmd.color.g, cmd.color.b, cmd.color.a,
                            cmd.radius, cmd.thickness,
                        );
                        self.flush_vertices(&verts, TextureMode::None, None);
                    }
                    CommandType::Text => {
                        let start = cmd.text_offset as usize;
                        let end = start + cmd.text_length as usize;
                        if end <= draw_data.text_arena.len() {
                            let text = std::str::from_utf8(&draw_data.text_arena[start..end]).unwrap_or("");
                            if !text.is_empty() && self.font_atlas.is_some() {
                                let gl_ref = Rc::clone(&self.gl);
                                let has_icon_font = self.icon_font_atlas.is_some();
                                let font_atlas = self.font_atlas.as_mut().unwrap();
                                let render_fs = font_atlas.render_font_size(cmd.font_size);
                                let icon_render_fs = if has_icon_font {
                                    self.icon_font_atlas.as_ref().unwrap().render_font_size(cmd.font_size)
                                } else {
                                    render_fs
                                };

                                // Pass 1: compute metrics and rasterize all glyphs.
                                // Collect per-glyph data to avoid holding mutable borrows during flush.
                                struct GlyphInfo {
                                    is_icon: bool,
                                    advance: f32,
                                    // Quad data (None if glyph has zero size)
                                    quad: Option<(f32, f32, f32, f32, f32, f32, f32, f32)>,
                                    // u0, v0, u1, v1
                                }
                                let mut max_above: f32 = 0.0;
                                let mut max_below: f32 = 0.0;
                                let mut total_width: f32 = 0.0;
                                let mut glyph_infos: Vec<GlyphInfo> = Vec::new();

                                for ch in text.chars() {
                                    let use_icon = has_icon_font && is_private_use_codepoint(ch as u32);
                                    let entry = if use_icon {
                                        let icon_atlas = self.icon_font_atlas.as_mut().unwrap();
                                        icon_atlas.get_or_rasterize(&gl_ref, ch, icon_render_fs)
                                    } else {
                                        font_atlas.get_or_rasterize(&gl_ref, ch, render_fs)
                                    };
                                    let above = (entry.offset_y + entry.height).max(0.0);
                                    let below = (-entry.offset_y).max(0.0);
                                    max_above = max_above.max(above);
                                    max_below = max_below.max(below);
                                    total_width += entry.advance_width;
                                    let quad = if entry.width > 0.0 && entry.height > 0.0 {
                                        Some((entry.offset_x, entry.offset_y, entry.width, entry.height,
                                              entry.u0, entry.v0, entry.u1, entry.v1))
                                    } else {
                                        None
                                    };
                                    glyph_infos.push(GlyphInfo {
                                        is_icon: use_icon,
                                        advance: entry.advance_width,
                                        quad,
                                    });
                                }

                                if max_above + max_below < 1.0 {
                                    if let Some(lm) = font_atlas.font.horizontal_line_metrics(render_fs) {
                                        max_above = lm.ascent;
                                        max_below = -lm.descent;
                                    } else {
                                        max_above = render_fs * 0.72;
                                        max_below = render_fs * 0.28;
                                    }
                                }

                                let text_h = (max_above + max_below).max(1.0);
                                let baseline_y = (cmd.rect.y + (cmd.rect.h - text_h).max(0.0) * 0.5 + max_above).round();
                                let start_x = match cmd.align {
                                    TextAlign::Left => cmd.rect.x,
                                    TextAlign::Center => cmd.rect.x + (cmd.rect.w - total_width) * 0.5,
                                    TextAlign::Right => cmd.rect.x + cmd.rect.w - total_width,
                                };

                                // Pass 2: build vertex batches and flush.
                                // No mutable atlas borrows needed — all glyphs already rasterized.
                                let text_tex = font_atlas.texture;
                                let icon_tex = self.icon_font_atlas.as_ref().map(|a| a.texture);

                                let mut verts = Vec::new();
                                let mut pen_x = start_x;
                                let mut current_is_icon = false;
                                for gi in &glyph_infos {
                                    if !verts.is_empty() && gi.is_icon != current_is_icon {
                                        let tex = if current_is_icon { icon_tex.unwrap() } else { text_tex };
                                        self.flush_vertices(&verts, TextureMode::AlphaMask, Some(tex));
                                        verts.clear();
                                    }
                                    current_is_icon = gi.is_icon;
                                    if let Some((ox, oy, w, h, u0, v0, u1, v1)) = gi.quad {
                                        let gx = (pen_x + ox).round();
                                        let gy = (baseline_y - oy - h).round();
                                        push_textured_quad(
                                            &mut verts,
                                            gx, gy, w, h,
                                            cmd.color.r, cmd.color.g, cmd.color.b, cmd.color.a,
                                            u0, v0, u1, v1,
                                        );
                                    }
                                    pen_x += gi.advance;
                                }
                                if !verts.is_empty() {
                                    let tex = if current_is_icon { icon_tex.unwrap() } else { text_tex };
                                    self.flush_vertices(&verts, TextureMode::AlphaMask, Some(tex));
                                }
                            }
                        }
                    }
                    CommandType::ImageRect => {
                        let start = cmd.text_offset as usize;
                        let end = start + cmd.text_length as usize;
                        if end <= draw_data.text_arena.len() {
                            let path = std::str::from_utf8(&draw_data.text_arena[start..end]).unwrap_or("");
                            let gl_ref = Rc::clone(&self.gl);
                            // Load texture, get its id
                            let tex_id = self.image_cache.get_or_load(&gl_ref, path).map(|c| c.texture);
                            if let Some(tex) = tex_id {
                                let mut verts = Vec::new();
                                push_textured_quad(
                                    &mut verts,
                                    cmd.rect.x, cmd.rect.y, cmd.rect.w, cmd.rect.h,
                                    1.0, 1.0, 1.0, 1.0,
                                    0.0, 0.0, 1.0, 1.0,
                                );
                                self.flush_vertices(&verts, TextureMode::Rgba, Some(tex));
                            }
                        }
                    }
                    CommandType::BackdropBlur => {
                        let mut verts = Vec::new();
                        push_quad(
                            &mut verts,
                            cmd.rect.x, cmd.rect.y, cmd.rect.w, cmd.rect.h,
                            0.0, 0.0, 0.0, 0.15,
                        );
                        self.flush_vertices(&verts, TextureMode::None, None);
                    }
                    CommandType::Chevron => {
                        let mut verts = Vec::new();
                        let cx = cmd.rect.x + cmd.rect.w * 0.5;
                        let cy = cmd.rect.y + cmd.rect.h * 0.5;
                        let sz = cmd.rect.w.min(cmd.rect.h) * 0.3;
                        let c = &cmd.color;
                        let t = cmd.thickness.max(0.5);
                        // Draw a V-shaped chevron pointing down (rotation=0),
                        // rotated by cmd.rotation (in radians)
                        let cos_r = cmd.rotation.cos();
                        let sin_r = cmd.rotation.sin();
                        let rotate = |dx: f32, dy: f32| -> (f32, f32) {
                            (cx + dx * cos_r - dy * sin_r, cy + dx * sin_r + dy * cos_r)
                        };
                        // Left arm: from top-left to center-bottom
                        let (x0, y0) = rotate(-sz * 0.5, -sz * 0.35);
                        let (x1, y1) = rotate(0.0, sz * 0.35);
                        push_quad(&mut verts, x0.min(x1), y0.min(y1), (x1 - x0).abs().max(t * 1.4), (y1 - y0).abs().max(t * 1.4), c.r, c.g, c.b, c.a);
                        // Right arm: from center-bottom to top-right
                        let (x2, y2) = rotate(sz * 0.5, -sz * 0.35);
                        push_quad(&mut verts, x1.min(x2), y1.min(y2), (x2 - x1).abs().max(t * 1.4), (y2 - y1).abs().max(t * 1.4), c.r, c.g, c.b, c.a);
                        self.flush_vertices(&verts, TextureMode::None, None);
                    }
                }
            }
        }
    }

    fn end_frame(&mut self) {
        unsafe {
            self.gl.disable(glow::SCISSOR_TEST);
            self.gl.disable(glow::BLEND);
        }
    }

    fn release_resources(&mut self) {
        unsafe {
            if let Some(ref atlas) = self.font_atlas {
                atlas.destroy(&self.gl);
            }
            if let Some(ref atlas) = self.icon_font_atlas {
                atlas.destroy(&self.gl);
            }
            self.image_cache.destroy(&self.gl);
            self.gl.delete_buffer(self.vbo);
            self.gl.delete_program(self.program);
        }
    }
}
