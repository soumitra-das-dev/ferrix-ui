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
