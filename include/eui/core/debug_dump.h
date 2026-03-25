#pragma once

#include <cmath>
#include <cstddef>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

#include "eui/core/foundation.h"
#include "eui/graphics/effects.h"
#include "eui/graphics/primitives.h"
#include "eui/graphics/transforms.h"

namespace eui::debug {

namespace detail {

inline float r2(float v) {
    return std::round(v * 100.0f) / 100.0f;
}

inline void write_f32(std::string& out, float v) {
    float r = r2(v);
    // Format: if integer, always show one decimal place
    if (r == std::floor(r) && std::fabs(r) < 1e15f) {
        char buf[64];
        std::snprintf(buf, sizeof(buf), "%.1f", static_cast<double>(r));
        out.append(buf);
    } else {
        char buf[64];
        std::snprintf(buf, sizeof(buf), "%g", static_cast<double>(r));
        out.append(buf);
    }
}

inline void write_f32_array(std::string& out, const float* vals, std::size_t count) {
    out.push_back('[');
    for (std::size_t i = 0; i < count; ++i) {
        if (i > 0) out.append(", ");
        write_f32(out, vals[i]);
    }
    out.push_back(']');
}

inline void write_json_string(std::string& out, const char* s, std::size_t len) {
    out.push_back('"');
    for (std::size_t i = 0; i < len; ++i) {
        char ch = s[i];
        switch (ch) {
            case '"':  out.append("\\\""); break;
            case '\\': out.append("\\\\"); break;
            case '\n': out.append("\\n"); break;
            case '\r': out.append("\\r"); break;
            case '\t': out.append("\\t"); break;
            default:
                if (static_cast<unsigned char>(ch) < 0x20) {
                    char buf[8];
                    std::snprintf(buf, sizeof(buf), "\\u%04x", static_cast<unsigned>(static_cast<unsigned char>(ch)));
                    out.append(buf);
                } else {
                    out.push_back(ch);
                }
                break;
        }
    }
    out.push_back('"');
}

inline const char* command_type_str(CommandType ct) {
    switch (ct) {
        case CommandType::FilledRect:   return "FilledRect";
        case CommandType::RectOutline:  return "RectOutline";
        case CommandType::BackdropBlur: return "BackdropBlur";
        case CommandType::Text:         return "Text";
        case CommandType::ImageRect:    return "ImageRect";
        case CommandType::Chevron:      return "Chevron";
        default:                        return "Unknown";
    }
}

inline const char* text_align_str(TextAlign a) {
    switch (a) {
        case TextAlign::Left:   return "Left";
        case TextAlign::Center: return "Center";
        case TextAlign::Right:  return "Right";
        default:                return "Left";
    }
}

inline const char* image_fit_str(eui::graphics::ImageFit f) {
    switch (f) {
        case eui::graphics::ImageFit::fill:    return "Fill";
        case eui::graphics::ImageFit::contain: return "Contain";
        case eui::graphics::ImageFit::cover:   return "Cover";
        case eui::graphics::ImageFit::stretch: return "Stretch";
        case eui::graphics::ImageFit::center:  return "Center";
        default:                               return "Cover";
    }
}

inline void write_brush(std::string& out, const eui::graphics::Brush& brush) {
    switch (brush.kind) {
        case eui::graphics::BrushKind::none:
            out.append("null");
            break;
        case eui::graphics::BrushKind::solid: {
            out.append("{ \"kind\": \"Solid\", \"color\": ");
            float c[4] = {brush.solid.r, brush.solid.g, brush.solid.b, brush.solid.a};
            write_f32_array(out, c, 4);
            out.append(" }");
            break;
        }
        case eui::graphics::BrushKind::linear_gradient: {
            const auto& lg = brush.linear;
            out.append("{ \"kind\": \"LinearGradient\", \"start\": ");
            float start[2] = {lg.start.x, lg.start.y};
            write_f32_array(out, start, 2);
            out.append(", \"end\": ");
            float end[2] = {lg.end.x, lg.end.y};
            write_f32_array(out, end, 2);
            out.append(", \"stops\": [");
            for (std::size_t i = 0; i < lg.stop_count; ++i) {
                if (i > 0) out.append(", ");
                out.append("{ \"pos\": ");
                write_f32(out, lg.stops[i].position);
                out.append(", \"color\": ");
                float sc[4] = {lg.stops[i].color.r, lg.stops[i].color.g, lg.stops[i].color.b, lg.stops[i].color.a};
                write_f32_array(out, sc, 4);
                out.append(" }");
            }
            out.append("] }");
            break;
        }
        case eui::graphics::BrushKind::radial_gradient: {
            const auto& rg = brush.radial;
            out.append("{ \"kind\": \"RadialGradient\", \"center\": ");
            float center[2] = {rg.center.x, rg.center.y};
            write_f32_array(out, center, 2);
            out.append(", \"radius\": ");
            write_f32(out, rg.radius);
            out.append(", \"stops\": [");
            for (std::size_t i = 0; i < rg.stop_count; ++i) {
                if (i > 0) out.append(", ");
                out.append("{ \"pos\": ");
                write_f32(out, rg.stops[i].position);
                out.append(", \"color\": ");
                float sc[4] = {rg.stops[i].color.r, rg.stops[i].color.g, rg.stops[i].color.b, rg.stops[i].color.a};
                write_f32_array(out, sc, 4);
                out.append(" }");
            }
            out.append("] }");
            break;
        }
    }
}

inline void write_transform(std::string& out, const eui::graphics::Transform3D& t) {
    out.append("{ \"tx\": "); write_f32(out, t.translation_x);
    out.append(", \"ty\": "); write_f32(out, t.translation_y);
    out.append(", \"tz\": "); write_f32(out, t.translation_z);
    out.append(", \"sx\": "); write_f32(out, t.scale_x);
    out.append(", \"sy\": "); write_f32(out, t.scale_y);
    out.append(", \"sz\": "); write_f32(out, t.scale_z);
    out.append(", \"rx\": "); write_f32(out, t.rotation_x_deg);
    out.append(", \"ry\": "); write_f32(out, t.rotation_y_deg);
    out.append(", \"rz\": "); write_f32(out, t.rotation_z_deg);
    out.append(", \"ox\": "); write_f32(out, t.origin_x);
    out.append(", \"oy\": "); write_f32(out, t.origin_y);
    out.append(", \"oz\": "); write_f32(out, t.origin_z);
    out.append(", \"perspective\": "); write_f32(out, t.perspective);
    out.append(" }");
}

}  // namespace detail

inline void dump_commands_json(
    const std::vector<DrawCommand>& commands,
    const std::vector<char>& text_arena,
    const std::vector<eui::graphics::Brush>& brush_payloads,
    const std::vector<eui::graphics::Transform3D>& transform_payloads,
    const std::string& path)
{
    std::string out;
    out.reserve(commands.size() * 512);
    out.append("{\n");
    out.append("  \"frame_command_count\": ");
    out.append(std::to_string(commands.size()));
    out.append(",\n");
    out.append("  \"commands\": [\n");

    for (std::size_t i = 0; i < commands.size(); ++i) {
        const auto& cmd = commands[i];
        out.append("    {\n");

        // index, type
        out.append("      \"index\": ");
        out.append(std::to_string(i));
        out.append(",\n");
        out.append("      \"type\": \"");
        out.append(detail::command_type_str(cmd.type));
        out.append("\",\n");

        // rect
        out.append("      \"rect\": ");
        float rect[4] = {cmd.rect.x, cmd.rect.y, cmd.rect.w, cmd.rect.h};
        detail::write_f32_array(out, rect, 4);
        out.append(",\n");

        // clip_rect
        out.append("      \"clip_rect\": ");
        float clip[4] = {cmd.clip_rect.x, cmd.clip_rect.y, cmd.clip_rect.w, cmd.clip_rect.h};
        detail::write_f32_array(out, clip, 4);
        out.append(",\n");

        // visible_rect
        out.append("      \"visible_rect\": ");
        float vis[4] = {cmd.visible_rect.x, cmd.visible_rect.y, cmd.visible_rect.w, cmd.visible_rect.h};
        detail::write_f32_array(out, vis, 4);
        out.append(",\n");

        // color
        out.append("      \"color\": ");
        float color[4] = {cmd.color.r, cmd.color.g, cmd.color.b, cmd.color.a};
        detail::write_f32_array(out, color, 4);
        out.append(",\n");

        // scalar fields
        out.append("      \"radius\": "); detail::write_f32(out, cmd.radius); out.append(",\n");
        out.append("      \"thickness\": "); detail::write_f32(out, cmd.thickness); out.append(",\n");
        out.append("      \"rotation\": "); detail::write_f32(out, cmd.rotation); out.append(",\n");
        out.append("      \"blur_radius\": "); detail::write_f32(out, cmd.blur_radius); out.append(",\n");
        out.append("      \"effect_alpha\": "); detail::write_f32(out, cmd.effect_alpha); out.append(",\n");

        // has_clip
        out.append("      \"has_clip\": ");
        out.append(cmd.has_clip ? "true" : "false");
        out.append(",\n");

        // text — inline from text_arena
        out.append("      \"text\": ");
        if (cmd.text_length > 0) {
            std::size_t start = static_cast<std::size_t>(cmd.text_offset);
            std::size_t end = start + static_cast<std::size_t>(cmd.text_length);
            if (end <= text_arena.size()) {
                detail::write_json_string(out, text_arena.data() + start, cmd.text_length);
            } else {
                out.append("\"\"");
            }
        } else {
            out.append("\"\"");
        }
        out.append(",\n");

        // font_size, align, image_fit
        out.append("      \"font_size\": "); detail::write_f32(out, cmd.font_size); out.append(",\n");
        out.append("      \"align\": \""); out.append(detail::text_align_str(cmd.align)); out.append("\",\n");
        out.append("      \"image_fit\": \""); out.append(detail::image_fit_str(cmd.image_fit)); out.append("\",\n");

        // brush — inline payload
        out.append("      \"brush\": ");
        if (cmd.brush_payload_index != DrawCommand::k_invalid_payload_index &&
            cmd.brush_payload_index < static_cast<std::uint32_t>(brush_payloads.size())) {
            detail::write_brush(out, brush_payloads[cmd.brush_payload_index]);
        } else {
            out.append("null");
        }
        out.append(",\n");

        // transform — inline payload
        out.append("      \"transform\": ");
        if (cmd.transform_payload_index != DrawCommand::k_invalid_payload_index &&
            cmd.transform_payload_index < static_cast<std::uint32_t>(transform_payloads.size())) {
            detail::write_transform(out, transform_payloads[cmd.transform_payload_index]);
        } else {
            out.append("null");
        }
        out.append("\n");

        // close object
        out.append("    }");
        if (i + 1 < commands.size()) {
            out.push_back(',');
        }
        out.push_back('\n');
    }

    out.append("  ]\n");
    out.append("}\n");

    std::ofstream file(path, std::ios::out | std::ios::binary);
    if (file.is_open()) {
        file.write(out.data(), static_cast<std::streamsize>(out.size()));
        file.close();
        std::cerr << "[eui] Dumped " << commands.size() << " commands to " << path << std::endl;
    } else {
        std::cerr << "[eui] Failed to create dump file " << path << std::endl;
    }
}

}  // namespace eui::debug
