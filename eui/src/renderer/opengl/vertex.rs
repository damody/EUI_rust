#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
    pub u: f32,
    pub v: f32,
}

impl Vertex {
    pub fn new(x: f32, y: f32, r: f32, g: f32, b: f32, a: f32, u: f32, v: f32) -> Self {
        Self { x, y, r, g, b, a, u, v }
    }

    pub fn colored(x: f32, y: f32, r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { x, y, r, g, b, a, u: 0.0, v: 0.0 }
    }
}

pub const VERTEX_SIZE: usize = std::mem::size_of::<Vertex>();
pub const VERTICES_PER_QUAD: usize = 6;

pub fn push_quad(vertices: &mut Vec<Vertex>, x: f32, y: f32, w: f32, h: f32, r: f32, g: f32, b: f32, a: f32) {
    let v0 = Vertex::colored(x, y, r, g, b, a);
    let v1 = Vertex::colored(x + w, y, r, g, b, a);
    let v2 = Vertex::colored(x + w, y + h, r, g, b, a);
    let v3 = Vertex::colored(x, y + h, r, g, b, a);
    vertices.extend_from_slice(&[v0, v1, v2, v0, v2, v3]);
}

pub fn push_textured_quad(vertices: &mut Vec<Vertex>, x: f32, y: f32, w: f32, h: f32,
                          r: f32, g: f32, b: f32, a: f32,
                          u0: f32, v0_coord: f32, u1: f32, v1_coord: f32) {
    let va = Vertex::new(x, y, r, g, b, a, u0, v0_coord);
    let vb = Vertex::new(x + w, y, r, g, b, a, u1, v0_coord);
    let vc = Vertex::new(x + w, y + h, r, g, b, a, u1, v1_coord);
    let vd = Vertex::new(x, y + h, r, g, b, a, u0, v1_coord);
    vertices.extend_from_slice(&[va, vb, vc, va, vc, vd]);
}

/// Push a filled rounded rectangle as triangles.
/// When `radius <= 0` this falls back to a plain quad.
/// Arc segment count is dynamic matching C++: `clamp(radius * 0.65, 3, 10)`.
pub fn push_rounded_quad(vertices: &mut Vec<Vertex>, x: f32, y: f32, w: f32, h: f32,
                         r: f32, g: f32, b: f32, a: f32, radius: f32) {
    let rad = radius.clamp(0.0, (w.min(h)) * 0.5);
    if rad <= 0.0 {
        push_quad(vertices, x, y, w, h, r, g, b, a);
        return;
    }

    // Dynamic segment count matching C++ build_rounded_points
    let steps = ((rad * 0.65) as usize).clamp(3, 10);

    // Center of the rectangle
    let cx = x + w * 0.5;
    let cy = y + h * 0.5;
    let center = Vertex::colored(cx, cy, r, g, b, a);

    let left = x;
    let right = x + w;
    let top = y;
    let bottom = y + h;
    let pi = std::f32::consts::PI;

    // Build perimeter points matching C++ build_rounded_points exactly:
    // 4 arcs, first arc includes first point, subsequent arcs skip first point to avoid duplicates
    let mut perimeter: Vec<Vertex> = Vec::with_capacity(steps * 4 + 4);
    let arcs: [(f32, f32, f32, f32, bool); 4] = [
        (left + rad,  top + rad,    pi,       1.5 * pi, true),   // top-left
        (right - rad, top + rad,    1.5 * pi, 2.0 * pi, false),  // top-right
        (right - rad, bottom - rad, 0.0,      0.5 * pi, false),  // bottom-right
        (left + rad,  bottom - rad, 0.5 * pi, pi,       false),  // bottom-left
    ];

    for &(ccx, ccy, start, end, include_first) in &arcs {
        for i in 0..=steps {
            if i == 0 && !include_first {
                continue;
            }
            let t = i as f32 / steps as f32;
            let angle = start + (end - start) * t;
            let px = ccx + rad * angle.cos();
            let py = ccy + rad * angle.sin();
            perimeter.push(Vertex::colored(px, py, r, g, b, a));
        }
    }

    // Triangle fan from center to perimeter
    let n = perimeter.len();
    for i in 0..n {
        let j = (i + 1) % n;
        vertices.push(center);
        vertices.push(perimeter[i]);
        vertices.push(perimeter[j]);
    }
}

/// Push a rounded rectangle outline as triangles.
/// When `radius <= 0` this falls back to plain quads for the edges.
/// Arc segment count is dynamic matching C++: `clamp(radius * 0.65, 3, 10)`.
pub fn push_rounded_outline(vertices: &mut Vec<Vertex>, x: f32, y: f32, w: f32, h: f32,
                            r: f32, g: f32, b: f32, a: f32, radius: f32, thickness: f32) {
    let rad = radius.clamp(0.0, (w.min(h)) * 0.5);
    if rad <= 0.0 {
        // Fallback: 4 edge quads
        let t = thickness;
        push_quad(vertices, x, y, w, t, r, g, b, a);
        push_quad(vertices, x, y + h - t, w, t, r, g, b, a);
        push_quad(vertices, x, y + t, t, h - t * 2.0, r, g, b, a);
        push_quad(vertices, x + w - t, y + t, t, h - t * 2.0, r, g, b, a);
        return;
    }

    // Dynamic segment count matching C++
    let steps = ((rad * 0.65) as usize).clamp(3, 10);

    let left = x;
    let right = x + w;
    let top = y;
    let bottom = y + h;
    let pi = std::f32::consts::PI;

    let arcs: [(f32, f32, f32, f32, bool); 4] = [
        (left + rad,  top + rad,    pi,       1.5 * pi, true),
        (right - rad, top + rad,    1.5 * pi, 2.0 * pi, false),
        (right - rad, bottom - rad, 0.0,      0.5 * pi, false),
        (left + rad,  bottom - rad, 0.5 * pi, pi,       false),
    ];

    // Build inner and outer perimeter rings
    let mut outer: Vec<(f32, f32)> = Vec::with_capacity(steps * 4 + 4);
    let mut inner: Vec<(f32, f32)> = Vec::with_capacity(steps * 4 + 4);
    let half_t = thickness * 0.5;

    for &(ccx, ccy, start, end, include_first) in &arcs {
        for i in 0..=steps {
            if i == 0 && !include_first {
                continue;
            }
            let t = i as f32 / steps as f32;
            let angle = start + (end - start) * t;
            let cos_a = angle.cos();
            let sin_a = angle.sin();
            outer.push((ccx + (rad + half_t) * cos_a, ccy + (rad + half_t) * sin_a));
            inner.push((ccx + (rad - half_t).max(0.0) * cos_a, ccy + (rad - half_t).max(0.0) * sin_a));
        }
    }

    // Generate quad strip between inner and outer rings
    let n = outer.len();
    for i in 0..n {
        let j = (i + 1) % n;
        let o0 = Vertex::colored(outer[i].0, outer[i].1, r, g, b, a);
        let i0 = Vertex::colored(inner[i].0, inner[i].1, r, g, b, a);
        let o1 = Vertex::colored(outer[j].0, outer[j].1, r, g, b, a);
        let i1 = Vertex::colored(inner[j].0, inner[j].1, r, g, b, a);
        vertices.extend_from_slice(&[o0, o1, i0, i0, o1, i1]);
    }
}
