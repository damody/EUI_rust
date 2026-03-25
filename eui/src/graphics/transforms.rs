#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform2D {
    pub translation_x: f32,
    pub translation_y: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub rotation_deg: f32,
    pub origin_x: f32,
    pub origin_y: f32,
}

impl Default for Transform2D {
    fn default() -> Self {
        Self {
            translation_x: 0.0,
            translation_y: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            rotation_deg: 0.0,
            origin_x: 0.0,
            origin_y: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform3D {
    pub translation_x: f32,
    pub translation_y: f32,
    pub translation_z: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub scale_z: f32,
    pub rotation_x_deg: f32,
    pub rotation_y_deg: f32,
    pub rotation_z_deg: f32,
    pub origin_x: f32,
    pub origin_y: f32,
    pub origin_z: f32,
    pub perspective: f32,
}

impl Default for Transform3D {
    fn default() -> Self {
        Self {
            translation_x: 0.0,
            translation_y: 0.0,
            translation_z: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            scale_z: 1.0,
            rotation_x_deg: 0.0,
            rotation_y_deg: 0.0,
            rotation_z_deg: 0.0,
            origin_x: 0.0,
            origin_y: 0.0,
            origin_z: 0.0,
            perspective: 0.0,
        }
    }
}
