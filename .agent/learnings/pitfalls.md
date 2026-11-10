# Pitfalls and Gotchas

### 2026-11-09 Initial Pitfalls Baseline
- **Context**: Setting up Rust JNI bridge with Android.
- **What happened / finding**: Panics unwinding across `extern "C"` JNI boundaries trigger undefined behavior or instant process abort.
- **Root cause**: Rust ABI does not specify unwinding across foreign C ABIs.
- **Rule going forward**: Every JNI entry point must wrap its implementation in `std::panic::catch_unwind` and return an error status code or throw a Java exception cleanly.
- **Evidence**: Rust documentation on FFI unwinding rules.

### 2026-11-10 Rust Shared Object Hyphens vs. Android System.loadLibrary
- **Context**: Loading native library in generated Android project.
- **What happened / finding**: `System.loadLibrary("my-app")` throws `UnsatisfiedLinkError: dlopen failed: library "libmy-app.so" not found`.
- **Root cause**: Rust/Cargo converts package names with hyphens to underscores for cdylibs (`libmy_app.so`). Android's `System.loadLibrary` does not normalize hyphens to underscores.
- **Rule going forward**: Always set `[lib] name = "{pkg_name}"` in `Cargo.toml` and call `System.loadLibrary("{pkg_name}")` where `pkg_name` has hyphens replaced with underscores.
- **Evidence**: Crash logs on Motorola Moto G34 5G.

### 2026-11-10 JNI Symbol Mangling with Underscores
- **Context**: Calling JNI native methods from package names containing underscores (`com.example.my_app`).
- **What happened / finding**: JNI symbol resolution fails with `UnsatisfiedLinkError: No implementation found for...`.
- **Root cause**: JNI specification dictates that `_` in Java identifiers must be escaped as `_1` in C symbol exports (`com_example_my_1app`).
- **Rule going forward**: Use `mangle_jni_symbol` or adhere strictly to `_1` escaping for any package or class containing underscores.
- **Evidence**: Oracle Java Native Interface Specification, Chapter 2.
