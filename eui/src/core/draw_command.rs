use crate::color::Color;
use crate::graphics::primitives::ImageFit;
use crate::rect::Rect;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum CommandType {
    #[default]
    FilledRect,
    RectOutline,
    BackdropBlur,
    Text,
    ImageRect,
    Chevron,
    Glyph,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextAlign {
    #[default]
    Left,
    Center,
    Right,
}

pub const K_INVALID_PAYLOAD_INDEX: u32 = u32::MAX;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DrawCommand {
    pub command_type: CommandType,
    pub rect: Rect,
    pub clip_rect: Rect,
    pub visible_rect: Rect,
    pub color: Color,
    pub payload_hash: u64,
    pub brush_payload_index: u32,
    pub transform_payload_index: u32,
    pub text_offset: u32,
    pub text_length: u32,
    pub font_size: f32,
    pub align: TextAlign,
    pub image_fit: ImageFit,
    pub radius: f32,
    pub thickness: f32,
    pub rotation: f32,
    pub blur_radius: f32,
    pub effect_alpha: f32,
    pub has_clip: bool,
    pub hash: u64,
}

impl Default for DrawCommand {
    fn default() -> Self {
        Self {
            command_type: CommandType::FilledRect,
            rect: Rect::default(),
            clip_rect: Rect::default(),
            visible_rect: Rect::default(),
            color: Color::default(),
            payload_hash: 0,
            brush_payload_index: K_INVALID_PAYLOAD_INDEX,
            transform_payload_index: K_INVALID_PAYLOAD_INDEX,
            text_offset: 0,
            text_length: 0,
            font_size: 13.0,
            align: TextAlign::Left,
            image_fit: ImageFit::Cover,
            radius: 0.0,
            thickness: 1.0,
            rotation: 0.0,
            blur_radius: 0.0,
            effect_alpha: 1.0,
            has_clip: false,
            hash: 0,
        }
    }
}
