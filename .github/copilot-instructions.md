# Copilot Instructions for Sword Spinner

## Repository Overview

Sword Spinner is a top-down arcade game built with Rust, Bevy 0.15, and Avian2D physics. The game features a character that moves around an arena and spins a physics-based sword to knock around obstacles. It supports both desktop (Windows, macOS, Linux) and Android platforms.

## Technical Stack

- **Language**: Rust (latest stable)
- **Game Engine**: Bevy 0.15 (ECS architecture)
- **Physics Engine**: Avian2D 0.2 (2D rigid body physics)
- **Build System**: Cargo
- **Android Build**: cargo-apk
- **CI/CD**: GitHub Actions

## Project Structure

```
sword-spinner/
├── .github/
│   ├── workflows/
│   │   ├── android-release.yml     # CI/CD for Android releases
│   │   ├── android-test.yml        # Android build testing
│   │   └── copilot-setup-steps.yml # Development environment setup
│   └── copilot-instructions.md     # This file
├── src/
│   ├── main.rs                     # Desktop entry point and game implementation
│   └── lib.rs                      # Android entry point
├── Cargo.toml                      # Dependencies and Android metadata
├── AndroidManifest.xml             # Android configuration
├── README.md                       # User documentation
├── SETUP.md                        # Developer setup guide
└── LICENSE                         # MIT license
```

## Coding Standards

### Rust Style

- Follow standard Rust conventions and idioms
- Use `cargo fmt` to format code (runs with default rustfmt settings)
- Use `cargo clippy` to catch common mistakes and improve code quality
- Prefer explicit types over inference when it improves clarity
- Use descriptive variable names that reflect the domain (e.g., `sword_entity`, `touch_state`)

### Bevy ECS Patterns

- Use Bevy's Entity Component System (ECS) architecture
- Systems should be small, focused functions
- Prefer queries over direct entity access
- Use resources for global state (e.g., `TouchState`)
- Use components for entity-specific data
- Bundle related components together
- Use change detection queries when appropriate

### Physics Code

- Physics configuration uses Avian2D
- Zero gravity for top-down gameplay
- Use `LockedAxes` to prevent unwanted rotation on player
- Apply damping to prevent endless motion
- Mass ratios: Player (2.0), Sword (0.5), Obstacles (1.0)
- Use `RevoluteJoint` for sword attachment with low compliance for stiffness

### Comments

- Add comments to explain complex physics calculations
- Document non-obvious game mechanics (e.g., double-tap detection logic)
- Avoid obvious comments that just restate the code
- Use doc comments (`///`) for public items

## Building and Testing

### Desktop Development

```bash
# Run in debug mode (fast compilation, slower runtime)
cargo run

# Run in release mode (optimized, best for testing gameplay)
cargo run --release

# Check code without building
cargo check

# Format code
cargo fmt

# Run linter
cargo clippy
```

### Android Development

```bash
# Build debug APK
cargo apk build

# Build release APK (optimized, smaller size)
cargo apk build --release

# Install and run on connected device
cargo apk run --release

# View logs from device
adb logcat | grep RustStdoutStderr
```

### Performance Considerations

- Always test gameplay performance with `--release` builds
- Debug builds are significantly slower and don't represent real performance
- Use thin LTO in release builds for optimization (already configured)
- Optimize Bevy features for mobile (already configured in Cargo.toml)

## Platform-Specific Code

### Desktop Controls

- Movement: WASD or Arrow Keys
- Sword Spin: Spacebar or Left Mouse Button

### Mobile/Android Controls

- Movement: Touch and drag anywhere on screen
- Sword Spin: Double-tap anywhere on screen
  - Must tap twice within 300ms
  - Taps must be within 50 pixels of each other
  - Won't trigger during dragging

### Platform Detection

The codebase uses conditional compilation:
- Android-specific code uses `#[cfg(target_os = "android")]`
- Desktop platforms share the same code path
- Touch input systems are always compiled but only active when touch events occur

## Important Game Mechanics

### Touch Input Handling

