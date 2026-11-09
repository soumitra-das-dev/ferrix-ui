package com.ferrix.runtime

import android.widget.Button
import android.widget.LinearLayout
import android.widget.TextView
import androidx.test.core.app.ApplicationProvider
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNotNull
import org.junit.Assert.assertTrue
import org.junit.BeforeClass
import org.junit.Test
import org.junit.runner.RunWith
import org.robolectric.RobolectricTestRunner
import org.robolectric.annotation.Config
import java.io.File

@RunWith(RobolectricTestRunner::class)
@Config(sdk = [34])
class FerrixE2ETest {

    companion object {
        @JvmStatic
        @BeforeClass
        fun loadNativeLibrary() {
            // Locate the compiled host libferrix_android.so
            val candidatePaths = listOf(
                "../../target/debug/libferrix_android.so",
                "target/debug/libferrix_android.so",
                "../target/debug/libferrix_android.so"
            )
            val soFile = candidatePaths
                .map { File(it).canonicalFile }
                .firstOrNull { it.exists() }

            assertNotNull("Could not find host libferrix_android.so", soFile)
            System.load(soFile!!.absolutePath)
        }
    }

    @Test
    fun testEndToEndNativeTextViewRenderAndEventMutation() {
        val context = ApplicationProvider.getApplicationContext<android.content.Context>()
        val rootLayout = LinearLayout(context).apply {
            orientation = LinearLayout.VERTICAL
        }

        val bridge = FerrixBridge()
        val renderer = bridge.start(context, rootLayout)

        // Verify renderer node lookup
        assertNotNull(renderer.getView(1L))
        assertNotNull(renderer.getView(2L))
        assertNotNull(renderer.getView(3L))

        // Verify root container has 2 native children attached from Rust
        assertEquals(2, rootLayout.childCount)

        // Child 0 must be native TextView rendered from Rust
        val textNode = rootLayout.getChildAt(0)
        assertTrue("Expected TextView, got ${textNode.javaClass.simpleName}", textNode is TextView)
        val textView = textNode as TextView
        assertEquals("Hello from Ferrix Native Rust!", textView.text.toString())

        // Child 1 must be interactive native Button
        val buttonNode = rootLayout.getChildAt(1)
        assertTrue("Expected Button, got ${buttonNode.javaClass.simpleName}", buttonNode is Button)
        val button = buttonNode as Button
        assertEquals("Tap to mutate from Rust", button.text.toString())

        // Simulate user clicking the button -> dispatches JNI event to Rust
        button.performClick()

        // Verify reactive state update returned from Rust mutated the TextView
        assertEquals("Rust Reactive Count: 1", textView.text.toString())

        // Click again -> count increments to 2
        button.performClick()
        assertEquals("Rust Reactive Count: 2", textView.text.toString())

        // Clean teardown
        bridge.destroy()
    }
}
