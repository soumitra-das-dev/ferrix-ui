# Verified Commands and Environment

### Rust Workspace Verification
```bash
cargo fmt --check
cargo clippy --workspace -- -D warnings
cargo test --workspace
```

### Android Cross-Compilation
```bash
cargo ndk -t arm64-v8a -t x86_64 -o android/ferrix-runtime/src/main/jniLibs build --release
```

### Gradle Build & Test
```bash
cd android && ./gradlew assembleDebug
cd android && ./gradlew test
```

### Ferrix Interactive Dev Server & Verification
```bash
# Start interactive Expo-style dev server
ferrix dev

# List connected devices & emulators
ferrix dev --list-devices

# Run non-interactive smoke test on device/emulator
ferrix dev --smoke --device emulator-5554

# Run full autonomous verification matrix
./scripts/verify_matrix.sh
```
