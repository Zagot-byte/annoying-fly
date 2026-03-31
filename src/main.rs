mod fly;
mod renderer;

use eframe::egui;
use fly::Fly;
use enigo::{Enigo, Mouse, Coordinate, Settings};

fn main() -> Result<(), eframe::Error> {
    // Get screen dimensions
    let screen = egui::Vec2::new(1920.0, 1080.0); // Hardcoded for now
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_decorations(false)
            .with_transparent(true)
            .with_always_on_top()
            .with_inner_size(screen)
            .with_position([0.0, 0.0])
            .with_resizable(false),
        ..Default::default()
    };

    eframe::run_native(
        "Annoying Fly",
        options,
        Box::new(|cc| {
            let mut visuals = egui::Visuals::dark();
            visuals.window_fill = egui::Color32::TRANSPARENT;
            visuals.panel_fill = egui::Color32::TRANSPARENT;
            cc.egui_ctx.set_visuals(visuals);
            Ok(Box::new(FlyApp::new(cc)))
        }),
    )
}
struct FlyApp {
    fly: Fly,
    idle_frames: Vec<egui::TextureHandle>,
    enigo: Option<Enigo>,
}

impl FlyApp {
    fn new(cc: &eframe::CreationContext) -> Self {
        let idle_frames = vec![
            load_texture(&cc.egui_ctx, "idle-1", include_bytes!("../assets/idle-1.png")),
            load_texture(&cc.egui_ctx, "idle-2", include_bytes!("../assets/idle-2.png")),
            load_texture(&cc.egui_ctx, "idle-3", include_bytes!("../assets/idle-3.png")),
            load_texture(&cc.egui_ctx, "idle-4", include_bytes!("../assets/idle-4.png")),
        ];

        Self {
            fly: Fly::new(400.0, 300.0),
            idle_frames,
            enigo: Enigo::new(&Settings::default()).ok(),
        }
    }
}

impl eframe::App for FlyApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        [0.0, 0.0, 0.0, 0.0] // fully transparent clear
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let dt = ctx.input(|i| i.unstable_dt).min(0.05);
        
        ctx.send_viewport_cmd(egui::ViewportCommand::MousePassthrough(true));

        // Ctrl+Shift+A toggles Companion ↔ Chaos
        if ctx.input(|i| i.key_pressed(egui::Key::A) && i.modifiers.ctrl && i.modifiers.shift) {
            self.fly.toggle_mode();
        }

        // Keep fly aware of viewport size
        if let Some(rect) = ctx.input(|i| i.viewport().inner_rect) {
            self.fly.set_viewport(rect.width(), rect.height());
        }

        // Feed cursor position
        if let Some(pos) = ctx.input(|i| i.pointer.latest_pos()) {
            self.fly.set_target(pos.x, pos.y);
        }

        self.fly.update(dt);

        // Cursor drift — nudge the real OS cursor while fly is landed
        if let (Some((dx, dy)), Some(enigo)) = (self.fly.cursor_drift(), self.enigo.as_mut()) {
            let _ = enigo.move_mouse(dx, dy, Coordinate::Rel);
        }

        // Pick animation frame from wing_phase (0..TAU → 4 frames)
        let frame_idx = (self.fly.wing_phase / std::f32::consts::TAU * 4.0) as usize % 4;
        let current_frame = &self.idle_frames[frame_idx];

        // Transparent panel — only the fly sprite is visible
        let panel_frame = egui::Frame::none().fill(egui::Color32::TRANSPARENT);
        egui::CentralPanel::default().frame(panel_frame).show(ctx, |ui| {
            renderer::draw_fly(&self.fly, ui.painter(), current_frame);
            ctx.request_repaint();
        });
    }
}

fn load_texture(ctx: &egui::Context, name: &str, bytes: &[u8]) -> egui::TextureHandle {
    let img = image::load_from_memory(bytes)
        .expect("failed to load sprite")
        .to_rgba8();
    let size = [img.width() as usize, img.height() as usize];
    let pixels = img.into_raw();
    ctx.load_texture(
        name,
        egui::ColorImage::from_rgba_unmultiplied(size, &pixels),
        egui::TextureOptions::LINEAR,
    )
}
