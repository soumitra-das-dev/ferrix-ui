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
    fun testElementSystemColumnRowTextButtonRenderingAndReconciliation() {
        val context = ApplicationProvider.getApplicationContext<android.content.Context>()
        val rootLayout = LinearLayout(context).apply {
            orientation = LinearLayout.VERTICAL
        }

        val bridge = FerrixBridge()
        val renderer = bridge.start(context, rootLayout)

        // 1. Verify renderer node lookups (Nodes 1 through 7)
        for (id in 1L..7L) {
            assertNotNull("Expected node $id to exist in renderer", renderer.getView(id))
        }

        // 2. Verify root container (Column) has 4 direct children
        assertEquals(4, rootLayout.childCount)

        // Child 0: Title TextView
        val titleView = rootLayout.getChildAt(0)
        assertTrue("Expected TextView, got ${titleView.javaClass.simpleName}", titleView is TextView)
        assertEquals("Ferrix Element System (M2)", (titleView as TextView).text.toString())

        // Child 1: Subtitle TextView
        val subView = rootLayout.getChildAt(1)
        assertTrue("Expected TextView, got ${subView.javaClass.simpleName}", subView is TextView)
        assertEquals(
            "Demonstrating Column, Row, Text, and Button from Rust",
            (subView as TextView).text.toString()
        )

        // Child 2: Row (horizontal LinearLayout) containing 2 native Buttons
        val rowView = rootLayout.getChildAt(2)
        assertTrue("Expected LinearLayout for Row, got ${rowView.javaClass.simpleName}", rowView is LinearLayout)
        val rowLayout = rowView as LinearLayout
        assertEquals(LinearLayout.HORIZONTAL, rowLayout.orientation)
        assertEquals(2, rowLayout.childCount)

        val incBtn = rowLayout.getChildAt(0)
        assertTrue("Expected Button, got ${incBtn.javaClass.simpleName}", incBtn is Button)
        assertEquals("Increment", (incBtn as Button).text.toString())

        val resetBtn = rowLayout.getChildAt(1)
        assertTrue("Expected Button, got ${resetBtn.javaClass.simpleName}", resetBtn is Button)
        assertEquals("Reset", (resetBtn as Button).text.toString())

        // Child 3: Counter TextView
        val counterView = rootLayout.getChildAt(3)
        assertTrue("Expected TextView, got ${counterView.javaClass.simpleName}", counterView is TextView)
        val counterText = counterView as TextView
        assertEquals("Counter: 0", counterText.text.toString())

        // 3. User Interaction & Reconciliation: Click "Increment"
        incBtn.performClick()
        assertEquals("Counter: 1", counterText.text.toString())

        // Click "Increment" again
        incBtn.performClick()
        assertEquals("Counter: 2", counterText.text.toString())

        // Click "Reset"
        resetBtn.performClick()
        assertEquals("Counter: 0", counterText.text.toString())

        // Clean teardown
        bridge.destroy()
    }
}
