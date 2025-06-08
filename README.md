## Local Testing Commands, with emulator or android device

```bash
# Install cross
cargo install cross --git https://github.com/cross-rs/cross

# Build everything
cross build --target aarch64-linux-android --release

# Check what was built
ls -la target/aarch64-linux-android/release/

# If you have Android device connected
adb push target/aarch64-linux-android/release/test_runner /data/local/tmp/
adb shell "cd /data/local/tmp && chmod +x test_runner && RUST_BACKTRACE=full ./test_runner"