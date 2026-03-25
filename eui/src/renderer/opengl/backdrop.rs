// Backdrop blur placeholder
// Full implementation would use framebuffer readback + Gaussian blur
// For now this is a stub that renders a semi-transparent overlay

pub struct BackdropBlurState {
    // Reserved for future FBO-based blur implementation
}

impl BackdropBlurState {
    pub fn new() -> Self {
        Self {}
    }
}
