mod fly;
mod renderer;

use eframe::egui;
use fly::Fly;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_decorations(true)
            .with_transparent(false)
            .with_always_on_top()
            .with_resizable(false),
        ..Default::default()
    };

    eframe::run_native(
        "Annoying Fly",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::light());
            Ok(Box::new(FlyApp::new(cc)))
        }),
    )
}

struct FlyApp {
    fly: Fly,
    idle_frames: Vec<egui::TextureHandle>,
}

impl FlyApp {
    fn new(cc: &eframe::CreationContext) -> Self {
        let idle_frames = vec![
            load_texture(&cc.egui_ctx, "idle-1", include_bytes!("../sprit/idle-1.png")),
            load_texture(&cc.egui_ctx, "idle-2", include_bytes!("../sprit/idle-2.png")),
            load_texture(&cc.egui_ctx, "idle-3", include_bytes!("../sprit/idl-3.png")),
            load_texture(&cc.egui_ctx, "idle-4", include_bytes!("../sprit/idle-4.png")),
        ];

        Self {
            fly: Fly::new(400.0, 300.0),
            idle_frames,
        }
    }
}

impl eframe::App for FlyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let dt = ctx.input(|i| i.unstable_dt).min(0.05);

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

        // Pick animation frame from wing_phase (0..TAU → 4 frames)
        let frame_idx = (self.fly.wing_phase / std::f32::consts::TAU * 4.0) as usize % 4;
        let current_frame = &self.idle_frames[frame_idx];

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(format!(
                "Mode: {:?}  State: {:?}  Scale: {:.2}  |  Ctrl+Shift+A",
                self.fly.mode, self.fly.state, self.fly.scale,
            ));

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
