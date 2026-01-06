# Development Setup Guide

This guide covers setting up your development environment for building Sword Spinner on desktop and Android.

## Table of Contents
- [Desktop Development](#desktop-development)
- [Android Development](#android-development)
- [GitHub Actions Setup](#github-actions-setup)
- [Testing](#testing)

## Desktop Development

### Prerequisites

1. **Install Rust**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   ```

2. **System Dependencies**
   
   **Linux (Ubuntu/Debian):**
   ```bash
   sudo apt-get update
   sudo apt-get install -y \
       pkg-config \
       libx11-dev \
       libasound2-dev \
       libudev-dev \
       libxcb-render0-dev \
       libxcb-shape0-dev \
       libxcb-xfixes0-dev
   ```

   **macOS:**
   ```bash
   # No additional dependencies required
   ```

   **Windows:**
   - Install [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/)
   - Include "Desktop development with C++" workload

### Running the Game

```bash
# Clone the repository
git clone https://github.com/CraigBuilds/Sword-Spinner-Agent.git
cd Sword-Spinner-Agent

# Run in debug mode (faster compilation, slower runtime)
cargo run

# Run in release mode (slower compilation, faster runtime)
cargo run --release
```

### Development Tips

- Use `cargo check` for fast syntax checking
- Use `cargo clippy` for linting
- Use `cargo fmt` for code formatting
- Enable dynamic linking for faster iteration:
  ```bash
  # Add this to your shell profile for dev builds
  export BEVY_DYNAMIC_LINKING=true
  ```

## Android Development

### Prerequisites

1. **Install Rust** (see Desktop section above)

2. **Install Android NDK**

   **Option A: Using Android Studio**
   - Install [Android Studio](https://developer.android.com/studio)
   - Open SDK Manager (Tools → SDK Manager)
   - Go to SDK Tools tab
   - Install:
     - Android SDK Build-Tools
     - Android NDK (Side by side)
     - Android SDK Command-line Tools

   **Option B: Using Command Line**
   ```bash
   # Download and extract Android SDK command-line tools
   wget https://dl.google.com/android/repository/commandlinetools-linux-9477386_latest.zip
   unzip commandlinetools-linux-9477386_latest.zip -d ~/android-sdk
   
   # Set up environment
   export ANDROID_HOME=~/android-sdk
   export PATH=$PATH:$ANDROID_HOME/cmdline-tools/latest/bin
   export PATH=$PATH:$ANDROID_HOME/platform-tools
   
   # Install required packages
   sdkmanager --sdk_root=$ANDROID_HOME "platform-tools" "platforms;android-34" "build-tools;34.0.0" "ndk;26.0.10792818"
   ```

3. **Set Environment Variables**
   
   Add to your `~/.bashrc` or `~/.zshrc`:
   ```bash
   # Android SDK location
   export ANDROID_HOME=~/Android/Sdk  # Or your SDK path
   export ANDROID_NDK_ROOT=$ANDROID_HOME/ndk/26.0.10792818  # Or your NDK version
   
   # Add to PATH
   export PATH=$PATH:$ANDROID_HOME/platform-tools
   export PATH=$PATH:$ANDROID_HOME/build-tools/34.0.0
   ```

   Then reload:
   ```bash
   source ~/.bashrc  # or source ~/.zshrc
   ```

4. **Add Rust Android Targets**
   ```bash
   rustup target add aarch64-linux-android
   rustup target add armv7-linux-androideabi
   rustup target add x86_64-linux-android
   ```

5. **Install cargo-apk**
   ```bash
   cargo install cargo-apk
   ```

### Building for Android

```bash
# Build debug APK (faster, larger, with debug symbols)
cargo apk build

# Build release APK (slower, smaller, optimized)
cargo apk build --release

# Build for specific target
cargo apk build --target aarch64-linux-android --release
```

The APK will be in `target/release/apk/` or `target/debug/apk/`.

### Installing and Running

```bash
# Connect your Android device via USB with USB debugging enabled
# Or start an Android emulator

# Install and run the app
cargo apk run --release

# Or manually install the APK
adb install -r target/release/apk/sword-spinner.apk
```

### Testing on Device/Emulator

1. **Enable USB Debugging** on your Android device:
   - Go to Settings → About Phone
   - Tap "Build Number" 7 times
   - Go back to Settings → Developer Options
   - Enable "USB Debugging"

2. **Connect Device**:
   ```bash
   # Check device is connected
   adb devices
   ```

3. **View Logs**:
   ```bash
   # View application logs
   adb logcat | grep RustStdoutStderr
   
   # Or filter by package name
   adb logcat | grep sword-spinner
   ```

### Troubleshooting Android Build

**Issue: "No Android NDK found"**
```bash
# Make sure ANDROID_NDK_ROOT is set
echo $ANDROID_NDK_ROOT
# Should output something like: /home/user/Android/Sdk/ndk/26.0.10792818

# If empty, set it:
export ANDROID_NDK_ROOT=$ANDROID_HOME/ndk/$(ls $ANDROID_HOME/ndk)
```

**Issue: "linker not found"**
```bash
# Install the Android targets again
rustup target add aarch64-linux-android --force
```

**Issue: APK won't install**
```bash
# Uninstall previous version
adb uninstall com.swordspinner.game

# Then reinstall
cargo apk run --release
```

## GitHub Actions Setup

The Android release workflow is configured to automatically build and release unsigned APKs.

### Trigger a Release

The workflow can be triggered manually from the Actions tab:
- Go to: Actions → Android Release → Run workflow

The GitHub Actions workflow will:
1. Build the release APK
2. Create a GitHub release
3. Upload the unsigned APK

No additional setup or secrets are required.

## Testing

### Desktop Testing

```bash
# Run with keyboard controls
cargo run --release

# Test controls:
# - WASD: Move character
# - Spacebar: Spin sword
# - Arrow keys: Alternative movement
# - Left mouse: Alternative spin
```

### Android Testing Checklist

#### Touch Movement
- [ ] Drag smoothly moves character
- [ ] Character follows finger position
- [ ] No stuttering or lag
- [ ] Works in all areas of screen

#### Double-Tap Sword Spin
- [ ] Double-tap within 300ms spins sword
- [ ] Single tap does nothing
- [ ] Taps >50px apart don't trigger spin
- [ ] Triple-tap only spins once
- [ ] Works from any screen position
- [ ] No accidental spins during dragging

#### Combined Input
- [ ] Can drag and double-tap simultaneously
- [ ] Can spin multiple times with repeated double-taps
- [ ] Movement doesn't interrupt spin
- [ ] Spin doesn't interrupt movement

#### Performance
- [ ] Runs at 60 FPS on device
- [ ] No frame drops during gameplay
- [ ] Sword physics feel responsive
- [ ] Touch input has no noticeable lag

### Emulator Testing

```bash
# List available emulators
emulator -list-avds

# Start an emulator
emulator -avd <AVD_NAME> &

# Wait for boot, then install
cargo apk run --release
```

## Tips and Best Practices

### Performance Optimization

1. **Use Release Builds for Testing**
   - Debug builds are much slower
   - Always test performance with `--release`

2. **Monitor Frame Rate**
   - Add FPS counter if needed
   - Watch for frame drops during sword spins

3. **Test on Real Device**
   - Emulators don't reflect real touch latency
   - Test on the oldest device you want to support

### Touch Control Tuning

If double-tap detection needs adjustment, modify these constants in `src/main.rs`:

```rust
// In TouchState::default()
double_tap_window: Duration::from_millis(300),  // Time between taps
tap_distance_threshold: 50.0,                    // Distance between taps

// In detect_double_tap system
if distance > 10.0 {  // Drag threshold
    touch_state.is_dragging = true;
}
```

### Debugging Tips

```bash
# View all logs
adb logcat

# Clear logs
adb logcat -c

# Save logs to file
adb logcat > logfile.txt

# Filter by priority (E=Error, W=Warning, I=Info, D=Debug)
adb logcat *:E  # Only errors
```

## Common Issues

### "APK won't install on device"
- Check minimum SDK version (must be Android 12+)
- Uninstall previous version first
- Enable "Install from unknown sources" in developer settings

### "Touch input not working"
- Verify touchscreen feature in AndroidManifest.xml
- Check that touch events are being received (add debug logs)
- Test on real device, not emulator

### "Game runs slow"
- Make sure you're using `--release` build
- Check device specifications (need OpenGL ES 3.0+)
- Reduce number of obstacles if needed

### "Sword physics feel weird"
- Adjust mass ratios in `setup` function
- Tune damping values
- Modify spin force in `sword_spin` system
- Adjust joint compliance

## Resources

- [Bevy Documentation](https://bevyengine.org/learn/)
- [Avian2D Documentation](https://docs.rs/avian2d/)
- [cargo-apk Documentation](https://crates.io/crates/cargo-apk)
- [Android Developer Guide](https://developer.android.com/guide)
- [Rust Android Development](https://mozilla.github.io/firefox-browser-architecture/experiments/2017-09-21-rust-on-android.html)

## Getting Help

If you encounter issues:
1. Check this SETUP.md file
2. Review [README.md](README.md) for general information
3. Open an issue on GitHub with:
   - Your OS and version
   - Rust version (`rustc --version`)
   - Complete error message
   - Steps to reproduce
