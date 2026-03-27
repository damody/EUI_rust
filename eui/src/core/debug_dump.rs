use std::fmt::Write as FmtWrite;
use std::io::Write as IoWrite;
use std::path::Path;

use crate::core::draw_command::{CommandType, DrawCommand, TextAlign, K_INVALID_PAYLOAD_INDEX};
use crate::graphics::effects::{Brush, BrushKind};
use crate::graphics::primitives::ImageFit;
use crate::graphics::transforms::Transform3D;

fn r2(v: f32) -> f32 {
    (v * 100.0).round() / 100.0
}

fn write_f32(out: &mut String, v: f32) {
    let r = r2(v);
    if r == r.floor() && r.abs() < 1e15 {
        write!(out, "{:.1}", r).unwrap();
    } else {
        write!(out, "{}", r).unwrap();
    }
}

fn write_f32_array(out: &mut String, vals: &[f32]) {
    out.push('[');
    for (i, v) in vals.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        write_f32(out, *v);
    }
    out.push(']');
}

fn write_json_string(out: &mut String, s: &str) {
    out.push('"');
    for ch in s.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                write!(out, "\\u{:04x}", c as u32).unwrap();
            }
            c => out.push(c),
        }
    }
    out.push('"');
}

fn command_type_str(ct: CommandType) -> &'static str {
    match ct {
        CommandType::FilledRect => "FilledRect",
        CommandType::RectOutline => "RectOutline",
        CommandType::BackdropBlur => "BackdropBlur",
        CommandType::Text => "Text",
        CommandType::ImageRect => "ImageRect",
        CommandType::Chevron => "Chevron",
        CommandType::Line => "Line",
    }
}

fn text_align_str(a: TextAlign) -> &'static str {
    match a {
        TextAlign::Left => "Left",
        TextAlign::Center => "Center",
        TextAlign::Right => "Right",
    }
}

fn image_fit_str(f: ImageFit) -> &'static str {
    match f {
        ImageFit::Fill => "Fill",
        ImageFit::Contain => "Contain",
        ImageFit::Cover => "Cover",
        ImageFit::Stretch => "Stretch",
        ImageFit::Center => "Center",
    }
}

fn write_brush(out: &mut String, brush: &Brush) {
    match brush.kind {
        BrushKind::None => {
            out.push_str("null");
        }
        BrushKind::Solid => {
            out.push_str("{ \"kind\": \"Solid\", \"color\": ");
            let c = &brush.solid;
            write_f32_array(out, &[c.r, c.g, c.b, c.a]);
            out.push_str(" }");
        }
        BrushKind::LinearGradient => {
            let lg = &brush.linear;
            out.push_str("{ \"kind\": \"LinearGradient\", \"start\": ");
            write_f32_array(out, &[lg.start.x, lg.start.y]);
            out.push_str(", \"end\": ");
            write_f32_array(out, &[lg.end.x, lg.end.y]);
            out.push_str(", \"stops\": [");
            for i in 0..lg.stop_count {
                if i > 0 {
                    out.push_str(", ");
                }
                let s = &lg.stops[i];
                out.push_str("{ \"pos\": ");
                write_f32(out, s.position);
                out.push_str(", \"color\": ");
                write_f32_array(out, &[s.color.r, s.color.g, s.color.b, s.color.a]);
                out.push_str(" }");
            }
            out.push_str("] }");
        }
        BrushKind::RadialGradient => {
            let rg = &brush.radial;
            out.push_str("{ \"kind\": \"RadialGradient\", \"center\": ");
            write_f32_array(out, &[rg.center.x, rg.center.y]);
            out.push_str(", \"radius\": ");
            write_f32(out, rg.radius);
            out.push_str(", \"stops\": [");
            for i in 0..rg.stop_count {
                if i > 0 {
                    out.push_str(", ");
                }
                let s = &rg.stops[i];
                out.push_str("{ \"pos\": ");
                write_f32(out, s.position);
                out.push_str(", \"color\": ");
                write_f32_array(out, &[s.color.r, s.color.g, s.color.b, s.color.a]);
                out.push_str(" }");
            }
            out.push_str("] }");
        }
    }
}

fn write_transform(out: &mut String, t: &Transform3D) {
    write!(
        out,
        "{{ \"tx\": {}, \"ty\": {}, \"tz\": {}, \"sx\": {}, \"sy\": {}, \"sz\": {}, \
         \"rx\": {}, \"ry\": {}, \"rz\": {}, \"ox\": {}, \"oy\": {}, \"oz\": {}, \
         \"perspective\": {} }}",
        r2(t.translation_x), r2(t.translation_y), r2(t.translation_z),
        r2(t.scale_x), r2(t.scale_y), r2(t.scale_z),
        r2(t.rotation_x_deg), r2(t.rotation_y_deg), r2(t.rotation_z_deg),
        r2(t.origin_x), r2(t.origin_y), r2(t.origin_z),
        r2(t.perspective),
    ).unwrap();
}

