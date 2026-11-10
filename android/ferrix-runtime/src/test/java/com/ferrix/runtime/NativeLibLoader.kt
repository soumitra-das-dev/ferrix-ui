package com.ferrix.runtime

import org.junit.Assume
import java.io.File

/**
 * Shared loader for the host JNI library `libferrix_android.so` during Robolectric tests.
 */
object NativeLibLoader {
    private var isLoaded = false

    @Synchronized
    fun ensureLoaded() {
        if (isLoaded) return

        val candidatePaths = listOf(
            "../../target/debug/libferrix_android.so",
            "target/debug/libferrix_android.so",
            "../target/debug/libferrix_android.so",
            "../../target/release/libferrix_android.so",
            "../target/release/libferrix_android.so"
        )

        val soFile = candidatePaths
            .map { File(it).canonicalFile }
            .firstOrNull { it.exists() }

        if (soFile != null) {
            try {
                System.load(soFile.absolutePath)
                isLoaded = true
                return
            } catch (e: Throwable) {
                System.err.println("Failed to load native library from ${soFile.absolutePath}: ${e.message}")
            }
        }

        try {
            System.loadLibrary("ferrix_android")
            isLoaded = true
            return
        } catch (_: UnsatisfiedLinkError) {
            // Not in java.library.path
        }

        Assume.assumeTrue(
            "Skipping test: libferrix_android.so not found on host. Run 'cargo build -p ferrix-android' to enable JNI tests.",
            false
        )
    }
}
