# Annoying Fly - Project Context

## Project Overview

A desktop application that renders a virtual fly on your screen with two distinct modes:
1. **Companion Mode** - Cute, friendly fly that orbits your cursor
2. **Chaos Mode** - Maximum annoyance fly that actively disrupts your work

Built in Rust using egui for cross-platform GUI (Linux + Windows).

---

## Current Status

### ✅ Completed (MVP - Phase 0)
- [x] Rust project scaffolding with Cargo
- [x] Basic egui window setup
- [x] Mouse position tracking (system-wide)
- [x] Debug UI showing real-time mouse coordinates
- [x] Simple fly sprite rendering (body + wings + eyes)
- [x] Basic follow behavior (smooth interpolation)

### 🚧 In Progress
- [ ] Dual mode architecture
- [ ] Mode switching mechanism

### 📋 Planned Features
See roadmap below.

---

## Architecture

### Tech Stack
- **Language:** Rust 2021 edition
- **GUI Framework:** eframe + egui (pure Rust, cross-platform)
- **Mouse Tracking:** `mouse_position` crate
- **Random Behavior:** `rand` crate
- **Build System:** Cargo

### Project Structure
```
annoying-fly/
├── src/
│   ├── main.rs           # Entry point, window setup
│   ├── fly.rs            # Fly behavior state machine (TODO)
│   ├── companion.rs      # Companion mode logic (TODO)
│   ├── chaos.rs          # Chaos mode logic (TODO)
│   └── config.rs         # Settings & configuration (TODO)
├── assets/               # Sprites, sounds (TODO)
├── Cargo.toml           # Dependencies
├── CONTEXT.md           # This file
├── README.md            # User-facing documentation
├── SETUP.md             # Development environment setup
└── QUICKSTART.md        # Get running in 5 minutes
```

---

## Feature Roadmap

### Phase 1: Companion Mode (Cute Fly)
**Goal:** Non-intrusive desktop pet

#### Core Features
- [ ] **Orbital Movement**
  - Fly circles around cursor at configurable radius (50-150px)
  - Smooth acceleration/deceleration
  - Occasional path variations (figure-8, spiral)

- [ ] **Landing Behavior**
  - Random landings when cursor is idle (>2 seconds)
  - Land near cursor, not on it
  - Rest animation (wings fold, occasional twitch)
  - Takeoff when cursor moves

- [ ] **Animations**
  - Wing flapping (3-4 frame cycle)
  - Wing cleaning (rub front legs together)
  - Looking around (head rotation)
  - Idle breathing

- [ ] **Personality Traits**
  - Configurable: Lazy, Energetic, Curious
  - Affects: flight speed, landing frequency, orbit radius

- [ ] **Visual Polish**
  - Better sprite with actual fly anatomy
  - Rotation based on movement direction
  - Shadow beneath fly
  - Smooth animations

#### Nice-to-Have
- Multiple flies that avoid each other
- React to typing speed (excited when active)
- Sleep mode when system idle
- Customizable fly colors/skins

---

### Phase 2: Chaos Mode (Annoying Fly)
**Goal:** Maximum disruption and annoyance

#### Core Features
- [ ] **Cursor Interference**
  - Land DIRECTLY on cursor
  - Block clicks for 1-2 seconds
  - Require "shake" gesture to remove (rapid mouse movement)

- [ ] **Click Hijacking**
  - Randomly intercept clicks
  - Click random UI elements when landing
  - Double-click when you single-click

- [ ] **Audio Annoyance**
  - Buzzing sound loop
  - Volume increases when cursor approaches
  - Pitched buzzing when "swatted"

- [ ] **Multiplication**
  - Spawn additional flies over time
  - Each fly has independent behavior
  - Cap at configurable max (default: 10)

- [ ] **Visual Disruption**
  - Leave "droppings" (tiny brown dots)
  - Droppings accumulate slowly
  - Require manual cleanup (button in tray)

- [ ] **Anti-Quit Mechanisms**
  - Fake "Are you sure?" dialogs
  - "Loading..." spinner when trying to quit
  - Fly escapes from close button
  - Require secret quit sequence (Ctrl+Alt+Shift+Q)

- [ ] **Focus Stealing**
  - Randomly bring window to front
  - Steal keyboard focus briefly
  - Type random characters occasionally

#### Nice-to-Have
- Fly "eats" text from screen (characters disappear temporarily)
- Fake system notifications ("Fly.exe has stopped working")
- Screen shake when swatting
- Fly corpses that remain when killed
- "Fly friends" arrive via fake email notifications

---

### Phase 3: Dual Mode System
**Goal:** Seamless mode switching with gradual escalation

#### Core Features
- [ ] **Mode Toggle**
  - Hotkey: `Ctrl+Shift+A` (toggle between modes)
  - System tray icon with context menu
  - Visual indicator of current mode

- [ ] **Settings Panel**
  - Annoyance slider (1-10)
  - Level 1-3: Companion mode
  - Level 4-6: Mild annoyance (cursor interference only)
  - Level 7-9: Heavy annoyance (clicks + audio)
  - Level 10: MAXIMUM CHAOS (everything enabled)

- [ ] **Gradual Escalation**
  - Companion mode by default
  - If ignored for X minutes → mild annoyance
  - If still ignored → escalate to chaos
  - Reset to companion when interacted with