The double-tap mechanic is carefully designed to:
- Allow simultaneous movement (drag) and spinning (double-tap)
- Prevent accidental spins during normal gameplay
- Feel intentional and satisfying
- Not interfere with drag-to-move controls

When modifying touch input:
- Maintain the `TouchState` resource structure
- Preserve the double-tap detection logic
- Test thoroughly on real Android devices (not just emulators)
- Consider drag threshold to prevent false positives

### Physics System

The sword is attached to the player via a `RevoluteJoint`:
- Low compliance for tight, responsive connection
- Spin is applied by setting angular velocity
- Damping prevents endless spinning
- Mass ratios ensure realistic momentum transfer

## Testing Guidelines

### Manual Testing

1. **Desktop**: Test all keyboard and mouse controls
2. **Android**: Test on real devices, not just emulators
3. **Touch Controls**: Follow the testing checklist in SETUP.md
4. **Performance**: Verify 60 FPS on target devices

### Touch Control Testing Checklist

- Single tap should not spin sword
- Double-tap (within 300ms) should spin sword
- Dragging should move character smoothly
- Can drag and double-tap simultaneously
- Taps >50px apart should not trigger spin
- Triple-tap should only spin once (on second tap)

## CI/CD Pipeline

### Automated Workflows

- **android-release.yml**: Builds and signs release APKs for version tags (v*)
- **android-test.yml**: Validates Android builds on pushes
- **copilot-setup-steps.yml**: Sets up development environment

### Release Process

1. Tag a version: `git tag v0.1.0`
2. Push tag: `git push origin v0.1.0`
3. GitHub Actions automatically builds, signs, and releases the APK

## Dependencies

### Adding New Dependencies

- Avoid adding dependencies unless necessary
- Prefer dependencies already in the Bevy/Rust ecosystem
- Consider binary size impact for mobile builds
- Check compatibility with Android targets
- Update Cargo.toml with specific version requirements

### Current Key Dependencies

- `bevy = "0.15"` with minimal feature set for mobile
- `avian2d = "0.2"` for physics
- `android-activity = "0.6"` for Android support

## Contribution Guidelines

### Pull Request Best Practices

- Make focused, single-purpose changes
- Test on both desktop and Android when modifying core game logic
- Test on Android device when modifying touch input
- Ensure code compiles for all targets
- Run `cargo fmt` and `cargo clippy` before committing
- Update documentation if changing user-facing features

### Commit Messages

- Use clear, descriptive commit messages
- Start with a verb (Add, Fix, Update, Remove, etc.)
- Reference issue numbers when applicable

## Common Development Tasks

### Adding a New Game Feature

1. Implement as Bevy systems following ECS patterns
2. Test on desktop first for faster iteration
3. Test on Android if it affects cross-platform code
4. Update README.md if it's user-facing
5. Consider performance impact on mobile devices

### Modifying Physics

1. Understand existing physics configuration in `setup` function
2. Test changes thoroughly with `--release` builds
3. Verify behavior feels correct on both platforms
4. Document any non-obvious physics parameters

### Adjusting Touch Controls

1. Review existing `TouchState` and detection logic
2. Make changes incrementally
3. Test on real Android device (emulator touch != real touch)
4. Verify double-tap timing and distance thresholds
5. Ensure no regression in drag-to-move functionality

## Troubleshooting

### Common Issues

- **Build Errors**: Check Rust version with `rustc --version` (should be latest stable)
- **Android Build Fails**: Verify Android NDK is installed and `ANDROID_NDK_ROOT` is set
- **Performance Issues**: Ensure testing with `--release` builds
- **Touch Not Working**: Test on real device, not emulator

### Getting Help

- Check SETUP.md for detailed setup instructions
- Review README.md for general information
- Open an issue with complete error messages and reproduction steps

## Security Considerations

- Never commit signing keystores to the repository (already gitignored)
- Keep GitHub secrets secure (keystore passwords, signing credentials)
- Review Android permissions in AndroidManifest.xml before adding new ones

## Additional Resources

- [Bevy Documentation](https://bevyengine.org/learn/)
- [Avian2D Documentation](https://docs.rs/avian2d/)
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Bevy Cheat Book](https://bevy-cheatbook.github.io/)
