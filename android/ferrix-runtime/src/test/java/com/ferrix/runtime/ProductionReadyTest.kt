package com.ferrix.runtime

import android.content.Context
import android.os.Build
import android.widget.Button
import android.widget.EditText
import android.widget.ImageView
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
class ProductionReadyTest {

    companion object {
        @JvmStatic
        @BeforeClass
        fun loadNativeLibrary() {
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

    /**
     * Milestone 7 Gate Verification:
     * - TalkBack Accessibility: content descriptions and headings
     * - Text / IME: EditText bidirectional typing without loopback
     * - State Restoration: serialization across destruction and recreation
     */
    @Test
    fun testProductionAccessibilityTextImeAndStateRestoration() {
        val context = ApplicationProvider.getApplicationContext<Context>()
        val rootLayout = LinearLayout(context).apply {
            orientation = LinearLayout.VERTICAL
        }

        var bridge = FerrixBridge()
        bridge.start(context, rootLayout, "production")

        // 1. Accessibility Checks
        val header = rootLayout.getChildAt(0) as TextView
        val logo = rootLayout.getChildAt(1) as ImageView
        val input = rootLayout.getChildAt(2) as EditText
        val echoText = rootLayout.getChildAt(3) as TextView
        val counterBtn = rootLayout.getChildAt(4) as Button

        assertEquals("Ferrix Production Showcase heading", header.contentDescription.toString())
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.P) {
            assertTrue("Header must be marked as accessibility heading", header.isAccessibilityHeading)
        }
        assertEquals("Ferrix framework architecture logo", logo.contentDescription.toString())
        assertEquals("Interactive text input field", input.contentDescription.toString())
        assertEquals("Type here to test native IME...", input.hint.toString())

        // 2. Text / IME Input verification
        input.setText("Testing IME Input")
        assertEquals("Echo: Testing IME Input", echoText.text.toString())

        // 3. Counter button interaction
        assertEquals("Counter: 0", counterBtn.text.toString())
        counterBtn.performClick()
        assertEquals("Counter: 1", counterBtn.text.toString())

        // 4. Save state snapshot
        val snapshot = bridge.saveState()
        assertNotNull("Snapshot must not be null", snapshot)
        assertTrue("Snapshot must contain bytes", snapshot!!.isNotEmpty())

        // 5. Simulate activity destroy
        bridge.destroy()
        rootLayout.removeAllViews()

        // 6. Simulate activity recreation with state restoration
        bridge = FerrixBridge()
        bridge.start(context, rootLayout, "production")

        val restored = bridge.restoreState(snapshot)
        assertTrue("State restoration must succeed", restored)

        // 7. Verify restored hierarchy
        val restoredEcho = rootLayout.getChildAt(3) as TextView
        val restoredBtn = rootLayout.getChildAt(4) as Button

        assertEquals("Echo: Testing IME Input", restoredEcho.text.toString())
        assertEquals("Counter: 1", restoredBtn.text.toString())

        bridge.destroy()
    }
}

