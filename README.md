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
- **Movement**: Use the virtual joystick (appears anywhere you touch the screen)
  - Touch and drag anywhere to move
  - Floating joystick appears where you touch for intuitive control
  - Visual feedback shows your input direction
- **Spin Sword**: Tap the "SPIN" button at the bottom center of the screen
  - Simple and reliable
  - No accidental triggers
  - Easy to use while moving with the joystick

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
- **Touch Handling**: Bevy virtual_joystick for mobile controls with UI button for sword spinning

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

1. **Joystick movement**: Touch and drag anywhere on screen - character should follow smoothly
2. **Spin button**: Tap the "SPIN" button at bottom center - sword should spin
3. **Simultaneous controls**: Move with joystick while tapping spin button - both should work
4. **Button visibility**: Ensure the spin button is clearly visible and accessible

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Acknowledgments

- Built with [Bevy](https://bevyengine.org/) game engine
- Physics powered by [Avian2D](https://github.com/Jondolf/avian)
- Touch controls provided by [virtual_joystick](https://github.com/SergioRibera/virtual_joystick)
- Inspired by classic top-down arcade games
