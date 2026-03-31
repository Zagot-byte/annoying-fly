# Annoying Fly - Development Setup Guide

## Prerequisites & Installation

### 1. Install Rust (if not already installed)

On Linux (Arch):
```bash
# Official way - rustup (recommended)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Or via pacman (Arch)
sudo pacman -S rustup
rustup default stable
```

After installation, add to your shell config (~/.config/fish/config.fish or ~/.bashrc):
```bash
# Cargo bin directory
export PATH="$HOME/.cargo/bin:$PATH"
```

Restart your terminal or run:
```bash
source ~/.cargo/env
```

Verify installation:
```bash
rustc --version
cargo --version
```

### 2. Install System Dependencies (Linux)

For egui/eframe to work, you need some system libraries:

**Arch Linux:**
```bash
sudo pacman -S libxcb libxkbcommon libxkbcommon-x11 wayland
```

**Ubuntu/Debian:**
```bash
sudo apt install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev \
    libxkbcommon-dev libssl-dev libwayland-dev
```

### 3. Install Rust Analyzer (LSP for IDE)

```bash
rustup component add rust-analyzer
```

## Project Structure

```
annoying-fly/
├── Cargo.toml          # Rust dependencies (like package.json or requirements.txt)
├── Cargo.lock          # Locked dependency versions (auto-generated, like package-lock.json)
├── src/
│   ├── main.rs         # Entry point
│   ├── fly.rs          # Fly behavior and AI logic
│   ├── window.rs       # Window management (transparency, always-on-top)
│   └── lib.rs          # Library code (optional, for shared utilities)
├── assets/
│   ├── fly.png         # Fly sprite (if we use images)
│   └── buzz.wav        # Buzzing sound (optional)
├── target/             # Build artifacts (auto-generated, gitignored)
│   ├── debug/          # Debug builds
│   └── release/        # Optimized release builds
├── .gitignore
├── README.md
└── SETUP.md           # This file
```

## Understanding Cargo (Rust's Package Manager)

Cargo is like:
- `npm` for Node.js
- `pip` + `venv` for Python
- `go mod` for Go

Key commands:

```bash
# Create new project
cargo new project-name        # Creates binary (executable)
cargo new --lib lib-name      # Creates library

# Build project
cargo build                   # Debug build (fast compile, slow runtime)
cargo build --release         # Release build (slow compile, fast runtime)

# Run project
cargo run                     # Build + run in one command
cargo run --release           # Run optimized version

# Check code without building (faster)
cargo check                   # Just verify it compiles

# Run tests
cargo test

# Format code
cargo fmt

# Lint code
cargo clippy                  # Like pylint/eslint for Rust

# Update dependencies
cargo update

# Clean build artifacts
cargo clean
```

## Development Workflow

### First Time Setup

```bash
cd annoying-fly

# Check everything compiles
cargo check

# Build debug version
cargo build

# Run the app
cargo run
```

### Development Loop

```bash
# Make changes in your editor (Claude Code, VSCode, etc.)

# Quick syntax check (fast)
cargo check

# Run to test
cargo run

# Format code before commit
cargo fmt
```

### Testing Release Build

```bash
# Build optimized binary
cargo build --release

# Binary location
./target/release/annoying-fly

# Run optimized version
cargo run --release
```

## IDE Setup (Claude Code / VS Code)

### Extensions Needed

1. **rust-analyzer** - LSP for Rust (must-have)
2. **CodeLLDB** - Debugger
3. **crates** - Dependency version management
4. **Even Better TOML** - Cargo.toml syntax highlighting

### Claude Code Specific

In your terminal:
```bash
# Install Claude Code CLI (if not already)
# Then open project
claude-code annoying-fly/
```

rust-analyzer should auto-detect the project and provide:
- Autocomplete
- Type hints
- Error checking
- Go to definition
- Hover documentation

## Dependency Management

### Adding Dependencies

Edit `Cargo.toml`:
```toml
[dependencies]
some-crate = "1.0"              # Latest 1.x version
another = { version = "2.0", features = ["extra"] }
```

Or use `cargo add`:
```bash
cargo add egui eframe           # Adds to Cargo.toml automatically
cargo add rand --features serde # With specific features
```

### Finding Crates

- Official registry: https://crates.io
- Search: `cargo search keyword`
- Docs: https://docs.rs

## Common Issues & Fixes

### "linker not found" error
```bash
# Install build essentials
sudo pacman -S base-devel       # Arch
sudo apt install build-essential # Ubuntu
```

### "cannot find -lxcb" or similar X11 errors
```bash
# Install X11 dev libraries (see section 2 above)
```

### Slow compile times
- Use `cargo check` during development (faster than build)
- Consider `cargo watch` for auto-recompile on file changes:
  ```bash
  cargo install cargo-watch
  cargo watch -x run
  ```

### Binary too large
```bash
# Add to Cargo.toml
[profile.release]
opt-level = "z"     # Optimize for size
lto = true          # Link-time optimization
strip = true        # Strip symbols
```

## Running on Your Hyprland Setup

The app will:
1. Create a transparent window
2. Set it to always-on-top
3. Track mouse position
4. Render the fly

To quit: Will add system tray icon or keyboard shortcut (Ctrl+Q)

## Next Steps

1. ✅ Complete this setup
2. ⬜ Implement basic window with egui
3. ⬜ Add mouse position tracking
4. ⬜ Implement fly AI behavior
5. ⬜ Add transparency and always-on-top
6. ⬜ Package as standalone binary
7. ⬜ (Optional) Add to AUR for easy Arch installation

Ready to code? Run `cargo run` and let's build this annoying fly! 🪰
