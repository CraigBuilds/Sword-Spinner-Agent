# Development Setup Guide

Quick setup for building Sword Spinner on desktop and Android.

## Desktop Development

### Prerequisites

**Install Rust**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**System Dependencies**

Linux (Ubuntu/Debian):
```bash
sudo apt-get install pkg-config libx11-dev libasound2-dev libudev-dev
```

macOS: No additional dependencies

Windows: Install [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/) with "Desktop development with C++"

### Running

```bash
git clone https://github.com/CraigBuilds/Sword-Spinner-Agent.git
cd Sword-Spinner-Agent
cargo run              # Debug mode
cargo run --release    # Release mode
```

### Development Commands

```bash
cargo check    # Fast syntax checking
cargo clippy   # Linting
cargo fmt      # Format code
```

## Android Development

### Prerequisites

1. **Install Android NDK** via [Android Studio](https://developer.android.com/studio):
   - Open SDK Manager (Tools → SDK Manager)
   - Install: Android SDK Build-Tools, Android NDK, SDK Command-line Tools

2. **Set Environment Variables** (add to `~/.bashrc` or `~/.zshrc`):
   ```bash
   export ANDROID_HOME=~/Android/Sdk
   export ANDROID_NDK_ROOT=$ANDROID_HOME/ndk/26.0.10792818
   export PATH=$PATH:$ANDROID_HOME/platform-tools
   ```

3. **Add Rust Targets**
   ```bash
   rustup target add aarch64-linux-android armv7-linux-androideabi
   ```

4. **Install cargo-apk**
   ```bash
   cargo install cargo-apk
   ```

### Building and Running

```bash
cargo apk build --release              # Build APK
cargo apk run --release                # Install and run on device
adb logcat | grep RustStdoutStderr     # View logs
```

### Device Setup

1. Enable USB Debugging:
   - Settings → About Phone → Tap "Build Number" 7 times
   - Settings → Developer Options → Enable "USB Debugging"

2. Connect device and verify: `adb devices`

## Troubleshooting

**"No Android NDK found"**
```bash
export ANDROID_NDK_ROOT=$ANDROID_HOME/ndk/$(ls $ANDROID_HOME/ndk)
```

**APK won't install**
```bash
adb uninstall com.swordspinner.game
cargo apk run --release
```

**Slow performance**
- Always test with `--release` builds
- Debug builds are significantly slower

## CI/CD with GitHub Actions

The project uses GitHub Actions to build Android APKs automatically. The workflow installs the Android platform SDK that matches the `target_sdk_version` in `Cargo.toml`:

```yaml
- name: Setup Android SDK
  uses: android-actions/setup-android@v3
  with:
    packages: 'platforms;android-34 build-tools;34.0.0'
```

This ensures cargo-apk has the correct platform SDK to compile against. Releases use a debug keystore for testing (not suitable for Play Store).

## Resources

- [Bevy Documentation](https://bevyengine.org/learn/)
- [Avian2D Documentation](https://docs.rs/avian2d/)
- [cargo-apk Documentation](https://crates.io/crates/cargo-apk)
- [Android Developer Guide](https://developer.android.com/guide)
