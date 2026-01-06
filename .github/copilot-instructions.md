# GitHub Copilot Instructions for Sword Spinner

## Project Overview

Sword Spinner is a top-down arcade game built with:
- **Language**: Rust (edition 2021)
- **Game Engine**: Bevy 0.15 (ECS architecture)
- **Physics**: Avian2D for 2D rigid body simulation
- **Platforms**: Desktop (Windows, macOS, Linux) and Android

## Architecture

- **ECS Pattern**: Use Bevy's Entity Component System throughout
- **Systems**: Organize game logic into focused systems
- **Components**: Keep components data-focused, avoid logic
- **Resources**: Use for global state and configuration
- **Physics**: Leverage Avian2D for collision detection and rigid bodies

## Code Style

- Follow standard Rust conventions (rustfmt and clippy)
- Use descriptive variable names
- Prefer explicit types over `auto`/inference in complex scenarios
- Add comments only for complex physics calculations or non-obvious logic
- Keep functions focused and single-purpose

## Build Commands

### Desktop
```bash
cargo check          # Fast syntax checking
cargo clippy         # Linting
cargo fmt            # Format code
cargo run            # Debug build
cargo run --release  # Release build
```

### Android
```bash
cargo apk build                              # Debug APK
cargo apk build --release                    # Release APK
cargo apk run --release                      # Install and run
adb logcat | grep RustStdoutStderr          # View logs
```

## Testing

- No automated test suite currently exists
- Manual testing required for both platforms
- For Android: Test touch controls on real devices, not just emulators
- Verify physics behavior after changes to mass, damping, or joint settings

## Dependencies

- **Bevy**: Use minimal feature flags to reduce compile time and binary size
- **Avian2D**: For all physics operations (RigidBody, Collider, joints)
- **virtual_joystick**: For mobile touch input
- **android-activity**: For Android platform integration
- Always check for security vulnerabilities before adding new dependencies

## Platform-Specific Code

### Desktop
- Use keyboard (WASD/arrows) and mouse input
- Enable X11 feature for Linux builds
- No special windowing considerations

### Android
- Minimum SDK: 24 (Android 7.0)
- Target SDK: 34
- Use touch input via virtual_joystick
- Test on devices with different screen sizes
- Release builds use debug keystore (not for Play Store)

## Physics Configuration

- **Gravity**: Zero (top-down game)
- **Player**: Mass 2.0, rotation locked
- **Sword**: Mass 0.5, connected via RevoluteJoint
- **Obstacles**: Mass 1.0
- **Damping**: Apply to prevent endless motion
- **Joint Compliance**: Keep low for stiff sword connection

## Security

- Never commit secrets or API keys
- Use environment variables for sensitive configuration
- The release keystore uses debug credentials (documented behavior)
- Review any external crate dependencies for vulnerabilities

## Performance

- Release builds use thin LTO and single codegen unit
- Strip debug symbols in release builds
- Test performance on release builds only
- Monitor frame rate on target devices (aim for 60 FPS)
- Optimize Bevy features for mobile builds

## File Structure

```
src/
  main.rs           # Game entry point and logic
  lib.rs           # Library entry for Android builds
Cargo.toml         # Dependencies and Android metadata
AndroidManifest.xml # Android configuration
.github/
  workflows/       # CI/CD pipelines
```

## Common Tasks

### Adding a New Component
- Create struct with public fields
- Derive `Component` trait
- Add to relevant entities in setup system

### Adding a New System
- Create function with appropriate system parameters
- Add to relevant App schedule (Update, FixedUpdate, etc.)
- Consider system ordering and dependencies

### Modifying Physics
- Adjust values in the `setup` function
- Test on both desktop and mobile
- Document non-obvious parameter choices

### Touch Controls
- Modify touch handling in main.rs
- Test with real device, not emulator
- Ensure no conflict between movement and action inputs

## CI/CD

- GitHub Actions builds Android release APKs automatically
- Workflows in `.github/workflows/` configure build steps
- `copilot-setup-steps.yml` ensures proper toolchain for Copilot agent
- Rust cache is shared with key "bevy" for faster builds

## Resources

- [Bevy Documentation](https://bevyengine.org/learn/)
- [Avian2D Documentation](https://docs.rs/avian2d/)
- [Rust Book](https://doc.rust-lang.org/book/)
- See SETUP.md for detailed development environment setup
