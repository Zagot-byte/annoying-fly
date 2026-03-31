# Annoying Fly 🪰

A virtual fly that lives on your desktop and follows your cursor everywhere. Built in Rust for learning and maximum annoyance.

## What It Does

- **Follows your cursor** across your entire screen
- **Transparent overlay window** that sits on top of everything
- **AI behavior**: Erratic flight patterns, occasional landings, realistic movement
- **Cross-platform**: Works on Linux (X11/Wayland) and Windows
- **Minimal resource usage**: Pure Rust, native performance

## Why This Project?

1. **Learning Rust** - GUI programming, system APIs, event loops
2. **OS Integration** - Window management, mouse tracking, transparency
3. **Game AI** - Simple behavior trees, state machines
4. **Portfolio Piece** - Fun, visual, demonstrates systems programming

## Features Roadmap

### Phase 1: Basic Functionality (MVP)
- [x] Project setup
- [ ] Transparent window with egui
- [ ] Mouse position tracking
- [ ] Basic follow behavior
- [ ] Simple fly sprite

### Phase 2: Realistic Behavior
- [ ] Erratic movement (sine waves, random jitter)
- [ ] Landing states (pause near cursor)
- [ ] Fleeing behavior (when cursor gets too close)
- [ ] Speed variation
- [ ] Rotation based on movement direction

### Phase 3: Polish
- [ ] Better fly sprite/animation
- [ ] Buzzing sound effect (optional toggle)
- [ ] System tray icon (pause/quit controls)
- [ ] Settings: speed, aggression level, enable/disable
- [ ] Multiple flies mode (swarm!)

### Phase 4: Distribution
- [ ] Compile to tiny binary (<5MB)
- [ ] Package for Arch (AUR)
- [ ] AppImage for other Linux distros
- [ ] Windows installer (.exe)
- [ ] Auto-start on boot option

## Project Structure

```
annoying-fly/
├── src/
│   ├── main.rs       # Entry point, window setup
│   ├── fly.rs        # Fly AI behavior and state machine
│   ├── renderer.rs   # Drawing the fly sprite
│   └── config.rs     # Settings and configuration
├── assets/           # Sprites, sounds, icons
├── Cargo.toml        # Dependencies
└── README.md         # This file
```

## Tech Stack

- **Language**: Rust 2021 edition
- **GUI**: egui + eframe (pure Rust, cross-platform)
- **Mouse Tracking**: mouse-position crate
- **Random**: rand crate
- **Build**: Cargo

## Development

See [SETUP.md](SETUP.md) for full environment setup instructions.

Quick start:
```bash
# Clone and enter directory
cd annoying-fly

# Check dependencies compile
cargo check

# Run in development mode
cargo run

# Build optimized binary
cargo build --release
```

## Controls (Planned)

- **System Tray**: Right-click icon → Pause/Quit
- **Keyboard**: `Ctrl+Q` to quit (if window has focus)
- **CLI**: `annoying-fly --help` for options

## Technical Challenges

1. **Transparent, always-on-top window** - Different per platform
2. **Global mouse tracking** - OS-specific APIs
3. **Efficient rendering** - 60 FPS with minimal CPU
4. **Cursor following without lag** - Smooth interpolation
5. **Cross-platform** - Abstract OS differences

## Inspiration

- Those damn flies in summer that won't leave you alone
- Desktop pets like eSheep, Shimeji
- Learning Rust GUI programming in a fun way

## License

MIT - Do whatever you want with it

## Contributing

This is a learning project, but PRs welcome for:
- Better fly AI behavior
- Platform-specific fixes
- Performance improvements
- More annoying features

---

**Status**: 🚧 In active development

Built with Rust 🦀 on Arch Linux + Hyprland
