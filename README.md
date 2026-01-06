# Sword Spinner

A top-down arcade game built with Rust, Bevy 0.15, and Avian2D physics. Control a character and spin a physics-based sword to knock around obstacles.

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/rust-stable-orange.svg)
![Platform](https://img.shields.io/badge/platform-desktop%20%7C%20android-green.svg)

## Controls

**Desktop**
- Movement: WASD or Arrow Keys
- Spin Sword: Spacebar or Left Mouse Button

**Mobile**
- Movement: Touch joystick (appears anywhere on screen)
- Spin Sword: "SPIN" button at bottom center

## Quick Start

**Desktop**
```bash
git clone https://github.com/CraigBuilds/Sword-Spinner-Agent.git
cd Sword-Spinner-Agent
cargo run
```

**Android**
```bash
cargo install cargo-apk
rustup target add aarch64-linux-android
cargo apk build --release
cargo apk run --release
```

See [SETUP.md](SETUP.md) for detailed setup instructions.

## Technical Stack

- **Engine**: Bevy 0.15 (ECS architecture)
- **Physics**: Avian2D (2D rigid bodies, revolute joints)
- **Platforms**: Desktop (Windows, macOS, Linux), Android (SDK 24+)

## Project Structure

```
src/
  lib.rs               # Game implementation
  main.rs              # Entry point
Cargo.toml             # Dependencies & Android metadata
AndroidManifest.xml    # Android configuration
```

## License

MIT License - see [LICENSE](LICENSE) for details.

## Resources

- [Bevy](https://bevyengine.org/) - Game engine
- [Avian2D](https://github.com/Jondolf/avian) - Physics engine
- [virtual_joystick](https://github.com/SergioRibera/virtual_joystick) - Touch controls
