package com.ferrix.runtime

import android.widget.Button
import android.widget.LinearLayout
import android.widget.TextView
import androidx.test.core.app.ApplicationProvider
import com.ferrix.runtime.protocol.Patch
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNotNull
import org.junit.Assert.assertSame
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
     * Milestone 4 Gate Verification:
     * `examples/counter/component.fsx` runs end-to-end; asserts:
     * 1. Exactly 1 JNI call per click.
     * 2. Exactly 1 SetText patch.
     * 3. Existing TextView instance reused.
     */
    @Test
    fun testGoldenCounterCompilerAotExecution() {
        val context = ApplicationProvider.getApplicationContext<android.content.Context>()
        val rootLayout = LinearLayout(context).apply {
            orientation = LinearLayout.VERTICAL
        }

        val bridge = FerrixBridge()
        bridge.start(context, rootLayout, "counter")

        // Initial Counter tree: Column with 3 children
        // Child 0: Text ("Ferrix Counter")
        // Child 1: Text ("Count: 0")
        // Child 2: Button ("Increment")
        assertEquals(3, rootLayout.childCount)

        val headerView = rootLayout.getChildAt(0) as TextView
        assertEquals("Ferrix Counter", headerView.text.toString())

        val counterTextView = rootLayout.getChildAt(1) as TextView
        assertEquals("Count: 0", counterTextView.text.toString())
        val initialTextViewIdentity = System.identityHashCode(counterTextView)

        val incrementBtn = rootLayout.getChildAt(2) as Button
        assertEquals("Increment", incrementBtn.text.toString())

        // --- FIRST CLICK ---
        val jniCallsBeforeClick1 = bridge.jniCallCount
        incrementBtn.performClick()

        val jniCallsAfterClick1 = bridge.jniCallCount - jniCallsBeforeClick1
        // Gate Assert 1: Exactly 1 JNI call per click
        assertEquals("Gate Requirement: Exactly 1 JNI call per click", 1, jniCallsAfterClick1)

        // Gate Assert 2: Exactly 1 SetText patch
        assertEquals("Gate Requirement: Exactly 1 patch emitted", 1, bridge.lastEmittedPatches.size)
        val patch1 = bridge.lastEmittedPatches[0]
        assertTrue("Gate Requirement: Patch must be SetText", patch1 is Patch.SetText)
        assertEquals("Count: 1", (patch1 as Patch.SetText).text)

        // Gate Assert 3: Existing TextView instance reused
        val currentTextView1 = rootLayout.getChildAt(1) as TextView
        assertEquals("Count: 1", currentTextView1.text.toString())
        assertSame("Gate Requirement: Existing TextView instance must be reused", counterTextView, currentTextView1)
        assertEquals("Gate Requirement: Identity hash code preserved", initialTextViewIdentity, System.identityHashCode(currentTextView1))

        // --- SECOND CLICK ---
        val jniCallsBeforeClick2 = bridge.jniCallCount
        incrementBtn.performClick()

        val jniCallsAfterClick2 = bridge.jniCallCount - jniCallsBeforeClick2
        assertEquals(1, jniCallsAfterClick2)
        assertEquals(1, bridge.lastEmittedPatches.size)
        val patch2 = bridge.lastEmittedPatches[0]
        assertTrue(patch2 is Patch.SetText)
        assertEquals("Count: 2", (patch2 as Patch.SetText).text)

        val currentTextView2 = rootLayout.getChildAt(1) as TextView
        assertEquals("Count: 2", currentTextView2.text.toString())
        assertSame(counterTextView, currentTextView2)

        bridge.destroy()
    }

    @Test
    fun testReactiveTodoAppLifecycleAndKeyedReconciliation() {
        val context = ApplicationProvider.getApplicationContext<android.content.Context>()
        val rootLayout = LinearLayout(context).apply {
            orientation = LinearLayout.VERTICAL
        }

        val bridge = FerrixBridge()
        bridge.start(context, rootLayout, "todo")

        // 1. Initial State: Title + Add Button + 2 Todo Rows = 4 children
        assertEquals(4, rootLayout.childCount)

        val titleView = rootLayout.getChildAt(0) as TextView
        assertEquals("Ferrix Todo App (M3)", titleView.text.toString())

        val addBtn = rootLayout.getChildAt(1) as Button
        assertEquals("+ Add Task", addBtn.text.toString())

        // Row 1 (Todo Item 1)
        val row1 = rootLayout.getChildAt(2) as LinearLayout
        val text1 = row1.getChildAt(0) as TextView
        val delete1 = row1.getChildAt(2) as Button
        assertEquals("[x] Explore Ferrix Architecture", text1.text.toString())

        // Row 2 (Todo Item 2)
        val row2 = rootLayout.getChildAt(3) as LinearLayout
        val text2 = row2.getChildAt(0) as TextView
        val toggle2 = row2.getChildAt(1) as Button
        assertEquals("[ ] Build Reactive Runtime", text2.text.toString())

        // 2. Add new task via "+ Add Task"
        addBtn.performClick()
        assertEquals(5, rootLayout.childCount)
        val row3 = rootLayout.getChildAt(4) as LinearLayout
        val text3 = row3.getChildAt(0) as TextView
        assertEquals("[ ] New Ferrix Task #3", text3.text.toString())

        // 3. Toggle completion of Todo Item 2
        toggle2.performClick()
        assertEquals("[x] Build Reactive Runtime", text2.text.toString())

        // 4. Delete Todo Item 1
        delete1.performClick()
        // Item 1 is removed, child count reduces to 4
        assertEquals(4, rootLayout.childCount)

        // Verify remaining items:
        // Child 0: Title, Child 1: Add Button, Child 2: Item 2, Child 3: Item 3
        val remainingRow1 = rootLayout.getChildAt(2) as LinearLayout
        val remainingText1 = remainingRow1.getChildAt(0) as TextView
        assertEquals("[x] Build Reactive Runtime", remainingText1.text.toString())

        val remainingRow2 = rootLayout.getChildAt(3) as LinearLayout
        val remainingText2 = remainingRow2.getChildAt(0) as TextView
        assertEquals("[ ] New Ferrix Task #3", remainingText2.text.toString())

        // Clean teardown
        bridge.destroy()
    }
}
