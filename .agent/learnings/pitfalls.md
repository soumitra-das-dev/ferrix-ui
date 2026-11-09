# Pitfalls and Gotchas

### 2026-11-09 Initial Pitfalls Baseline
- **Context**: Setting up Rust JNI bridge with Android.
- **What happened / finding**: Panics unwinding across `extern "C"` JNI boundaries trigger undefined behavior or instant process abort.
- **Root cause**: Rust ABI does not specify unwinding across foreign C ABIs.
- **Rule going forward**: Every JNI entry point must wrap its implementation in `std::panic::catch_unwind` and return an error status code or throw a Java exception cleanly.
- **Evidence**: Rust documentation on FFI unwinding rules.
