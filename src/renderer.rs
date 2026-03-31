use egui::{Color32, Painter, Rect, Vec2, pos2};
use crate::fly::{Fly, FlyMode, FlyState};

const SPRITE_BASE_SIZE: f32 = 80.0; // display size at scale 1.0

pub fn draw_fly(fly: &Fly, painter: &Painter, frame: &egui::TextureHandle) {
    let pos = pos2(fly.x, fly.y);
    let size = SPRITE_BASE_SIZE * fly.scale;

    // Shadow when landed
    if fly.is_landed() {
        painter.circle_filled(
            pos2(pos.x + 2.0 * fly.scale, pos.y + (size / 2.0) + 4.0),
            10.0 * fly.scale,
            Color32::from_rgba_unmultiplied(0, 0, 0, 45),
        );
    }

    // Draw sprite centered on fly position
    let rect = Rect::from_center_size(pos, Vec2::splat(size));
    let uv = Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0));
    painter.image(frame.id(), rect, uv, Color32::WHITE);

    // Mode indicator dot above the sprite
    let indicator_color = match fly.mode {
        FlyMode::Companion => Color32::from_rgb(80, 200, 120),
        FlyMode::Chaos => Color32::from_rgb(220, 60, 60),
    };
    let dot_r = if matches!(fly.state, FlyState::Landed) && matches!(fly.mode, FlyMode::Chaos) {
        5.0
    } else {
        3.0
    } * fly.scale;
    painter.circle_filled(pos2(pos.x, pos.y - size / 2.0 - 6.0), dot_r, indicator_color);
}
