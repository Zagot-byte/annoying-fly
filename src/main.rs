use eframe::egui;

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
            // Force a light/visible style so we can see what renders
            cc.egui_ctx.set_visuals(egui::Visuals::light());
            Ok(Box::new(FlyApp::default()))
        }),
    )
}

struct FlyApp {
    fly_x: f32,
    fly_y: f32,
    target_x: f32,
    target_y: f32,
}

impl Default for FlyApp {
    fn default() -> Self {
        Self {
            fly_x: 400.0,
            fly_y: 300.0,
            target_x: 400.0,
            target_y: 300.0,
        }
    }
}

impl eframe::App for FlyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Get cursor position relative to this window (works on Wayland + X11)
        if let Some(pos) = ctx.input(|i| i.pointer.latest_pos()) {
            self.target_x = pos.x;
            self.target_y = pos.y;
        }

        self.update_fly_position();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Annoying Fly Debug");
            ui.label(format!("Mouse: {:.0}, {:.0}", self.target_x, self.target_y));
            ui.label(format!("Fly pos: {:.1}, {:.1}", self.fly_x, self.fly_y));

            ui.separator();

            self.draw_fly(ui);

            ctx.request_repaint();
        });

        let _ = frame;

    }
}

impl FlyApp {
    fn update_fly_position(&mut self) {
        let dx = self.target_x - self.fly_x;
        let dy = self.target_y - self.fly_y;
        self.fly_x += dx * 0.08;
        self.fly_y += dy * 0.08;
    }

    fn draw_fly(&self, ui: &mut egui::Ui) {
        let painter = ui.painter();

        // Use actual fly position (was hardcoded before — that was the bug)
        let fly_pos = egui::pos2(self.fly_x, self.fly_y);
        let fly_size = 30.0;

        // Wings (behind body)
        painter.circle_filled(
            egui::pos2(fly_pos.x - 15.0, fly_pos.y - 8.0),
            12.0,
            egui::Color32::from_rgba_unmultiplied(180, 180, 200, 200),
        );
        painter.circle_filled(
            egui::pos2(fly_pos.x + 15.0, fly_pos.y - 8.0),
            12.0,
            egui::Color32::from_rgba_unmultiplied(180, 180, 200, 200),
        );

        // Body
        painter.circle_filled(
            fly_pos,
            fly_size / 2.0,
            egui::Color32::from_rgb(30, 30, 30),
        );

        // Eyes
        for sign in [-1.0_f32, 1.0] {
            let eye = egui::pos2(fly_pos.x + sign * 5.0, fly_pos.y - 3.0);
            painter.circle_filled(eye, 3.0, egui::Color32::WHITE);
            painter.circle_filled(eye, 1.5, egui::Color32::BLACK);
        }
    }
}
