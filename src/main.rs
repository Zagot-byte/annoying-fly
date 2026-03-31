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
            Ok(Box::new(FlyApp::new()))
        }),
    )
}

struct FlyApp {
    fly: Fly,
}

impl FlyApp {
    fn new() -> Self {
        Self {
            fly: Fly::new(400.0, 300.0),
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

        // Keep fly aware of viewport size (for off-screen entry/exit)
        if let Some(rect) = ctx.input(|i| i.viewport().inner_rect) {
            self.fly.set_viewport(rect.width(), rect.height());
        }

        // Feed cursor position into fly
        if let Some(pos) = ctx.input(|i| i.pointer.latest_pos()) {
            self.fly.set_target(pos.x, pos.y);
        }

        self.fly.update(dt);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(format!(
                "Mode: {:?}  State: {:?}  |  Ctrl+Shift+A to toggle",
                self.fly.mode, self.fly.state,
            ));
            ui.label(format!(
                "Cursor ({:.0},{:.0})  Fly ({:.0},{:.0})  Scale {:.2}",
                self.fly.target_x, self.fly.target_y, self.fly.x, self.fly.y, self.fly.scale,
            ));

            renderer::draw_fly(&self.fly, ui.painter());

            ctx.request_repaint();
        });
    }
}
