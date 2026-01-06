# Sword Spinner

A top-down arcade game built with Rust, Bevy 0.15, and Avian2D physics. Control a character that moves around an arena and spins a physics-based sword to knock around obstacles.

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/rust-stable-orange.svg)
![Platform](https://img.shields.io/badge/platform-desktop%20%7C%20android-green.svg)

## Features

- **Physics-based gameplay**: Sword attached via revolute joint for realistic momentum
- **Dual control schemes**: 
  - Desktop: Keyboard/mouse controls
  - Mobile: Touch drag + double-tap mechanics
- **Cross-platform**: Runs on desktop (Windows, macOS, Linux) and Android
- **Dynamic obstacles**: Interactive physics objects that respond to sword collisions
- **Smooth camera follow**: Camera tracks player movement
- **Optimized builds**: Release builds with LTO and optimizations

## Gameplay

### Objective
Move around the arena and use your spinning sword to knock physics objects around!

### Controls

#### Desktop
- **Movement**: WASD or Arrow Keys
- **Spin Sword**: Spacebar or Left Mouse Button

#### Mobile (Android)
- **Movement**: Touch and drag anywhere on screen - your character follows your finger
- **Spin Sword**: Double-tap anywhere on screen (like Instagram likes!)
  - Must tap twice within 300ms
  - Taps must be within 50 pixels of each other
  - Works from any screen position
  - Won't trigger accidentally during dragging

### Double-Tap Design
The double-tap mechanic was chosen because:
- ✅ Allows simultaneous movement and spinning (drag + tap)
- ✅ Prevents accidental spins during normal gameplay
- ✅ Feels intentional and satisfying (like a "power move")
- ✅ Familiar gesture from social media apps
- ✅ Works with one or two hands
- ✅ Doesn't interfere with drag-to-move controls

## Building

### Desktop

Requirements:
- Rust (latest stable)

```bash
# Clone the repository
git clone https://github.com/CraigBuilds/Sword-Spinner-Agent.git
cd Sword-Spinner-Agent

# Run in development mode
cargo run

# Build release version
cargo build --release
```

### Android

Requirements:
- Rust (latest stable)
- Android NDK
- cargo-apk

```bash
# Install cargo-apk
cargo install cargo-apk

# Add Android targets
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi

# Build debug APK
cargo apk build

# Build release APK
cargo apk build --release

# Install on connected device
cargo apk run --release
```

For detailed Android setup instructions, see [SETUP.md](SETUP.md).

## CI/CD Pipeline

The project includes a GitHub Actions workflow that automatically:
- Builds release APKs for Android (aarch64)
- Creates GitHub releases with APK attachments
- Uploads APKs for distribution

### Releasing

The workflow can be triggered manually from the Actions tab:
1. Go to: Actions → Android Release → Run workflow
2. The workflow automatically builds and releases a signed APK

**Note**: Release APKs are signed with a debug keystore (using standard Android debug credentials). This allows the APK to be installed on any device for testing. For production Play Store releases, a proper production keystore would be required.

## Technical Details

### Architecture
- **Engine**: Bevy 0.15 with ECS architecture
- **Physics**: Avian2D for 2D rigid body physics
- **Joint System**: RevoluteJoint connects sword to player
- **Touch Handling**: Custom TouchState resource for gesture detection

### Physics Configuration
- Zero gravity (top-down game)
- Player rotation locked (LockedAxes)
- Damping applied to prevent endless motion
- Mass ratios: Player (2.0), Sword (0.5), Obstacles (1.0)
- Low compliance joint for stiff sword connection

### Performance
- Thin LTO enabled for release builds
- Single codegen unit for maximum optimization
- Optimized Bevy features for mobile
- Efficient touch event processing

## Project Structure

```
sword-spinner/
├── .github/
│   └── workflows/
│       └── android-release.yml    # CI/CD pipeline
├── src/
│   └── main.rs                    # Game implementation
├── Cargo.toml                     # Dependencies and Android metadata
├── AndroidManifest.xml            # Android configuration
├── README.md                      # This file
├── SETUP.md                       # Development setup guide
├── LICENSE                        # MIT license
└── .gitignore                     # Git ignore rules
```

## Testing Touch Controls

To test touch controls on Android:

1. **Single tap**: Should not spin sword (test for false positives)
2. **Double-tap**: Tap twice quickly (~200ms apart) - should spin sword
3. **Drag**: Touch and drag - character should follow smoothly
4. **Drag + double-tap**: While dragging, double-tap with another finger - both should work
5. **Distant taps**: Tap in different corners - should not trigger spin (>50px apart)
6. **Triple-tap**: Three quick taps - should only spin once (on second tap)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### For AI Agents and Contributors

**Before submitting any code changes, you MUST:**

1. **Run cargo check**: Ensures code compiles without errors
   ```bash
   cargo check --all-targets
   ```

2. **Run cargo clippy**: Catches common mistakes and non-idiomatic code
   ```bash
   cargo clippy --all-targets -- -D warnings
   ```

3. **Format code with rustfmt**: Ensures consistent code style
   ```bash
   cargo fmt
   ```

4. **Run tests**: Verify functionality works as expected
   ```bash
   cargo test --lib
   ```

5. **Build the project**: Confirm it compiles successfully
   ```bash
   cargo build
   ```

**Additional Guidelines:**
- Follow existing code patterns and conventions in the codebase
- Add comments for complex logic
- Update documentation (README.md, SETUP.md) if adding features
- Test Android builds if making platform-specific changes: `cargo apk build --lib`
- Don't introduce new compiler warnings
- Avoid adding unnecessary dependencies
- Review the PR template checklist before submitting

**For AI Agents:** Run these checks during your development process to catch and fix issues immediately, before finalizing your changes. This ensures high code quality without relying on CI gates.

## Acknowledgments

- Built with [Bevy](https://bevyengine.org/) game engine
- Physics powered by [Avian2D](https://github.com/Jondolf/avian)
- Inspired by classic top-down arcade games