pub fn dump_commands_json(
    commands: &[DrawCommand],
    text_arena: &[u8],
    brush_payloads: &[Brush],
    transform_payloads: &[Transform3D],
    path: &Path,
) {
    let mut out = String::with_capacity(commands.len() * 512);
    writeln!(out, "{{").unwrap();
    writeln!(out, "  \"frame_command_count\": {},", commands.len()).unwrap();
    writeln!(out, "  \"commands\": [").unwrap();

    for (i, cmd) in commands.iter().enumerate() {
        out.push_str("    {\n");

        // index, type
        writeln!(out, "      \"index\": {},", i).unwrap();
        writeln!(out, "      \"type\": \"{}\",", command_type_str(cmd.command_type)).unwrap();

        // rect
        out.push_str("      \"rect\": ");
        write_f32_array(&mut out, &[cmd.rect.x, cmd.rect.y, cmd.rect.w, cmd.rect.h]);
        out.push_str(",\n");

        // clip_rect
        out.push_str("      \"clip_rect\": ");
        write_f32_array(&mut out, &[cmd.clip_rect.x, cmd.clip_rect.y, cmd.clip_rect.w, cmd.clip_rect.h]);
        out.push_str(",\n");

        // visible_rect
        out.push_str("      \"visible_rect\": ");
        write_f32_array(&mut out, &[cmd.visible_rect.x, cmd.visible_rect.y, cmd.visible_rect.w, cmd.visible_rect.h]);
        out.push_str(",\n");

        // color
        out.push_str("      \"color\": ");
        write_f32_array(&mut out, &[cmd.color.r, cmd.color.g, cmd.color.b, cmd.color.a]);
        out.push_str(",\n");

        // scalar fields
        out.push_str("      \"radius\": "); write_f32(&mut out, cmd.radius); out.push_str(",\n");
        out.push_str("      \"thickness\": "); write_f32(&mut out, cmd.thickness); out.push_str(",\n");
        out.push_str("      \"rotation\": "); write_f32(&mut out, cmd.rotation); out.push_str(",\n");
        out.push_str("      \"blur_radius\": "); write_f32(&mut out, cmd.blur_radius); out.push_str(",\n");
        out.push_str("      \"effect_alpha\": "); write_f32(&mut out, cmd.effect_alpha); out.push_str(",\n");

        // has_clip
        writeln!(out, "      \"has_clip\": {},", cmd.has_clip).unwrap();

        // text — inline from text_arena
        let text = if cmd.text_length > 0 {
            let start = cmd.text_offset as usize;
            let end = start + cmd.text_length as usize;
            if end <= text_arena.len() {
                std::str::from_utf8(&text_arena[start..end]).unwrap_or("")
            } else {
                ""
            }
        } else {
            ""
        };
        out.push_str("      \"text\": ");
        write_json_string(&mut out, text);
        out.push_str(",\n");

        // font_size, align, image_fit
        out.push_str("      \"font_size\": "); write_f32(&mut out, cmd.font_size); out.push_str(",\n");
        writeln!(out, "      \"align\": \"{}\",", text_align_str(cmd.align)).unwrap();
        writeln!(out, "      \"image_fit\": \"{}\",", image_fit_str(cmd.image_fit)).unwrap();

        // brush — inline payload
        out.push_str("      \"brush\": ");
        if cmd.brush_payload_index != K_INVALID_PAYLOAD_INDEX {
            let idx = cmd.brush_payload_index as usize;
            if idx < brush_payloads.len() {
                write_brush(&mut out, &brush_payloads[idx]);
            } else {
                out.push_str("null");
            }
        } else {
            out.push_str("null");
        }
        out.push_str(",\n");

        // transform — inline payload
        out.push_str("      \"transform\": ");
        if cmd.transform_payload_index != K_INVALID_PAYLOAD_INDEX {
            let idx = cmd.transform_payload_index as usize;
            if idx < transform_payloads.len() {
                write_transform(&mut out, &transform_payloads[idx]);
            } else {
                out.push_str("null");
            }
        } else {
            out.push_str("null");
        }
        out.push('\n');

        // close object
        out.push_str("    }");
        if i + 1 < commands.len() {
            out.push(',');
        }
        out.push('\n');
    }

    writeln!(out, "  ]").unwrap();
    writeln!(out, "}}").unwrap();

    match std::fs::File::create(path) {
        Ok(mut f) => {
            if let Err(e) = f.write_all(out.as_bytes()) {
                eprintln!("[eui] Failed to write dump file: {}", e);
            } else {
                eprintln!("[eui] Dumped {} commands to {}", commands.len(), path.display());
            }
        }
        Err(e) => {
            eprintln!("[eui] Failed to create dump file {}: {}", path.display(), e);
        }
    }
}
