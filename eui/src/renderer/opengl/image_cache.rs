use std::collections::HashMap;
use glow::HasContext;
use image::ImageReader;

pub struct CachedImage {
    pub texture: glow::Texture,
    pub width: u32,
    pub height: u32,
}

pub struct ImageCache {
    pub cache: HashMap<String, CachedImage>,
}

impl ImageCache {
    pub fn new() -> Self {
        Self { cache: HashMap::new() }
    }

    pub unsafe fn get_or_load(&mut self, gl: &glow::Context, path: &str) -> Option<&CachedImage> {
        if self.cache.contains_key(path) {
            return self.cache.get(path);
        }

        let reader = match ImageReader::open(path) {
            Ok(r) => r,
            Err(e) => { eprintln!("[IMAGE] open failed {:?}: {}", path, e); return None; }
        };
        let reader = match reader.with_guessed_format() {
            Ok(r) => r,
            Err(e) => { eprintln!("[IMAGE] format guess failed {:?}: {}", path, e); return None; }
        };
        let decoded = match reader.decode() {
            Ok(d) => d,
            Err(e) => { eprintln!("[IMAGE] decode failed {:?}: {}", path, e); return None; }
        };
        let img = decoded.to_rgba8();
        let (w, h) = img.dimensions();
        let data = img.into_raw();

        let texture = gl.create_texture().ok()?;
        gl.bind_texture(glow::TEXTURE_2D, Some(texture));
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::LINEAR as i32);
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, glow::LINEAR as i32);
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::CLAMP_TO_EDGE as i32);
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::CLAMP_TO_EDGE as i32);
        gl.tex_image_2d(
            glow::TEXTURE_2D, 0, glow::RGBA as i32,
            w as i32, h as i32, 0,
            glow::RGBA, glow::UNSIGNED_BYTE, Some(&data),
        );
        gl.bind_texture(glow::TEXTURE_2D, None);

        eprintln!("[IMAGE] loaded {:?} ({}x{}) tex={:?}", path, w, h, texture);
        self.cache.insert(path.to_string(), CachedImage { texture, width: w, height: h });
        self.cache.get(path)
    }

    pub unsafe fn destroy(&mut self, gl: &glow::Context) {
        for (_, entry) in self.cache.drain() {
            gl.delete_texture(entry.texture);
        }
    }
}
