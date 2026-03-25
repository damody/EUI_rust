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
