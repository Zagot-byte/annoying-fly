use egui::{Color32, Painter, Pos2, pos2};
use crate::fly::{Fly, FlyMode, FlyState};

pub fn draw_fly(fly: &Fly, painter: &Painter) {
    let pos = pos2(fly.x, fly.y);
    let s = fly.scale;
    let landed = fly.is_landed();

    draw_shadow(painter, pos, s, landed);
    draw_wings(painter, pos, s, fly.wing_phase, landed);
    draw_body(painter, pos, s);
    draw_eyes(painter, pos, s);
    draw_mode_indicator(painter, pos, s, &fly.mode, &fly.state);
}

fn draw_shadow(painter: &Painter, pos: Pos2, s: f32, landed: bool) {
    if landed {
        painter.circle_filled(
            pos2(pos.x + 2.0 * s, pos.y + 14.0 * s),
            10.0 * s,
            Color32::from_rgba_unmultiplied(0, 0, 0, 50),
        );
    }
}

fn draw_wings(painter: &Painter, pos: Pos2, s: f32, phase: f32, landed: bool) {
    let (wing_r, y_off) = if landed {
        (8.0 * s, 2.0 * s)
    } else {
        let flap = (phase.sin() * 8.0 * s).abs();
        (13.0 * s, flap + 4.0 * s)
    };

    let wing_color = Color32::from_rgba_unmultiplied(200, 210, 230, 185);
    painter.circle_filled(pos2(pos.x - 16.0 * s, pos.y - y_off), wing_r, wing_color);
    painter.circle_filled(pos2(pos.x + 16.0 * s, pos.y - y_off), wing_r, wing_color);
}

fn draw_body(painter: &Painter, pos: Pos2, s: f32) {
    painter.circle_filled(pos2(pos.x, pos.y + 5.0 * s), 9.0 * s,  Color32::from_rgb(20, 20, 20));
    painter.circle_filled(pos2(pos.x, pos.y - 3.0 * s), 7.0 * s,  Color32::from_rgb(35, 35, 35));
    painter.circle_filled(pos2(pos.x, pos.y - 11.0 * s), 5.5 * s, Color32::from_rgb(25, 25, 25));
}

fn draw_eyes(painter: &Painter, pos: Pos2, s: f32) {
    for sign in [-1.0_f32, 1.0] {
        let eye = pos2(pos.x + sign * 4.0 * s, pos.y - 12.5 * s);
        painter.circle_filled(eye, 2.5 * s, Color32::from_rgb(180, 50, 50));
        painter.circle_filled(eye, 1.0 * s, Color32::BLACK);
    }
}

fn draw_mode_indicator(painter: &Painter, pos: Pos2, s: f32, mode: &FlyMode, state: &FlyState) {
    let color = match mode {
        FlyMode::Companion => Color32::from_rgb(80, 200, 120),
        FlyMode::Chaos => Color32::from_rgb(220, 60, 60),
    };
    let r = if matches!(state, FlyState::Landed) && matches!(mode, FlyMode::Chaos) {
        4.5
    } else {
        2.5
    } * s;
    painter.circle_filled(pos2(pos.x, pos.y - 24.0 * s), r, color);
}
