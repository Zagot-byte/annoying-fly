# Quick Start - Get It Running in 5 Minutes

## Absolute Minimum to Run This

### 1. Install Rust (one-time setup)

```bash
# On Arch Linux
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Follow prompts, choose default installation
# Then reload shell
source $HOME/.cargo/env
```

### 2. Install System Dependencies

**Arch Linux (your setup):**
```bash
sudo pacman -S libxcb libxkbcommon wayland
```

**If you get build errors about missing libs, also install:**
```bash
sudo pacman -S base-devel libx11 libxrandr libxi
```

### 3. Clone/Navigate to Project

```bash
cd /path/to/annoying-fly
```

### 4. Run It

```bash
# First run will download dependencies and compile (takes 2-3 min)
cargo run

# Subsequent runs are faster
cargo run
```

That's it! You should see a transparent window with a fly following your cursor.

## Common First-Run Issues

### Error: "linker 'cc' not found"
```bash
sudo pacman -S base-devel
```

### Error: "cannot find -lxcb"
```bash
sudo pacman -S libxcb libxkbcommon
```

### Error: "failed to run custom build command for..."
You're missing a system library. Read the error carefully and install the missing package.

### Window doesn't appear
Check if it's appearing on a different monitor/workspace. The window might be positioned off-screen initially.

### Fly is laggy
You're running debug build. Try:
```bash
cargo run --release
```

## Quick Development Commands

```bash
# Just check if code compiles (fast, no run)
cargo check

# Run with detailed error messages
RUST_BACKTRACE=1 cargo run

# Clean build artifacts (if things get weird)
cargo clean && cargo build

# Format code automatically
cargo fmt

# Run linter
cargo clippy
```

## Next Steps After It Runs

1. Open `src/main.rs` in Claude Code or your editor
2. Look at the `update_fly_position()` function
3. Experiment with changing the `0.05` value (fly speed)
4. Change fly colors in `draw_fly()`
5. Add randomness using the `rand` crate

## Expected Behavior (Current MVP)

- Transparent window appears
- Simple black fly with gray wings
- Smoothly follows your cursor
- Stays on top of all windows
- No title bar or decorations

## To Quit

Since there's no UI yet, kill it from terminal:
- Focus terminal and press `Ctrl+C`
- Or: `killall annoying-fly`

## File You'll Edit Most

`src/main.rs` - Contains everything for now. As the project grows, we'll split into multiple files.

---

**Stuck?** Check SETUP.md for detailed explanations, or drop the error in chat.