- [ ] **Interaction System**
  - "Pet" the fly (hover for 3 seconds)
  - "Feed" the fly (drop file on it)
  - Resets escalation timer

#### Nice-to-Have
- Mode schedule (companion during work hours, chaos after 5pm)
- Friend mode (share your fly's chaos level with others)
- Achievement system (survived 1 hour of chaos mode)

---

### Phase 4: Platform Integration & Polish

#### Cross-Platform
- [ ] **Linux (Current)**
  - X11 transparency working
  - Wayland support (test on Hyprland)
  - System tray integration

- [ ] **Windows**
  - Transparent overlay window
  - Click-through regions
  - System tray icon
  - Auto-start on boot (optional)

#### Distribution
- [ ] **Arch Linux**
  - PKGBUILD for AUR
  - Include in `annoying-fly` package

- [ ] **Other Linux**
  - AppImage
  - Flatpak (maybe)

- [ ] **Windows**
  - Standalone .exe installer
  - Windows Defender exclusion warning

#### Polish
- [ ] Better sprites (multiple fly types)
- [ ] Sound effects (buzzing, swatting, landing)
- [ ] Persistent settings (save/load config)
- [ ] First-run tutorial
- [ ] Uninstaller that shows "sad fly" animation

---

## Technical Challenges

### Current Issues
1. **Transparency on Hyprland**
   - Black window instead of transparent
   - Need window rules or eframe configuration fix
   - Workaround: Running in debug mode with decorations

2. **Coordinate System Mismatch**
   - Mouse position is screen coordinates (absolute)
   - Painter draws in window coordinates (relative)
   - Need to convert between systems for proper following

3. **Performance**
   - 60 FPS repaint might be overkill
   - Optimize to only repaint on movement

### Upcoming Challenges
1. **Click Interception (Chaos Mode)**
   - Need OS-level input hooks
   - X11: XGrabPointer
   - Windows: SetWindowsHookEx
   - Security implications - might trigger antivirus

2. **Audio Playback**
   - Need audio library (rodio crate?)
   - Loop buzzing sound
   - Spatial audio (louder when near cursor)

3. **Multiple Windows**
   - Each fly = separate window? Or one window with many flies?
   - Performance implications

4. **Window Always-On-Top**
   - Works on X11, but needs testing on Wayland
   - Windows might have Z-order conflicts

---

## Development Workflow

### Current Setup
```bash
# Development cycle
cargo check        # Fast syntax check
cargo run          # Debug build + run
cargo run --release # Optimized build

# Code formatting
cargo fmt

# Linting
cargo clippy
```

### Testing Strategy
1. **Manual Testing**
   - Run on Hyprland (primary)
   - Test each behavior individually
   - Annoyance testing (recruit friends)

2. **Future: Automated Tests**
   - Unit tests for behavior state machine
   - Integration tests for mode switching
   - Performance benchmarks

---

## Design Decisions

### Why Rust?
- Learning opportunity (primary goal)
- Memory safety for long-running app
- Cross-platform without Electron bloat
- Performance for smooth animations

### Why egui?
- Pure Rust (no FFI)
- Immediate mode = simple state management
- Cross-platform out of box
- Good transparency support

### Why Dual Mode?
- Companion mode = actually useful/cute
- Chaos mode = fun demo/prank
- Both together = more interesting project
- Shows state management complexity

---

## Known Bugs

### Current
- [ ] Window is black instead of transparent on Hyprland
- [ ] Fly doesn't actually follow cursor (coordinates not converted)
- [ ] Debug text updates but fly sprite is static

### Future
- Track in GitHub issues once project is public

---

## Resources & References

### Rust GUI
- [egui documentation](https://docs.rs/egui/)
- [eframe examples](https://github.com/emilk/egui/tree/master/examples)

### Desktop Pets Inspiration
- eSheep (classic Windows desktop sheep)
- Shimeji (Java desktop mascots)
- Neko (classic cat cursor follower)

### Similar Projects
- [xeyes](https://www.x.org/releases/X11R7.5/doc/man/man1/xeyes.1.html) - eyes follow cursor
- [oneko](https://wiki.archlinux.org/title/Oneko) - cat chases cursor

---

## Future Ideas (Maybe)

- [ ] Fly leaves a trail (like cursor trail)
- [ ] Multiplayer mode (flies interact across network)
- [ ] Fly "evolution" (levels up over time)
- [ ] Different insect types (bee, wasp, moth)
- [ ] Seasonal themes (butterfly in spring, firefly at night)
- [ ] VR mode (fly in 3D space)
- [ ] Blockchain integration (just kidding)

---

## Notes for Development

### Code Style
- Use descriptive variable names
- Comment complex behavior logic
- Keep functions small (<50 lines)
- Separate concerns (fly logic vs. rendering vs. audio)

### Performance Targets
- < 5% CPU usage when idle
- < 50MB RAM for single fly
- 60 FPS smooth animation
- < 5MB binary size (release build)

### Accessibility Considerations
- Disable animations option (motion sickness)
- Mute sound option
- High contrast mode
- Easy quit mechanism (don't trap users)

---

**Last Updated:** Current session
**Version:** 0.1.0-alpha (MVP complete, dual mode in progress)
**Status:** Active development, learning Rust fundamentals
