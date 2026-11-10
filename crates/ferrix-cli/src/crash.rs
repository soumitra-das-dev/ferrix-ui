//! Android and Rust crash classification and diagnostic reporting.
//!
//! Parses logcat streams for crash signals, extracts Java and native stack traces,
//! and generates categorized diagnostic hints to guide developer remediation.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CrashKind {
    UnsatisfiedLinkError,
    RustPanic,
    ThemeMismatch,
    NativeSignal(String),
    StrictModeViolation,
    Anr,
    Unknown(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrashReport {
    pub kind: CrashKind,
    pub title: String,
    pub summary: String,
    pub stack_trace: Vec<String>,
    pub hint: String,
}

impl CrashReport {
    pub fn display_formatted(&self) -> String {
        let mut out = String::new();
        out.push_str("\n❌ \x1b[1;31mApp crashed:\x1b[0m ");
        out.push_str(&self.title);
        out.push('\n');

        if !self.summary.is_empty() {
            out.push_str(&format!("   \x1b[33m{}\x1b[0m\n", self.summary));
        }

        if !self.stack_trace.is_empty() {
            out.push_str("\n   \x1b[1mFiltered Stack Trace:\x1b[0m\n");
            for line in &self.stack_trace {
                out.push_str(&format!("     {}\n", line));
            }
        }

        out.push_str(&format!(
            "\n💡 \x1b[1;36mDiagnostic Hint:\x1b[0m {}\n",
            self.hint
        ));
        out
    }
}

/// Scans a collection of log lines for fatal exceptions, panics, or native crashes.
pub fn classify_crash(lines: &[String]) -> Option<CrashReport> {
    let mut fatal_index = None;
    let mut crash_kind = None;
    let mut summary = String::new();

    for (idx, line) in lines.iter().enumerate() {
        if line.contains("FATAL EXCEPTION") || line.contains("AndroidRuntime: FATAL") {
            fatal_index = Some(idx);
        } else if line.contains("UnsatisfiedLinkError") {
            crash_kind = Some(CrashKind::UnsatisfiedLinkError);
            summary = line.trim().to_string();
            if fatal_index.is_none() {
                fatal_index = Some(idx);
            }
        } else if line.contains("panicked at") || line.contains("fatal runtime error") {
            crash_kind = Some(CrashKind::RustPanic);
            summary = line.trim().to_string();
            if fatal_index.is_none() {
                fatal_index = Some(idx);
            }
        } else if line.contains("Theme.AppCompat") || line.contains("Theme.MaterialComponents") {
            crash_kind = Some(CrashKind::ThemeMismatch);
            summary = line.trim().to_string();
            if fatal_index.is_none() {
                fatal_index = Some(idx);
            }
        } else if line.contains("SIGSEGV") || line.contains("SIGABRT") {
            let sig = if line.contains("SIGSEGV") {
                "SIGSEGV"
            } else {
                "SIGABRT"
            };
            crash_kind = Some(CrashKind::NativeSignal(sig.to_string()));
            summary = line.trim().to_string();
            if fatal_index.is_none() {
                fatal_index = Some(idx);
            }
        } else if line.contains("StrictMode policy violation") {
            crash_kind = Some(CrashKind::StrictModeViolation);
            summary = line.trim().to_string();
            if fatal_index.is_none() {
                fatal_index = Some(idx);
            }
        } else if line.contains("ANR in") {
            crash_kind = Some(CrashKind::Anr);
            summary = line.trim().to_string();
            if fatal_index.is_none() {
                fatal_index = Some(idx);
            }
        }
    }

    let start_idx = fatal_index?;
    let mut stack_trace = Vec::new();

    for line in lines.iter().skip(start_idx).take(20) {
        let trimmed = line.trim();
        if trimmed.starts_with("at ")
            || trimmed.contains("Caused by:")
            || trimmed.contains("dlopen failed")
            || trimmed.contains("Exception")
            || trimmed.contains("Error:")
            || trimmed.contains("#00 pc")
            || trimmed.contains("#01 pc")
        {
            stack_trace.push(trimmed.to_string());
        }
    }

    let kind =
        crash_kind.unwrap_or_else(|| CrashKind::Unknown("Runtime exception occurred".to_string()));

    let (title, hint) = match &kind {
        CrashKind::UnsatisfiedLinkError => (
            "UnsatisfiedLinkError (Native library load failure)".to_string(),
            "The native .so library could not be loaded. Ensure: (1) 'cargo ndk' compiled the library for target device ABI ('arm64-v8a', 'x86_64'); (2) System.loadLibrary matches the crate name with hyphens converted to underscores (e.g. 'my_app' for 'my-app'); (3) if package has underscores ('my_app'), JNI export symbols use '_1' encoding.".to_string(),
        ),
        CrashKind::RustPanic => (
            "Rust Runtime Panic".to_string(),
            "A panic occurred in native Rust code crossing or approaching the JNI boundary. Ensure all JNI entry points wrap logic in std::panic::catch_unwind and do not unwrap() empty values.".to_string(),
        ),
        CrashKind::ThemeMismatch => (
            "Android Theme Mismatch".to_string(),
            "Activity requires an AppCompat or Material theme. Check AndroidManifest.xml and ensure android:theme is set to '@android:style/Theme.Material.Light.NoActionBar' or a valid AppCompat theme.".to_string(),
        ),
        CrashKind::NativeSignal(sig) => (
            format!("Native Crash ({})", sig),
            "A native segmentation fault or abort occurred. Check for null pointer dereferences, stack overflows, or JNI DirectByteBuffer boundary overruns.".to_string(),
        ),
        CrashKind::StrictModeViolation => (
            "StrictMode Violation".to_string(),
            "Disk or network I/O was detected on the Android main UI thread. Offload blocking operations to background coroutines or tokio async pools.".to_string(),
        ),
        CrashKind::Anr => (
            "Application Not Responding (ANR)".to_string(),
            "The main thread was blocked for >5 seconds. Ensure long-running Rust or Kotlin computation is dispatched to background threads.".to_string(),
        ),
        CrashKind::Unknown(msg) => (
            msg.clone(),
            "Inspect the stack trace above to identify the failing component.".to_string(),
        ),
    };

    Some(CrashReport {
        kind,
        title,
        summary,
        stack_trace,
        hint,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classify_unsatisfied_link_error() {
        let sample = vec![
            "11-10 17:00:48.217 31620 31620 D nativeloader: Load libmy-app.so dlopen failed: library \"libmy-app.so\" not found".to_string(),
            "11-10 17:00:48.219 31620 31620 E AndroidRuntime: FATAL EXCEPTION: main".to_string(),
            "11-10 17:00:48.219 31620 31620 E AndroidRuntime: java.lang.UnsatisfiedLinkError: dlopen failed: library \"libmy-app.so\" not found".to_string(),
            "11-10 17:00:48.219 31620 31620 E AndroidRuntime: \tat java.lang.Runtime.loadLibrary0(Runtime.java:1112)".to_string(),
            "11-10 17:00:48.219 31620 31620 E AndroidRuntime: \tat com.example.my_app.MainActivity.<clinit>(MainActivity.kt:10)".to_string(),
        ];

        let report = classify_crash(&sample).expect("Should classify crash");
        assert_eq!(report.kind, CrashKind::UnsatisfiedLinkError);
        assert!(report.hint.contains("cargo ndk"));
        assert!(report.hint.contains("underscores"));
        assert!(!report.stack_trace.is_empty());
    }

    #[test]
    fn test_classify_rust_panic() {
        let sample = vec![
            "11-10 17:00:48.219 31620 31620 E AndroidRuntime: FATAL EXCEPTION: main".to_string(),
            "thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', src/lib.rs:42:5".to_string(),
            "fatal runtime error: failed to initiate panic, error 5".to_string(),
        ];

        let report = classify_crash(&sample).expect("Should classify panic");
        assert_eq!(report.kind, CrashKind::RustPanic);
        assert!(report.hint.contains("catch_unwind"));
    }

    #[test]
    fn test_classify_theme_mismatch() {
        let sample = vec![
            "11-10 17:00:48.219 E AndroidRuntime: FATAL EXCEPTION: main".to_string(),
            "java.lang.IllegalStateException: You need to use a Theme.AppCompat theme (or descendant) with this activity.".to_string(),
            "at androidx.appcompat.app.AppCompatDelegateImpl.createSubDecor(AppCompatDelegateImpl.java:843)".to_string(),
        ];

        let report = classify_crash(&sample).expect("Should classify theme error");
        assert_eq!(report.kind, CrashKind::ThemeMismatch);
        assert!(report.hint.contains("AndroidManifest.xml"));
    }
}
