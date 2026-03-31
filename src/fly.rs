use rand::Rng;

pub const IDLE_THRESHOLD: f32 = 2.0; // seconds (change to 20.0 for production)

#[derive(Debug, Clone, PartialEq)]
pub enum FlyMode {
    Companion,
    Chaos,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FlyState {
    Flying,
    Landing,
    Landed,
    TakingOff,
    /// Fly entering from off-screen toward `dest`
    FlyingIn { dest_x: f32, dest_y: f32 },
}

pub struct Fly {
    pub x: f32,
    pub y: f32,
    pub target_x: f32,
    pub target_y: f32,
    pub mode: FlyMode,
    pub state: FlyState,
    pub wing_phase: f32,
    /// Current draw scale (grows while cursor is idle)
    pub scale: f32,

    // Viewport size so we know where "off-screen" is
    pub viewport_w: f32,
    pub viewport_h: f32,

    land_timer: f32,
    landed_timer: f32,
    idle_timer: f32,

    prev_cursor_x: f32,
    prev_cursor_y: f32,
    cursor_speed: f32,
}

impl Fly {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            target_x: x,
            target_y: y,
            mode: FlyMode::Companion,
            state: FlyState::Flying,
            wing_phase: 0.0,
            scale: 1.0,
            viewport_w: 800.0,
            viewport_h: 600.0,
            land_timer: rng().gen_range(4.0..8.0),
            landed_timer: 0.0,
            idle_timer: 0.0,
            prev_cursor_x: x,
            prev_cursor_y: y,
            cursor_speed: 0.0,
        }
    }

    pub fn toggle_mode(&mut self) {
        self.mode = match self.mode {
            FlyMode::Companion => FlyMode::Chaos,
            FlyMode::Chaos => FlyMode::Companion,
        };
        self.land_timer = rng().gen_range(3.0..8.0);
    }

    pub fn set_viewport(&mut self, w: f32, h: f32) {
        self.viewport_w = w;
        self.viewport_h = h;
    }

    pub fn set_target(&mut self, cursor_x: f32, cursor_y: f32) {
        let dcx = cursor_x - self.prev_cursor_x;
        let dcy = cursor_y - self.prev_cursor_y;
        self.cursor_speed = (dcx * dcx + dcy * dcy).sqrt();
        self.prev_cursor_x = cursor_x;
        self.prev_cursor_y = cursor_y;
        self.target_x = cursor_x;
        self.target_y = cursor_y;
    }

    pub fn update(&mut self, dt: f32) {
        let cursor_active = self.cursor_speed > 1.5;

        // ── Idle tracking ──────────────────────────────────────────────────────
        if cursor_active {
            self.idle_timer = 0.0;
        } else {
            self.idle_timer += dt;
        }

        // Scale grows past the idle threshold, caps at 4×
        if self.idle_timer > IDLE_THRESHOLD {
            let excess = self.idle_timer - IDLE_THRESHOLD;
            self.scale = (1.0 + excess * 0.25).min(4.0);
        } else {
            // Shrink slowly — fly stays big and menacing for a while after waking
            self.scale = (self.scale - dt * 0.4).max(1.0);
        }

        // ── Trigger FlyingIn when idle threshold first hit ─────────────────────
        if self.idle_timer >= IDLE_THRESHOLD
            && matches!(self.state, FlyState::Flying | FlyState::TakingOff)
        {
            let (sx, sy) = self.random_edge_point();
            let (dx, dy) = self.random_inner_point();
            self.x = sx;
            self.y = sy;
            self.state = FlyState::FlyingIn { dest_x: dx, dest_y: dy };
        }

        // ── If cursor wakes up while flying in → chase it aggressively ───────────
        if cursor_active {
            if matches!(self.state, FlyState::FlyingIn { .. }) {
                // Don't flee — go straight for the cursor and land on it
                self.state = FlyState::Landing;
                self.landed_timer = 0.0;
            }
        }

        // ── Wing animation ─────────────────────────────────────────────────────
        let flap_speed = match self.state {
            FlyState::Landed => 4.0,
            _ => 18.0,
        };
        self.wing_phase = (self.wing_phase + dt * flap_speed) % std::f32::consts::TAU;

        // ── State machine ──────────────────────────────────────────────────────
        match self.state.clone() {
            FlyState::Flying => {
                let dx = self.target_x - self.x;
                let dy = self.target_y - self.y;
                self.x += dx * 0.05;
                self.y += dy * 0.05;

                if matches!(self.mode, FlyMode::Chaos) {
                    self.land_timer -= dt;
                    if self.land_timer <= 0.0 {
                        self.state = FlyState::Landing;
                        self.land_timer = rng().gen_range(3.0..8.0);
                    }
                }
            }

            FlyState::Landing => {
                let dx = self.target_x - self.x;
                let dy = self.target_y - self.y;
                let distance = (dx * dx + dy * dy).sqrt();
                if distance < 5.0 {
                    self.state = FlyState::Landed;
                    self.landed_timer = 0.0;
                } else {
                    self.x += dx * 0.15;
                    self.y += dy * 0.15;
                }
            }

            FlyState::Landed => {
                self.x = self.target_x;
                self.y = self.target_y;
                self.landed_timer += dt;

                // Bigger fly needs a harder shake to dislodge
                let shake_threshold = 25.0 * self.scale;
                let shaken = self.cursor_speed > shake_threshold;
                if shaken || self.landed_timer > 2.0 {
                    self.state = FlyState::TakingOff;
                }
            }

            FlyState::TakingOff => {
                let dx = self.target_x - self.x;
                let dy = self.target_y - self.y;
                self.x -= dx * 0.1;
                self.y -= dy * 0.1;

                let distance = (dx * dx + dy * dy).sqrt();
                if distance > 100.0 {
                    self.state = FlyState::Flying;
                }
            }

            FlyState::FlyingIn { dest_x, dest_y } => {
                let dx = dest_x - self.x;
                let dy = dest_y - self.y;
                let dist = (dx * dx + dy * dy).sqrt();

                // Faster entry to feel dramatic
                self.x += dx * 0.06;
                self.y += dy * 0.06;

                if dist < 8.0 {
                    // Arrived — just hover (stay in FlyingIn with same dest, drifting slowly)
                    // Pick a new nearby dest so it slowly wanders while growing
                    let new_dest_x = (dest_x + rng().gen_range(-40.0..40.0))
                        .clamp(60.0, self.viewport_w - 60.0);
                    let new_dest_y = (dest_y + rng().gen_range(-40.0..40.0))
                        .clamp(60.0, self.viewport_h - 60.0);
                    self.state = FlyState::FlyingIn { dest_x: new_dest_x, dest_y: new_dest_y };
                }
            }

        }
    }

    pub fn is_landed(&self) -> bool {
        self.state == FlyState::Landed
    }

    fn random_edge_point(&self) -> (f32, f32) {
        let w = self.viewport_w;
        let h = self.viewport_h;
        match rng().gen_range(0..4) {
            0 => (rng().gen_range(0.0..w), -60.0),       // top
            1 => (rng().gen_range(0.0..w), h + 60.0),    // bottom
            2 => (-60.0, rng().gen_range(0.0..h)),        // left
            _ => (w + 60.0, rng().gen_range(0.0..h)),    // right
        }
    }

    fn random_inner_point(&self) -> (f32, f32) {
        let margin = 80.0;
        (
            rng().gen_range(margin..self.viewport_w - margin),
            rng().gen_range(margin..self.viewport_h - margin),
        )
    }
}

fn rng() -> impl Rng {
    rand::thread_rng()
}
