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
    image_cache: ImageCache,
    vertices: Vec<Vertex>,
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
            image_cache: ImageCache::new(),
            vertices: Vec::with_capacity(4096),
        })
    }

    pub unsafe fn set_font(&mut self, font: fontdue::Font) {
        if let Some(ref atlas) = self.font_atlas {
            atlas.destroy(&self.gl);
        }
        self.font_atlas = Some(FontAtlas::new(&self.gl, font));
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
                        self.vertices.clear();
                        push_quad(
                            &mut self.vertices,
                            cmd.rect.x, cmd.rect.y, cmd.rect.w, cmd.rect.h,
                            cmd.color.r, cmd.color.g, cmd.color.b, cmd.color.a,
                        );
                        self.flush_vertices(&self.vertices.clone(), TextureMode::None, None);
                        self.vertices.clear();
                    }
                    CommandType::RectOutline => {
                        self.vertices.clear();
                        let t = cmd.thickness;
                        let r = &cmd.rect;
                        let c = &cmd.color;
                        push_quad(&mut self.vertices, r.x, r.y, r.w, t, c.r, c.g, c.b, c.a);
                        push_quad(&mut self.vertices, r.x, r.y + r.h - t, r.w, t, c.r, c.g, c.b, c.a);
                        push_quad(&mut self.vertices, r.x, r.y + t, t, r.h - t * 2.0, c.r, c.g, c.b, c.a);
                        push_quad(&mut self.vertices, r.x + r.w - t, r.y + t, t, r.h - t * 2.0, c.r, c.g, c.b, c.a);
                        self.flush_vertices(&self.vertices.clone(), TextureMode::None, None);
                        self.vertices.clear();
                    }
                    CommandType::Text => {
                        let start = cmd.text_offset as usize;
                        let end = start + cmd.text_length as usize;
                        if end <= draw_data.text_arena.len() {
                            let text = std::str::from_utf8(&draw_data.text_arena[start..end]).unwrap_or("");
                            if !text.is_empty() {
                                if let Some(ref mut font_atlas) = self.font_atlas {
                                    let gl_ref = Rc::clone(&self.gl);
                                    let font_size = cmd.font_size;
                                    let line_metrics = font_atlas.font.horizontal_line_metrics(font_size);
                                    let ascent = line_metrics.map(|m| m.ascent).unwrap_or(font_size * 0.8);
                                    let total_width: f32 = text.chars().map(|ch| font_atlas.font.metrics(ch, font_size).advance_width).sum();
                                    let start_x = match cmd.align {
                                        TextAlign::Left => cmd.rect.x,
                                        TextAlign::Center => cmd.rect.x + (cmd.rect.w - total_width) * 0.5,
                                        TextAlign::Right => cmd.rect.x + cmd.rect.w - total_width,
                                    };
                                    let baseline_y = cmd.rect.y + (cmd.rect.h - font_size) * 0.5 + ascent;

                                    let mut verts = Vec::new();
                                    let mut pen_x = start_x;
                                    for ch in text.chars() {
                                        let entry = font_atlas.get_or_rasterize(&gl_ref, ch, font_size);
                                        if entry.width > 0.0 && entry.height > 0.0 {
                                            let gx = pen_x + entry.offset_x;
                                            let gy = baseline_y - entry.offset_y - entry.height;
                                            push_textured_quad(
                                                &mut verts,
                                                gx, gy, entry.width, entry.height,
                                                cmd.color.r, cmd.color.g, cmd.color.b, cmd.color.a,
                                                entry.u0, entry.v0, entry.u1, entry.v1,
                                            );
                                        }
                                        pen_x += entry.advance_width;
                                    }
                                    let tex = font_atlas.texture;
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
                        let t = 1.5;
                        push_quad(&mut verts, cx - sz * 0.5, cy - sz, t * 1.4, sz, c.r, c.g, c.b, c.a);
                        push_quad(&mut verts, cx - sz * 0.5, cy, t * 1.4, sz, c.r, c.g, c.b, c.a);
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
            self.image_cache.destroy(&self.gl);
            self.gl.delete_buffer(self.vbo);
            self.gl.delete_program(self.program);
        }
    }
}
