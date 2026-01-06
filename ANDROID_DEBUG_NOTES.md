# Android Loading Issue - Debug Notes

## Issue
The Android app gets stuck on an empty screen after installation and launch.

## Changes Made (2026-01-06)

### 1. Removed Asset Dependencies
- **Removed:** `bevy_embedded_assets` plugin
- **Removed:** Image assets (Knob.png, Outline.png) from virtual joystick
- **Reason:** Asset loading is a common cause of Android app hangs:
  - File path issues on Android (different file system structure)
  - Asset packing issues within APK
  - Async asset loading potentially blocking main thread
  - `bevy_embedded_assets` may have compatibility issues with Bevy 0.15

### 2. Simplified Input System
- **Removed:** `virtual_joystick` dependency (third-party crate)
- **Added:** Simple TouchState resource system
- **Reason:** Third-party plugins can have Android-specific issues:
  - Touch event handling differences
  - Plugin initialization timing problems
  - Resource dependency issues

### 3. Added Unit Tests
- Created tests that verify app initialization and entity creation
- Tests use MinimalPlugins to avoid window/rendering dependencies
- All tests passing

## Current State
The app now has:
- ✅ No external asset loading
- ✅ Simplified touch input system (placeholder implementation)
- ✅ Working unit tests
- ✅ Clean compilation

The app should now at least start on Android, showing the basic game elements (player circle, sword, walls, obstacles, spin button) without the touch joystick.

## Next Steps

### If App Now Loads on Android:
1. Re-add touch controls incrementally
2. Test each addition on Android device
3. Consider implementing custom joystick without external library
4. Add back assets using proper Android asset loading

### If App Still Doesn't Load:
Investigate:
1. Check Android logs: `adb logcat | grep RustStdoutStderr`
2. Verify window creation on Android
3. Check if physics engine (Avian2D) initializes properly
4. Test with even more minimal setup (remove physics temporarily)

## Suspected Root Cause
Most likely: **Asset loading blocking main thread**
- The original code loaded Knob.png and Outline.png through bevy_embedded_assets
- If asset loading hung or failed, the entire app would freeze
- Android's asset system is different from desktop, requiring special handling

## Testing Recommendations
1. Build and install APK: `cargo apk build --release && cargo apk run --release`
2. Check logs immediately: `adb logcat -c && adb logcat | grep -i rust`
3. If it loads, player should be able to:
   - See blue circle (player) and gray sword
   - Move with WASD (on desktop) but no touch controls yet
   - Press SPIN button to spin sword
   - See walls and obstacles

## Future: Proper Touch Controls
If the app loads successfully, implement touch controls properly:
1. Use Bevy's built-in `Touches` resource
2. Create visual joystick using colored UI nodes (no images needed)
3. Implement touch delta tracking for movement
4. Test incrementally on Android after each change
