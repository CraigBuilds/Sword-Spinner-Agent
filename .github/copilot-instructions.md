# GitHub Copilot Instructions for Sword Spinner

## Project Overview

Top-down arcade game built with:
- **Language**: Rust 2021
- **Engine**: Bevy 0.15 (ECS)
- **Physics**: Avian2D
- **Platforms**: Desktop, Android

## Code Style

- Follow Rust conventions (rustfmt, clippy)
- Descriptive variable names
- Minimal type annotations (only when needed)
- Comments only for complex logic
- Single-purpose functions

## Development Workflow

**Always run checks after code changes:**

1. `cargo check` - Fast syntax validation
2. `cargo clippy` - Linting
3. `cargo fmt` - Format code
4. Use `report_progress` to commit changes
5. Fix any clippy warnings related to your changes

**When to check:**
- After editing Rust files
- After adding dependencies
- After architectural changes

**Full builds:** Only at the end or when specifically needed (`cargo run`/`cargo build`).

## Architecture

**Bevy ECS:**
- Use Entity Component System pattern
- Components: Data-focused, no logic
- Systems: Focused, single-purpose functions
- Resources: Global state and configuration

**Physics:**
- Avian2D for collision detection and rigid bodies
- Zero gravity (top-down)
- Player: Mass 2.0, rotation locked
- Sword: Mass 0.5, connected via RevoluteJoint
- Obstacles: Mass 1.0

## Platform-Specific

**Desktop:**
- WASD/arrow keys + mouse input
- X11 feature for Linux

**Android:**
- Minimum SDK 24, Target SDK 34
- Touch input via virtual_joystick
- Debug keystore for releases (not Play Store ready)
- Test on real devices

## Common Tasks

**New Component:**
- Create struct with public fields
- Derive `Component` trait
- Add to entities in setup

**New System:**
- Function with system parameters
- Add to App schedule (Update, FixedUpdate)
- Consider system ordering

**Touch Controls:**
- Modify in main.rs
- Test on real device
- Avoid conflicts between movement and actions

## Security

- Never commit secrets
- Check dependencies for vulnerabilities
- Debug keystore is intentional (documented)

## Resources

- [Bevy Docs](https://bevyengine.org/learn/)
- [Avian2D Docs](https://docs.rs/avian2d/)
- See SETUP.md for environment setup
