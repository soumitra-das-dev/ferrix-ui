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
    fun testReactiveTodoAppLifecycleAndKeyedReconciliation() {
        val context = ApplicationProvider.getApplicationContext<android.content.Context>()
        val rootLayout = LinearLayout(context).apply {
            orientation = LinearLayout.VERTICAL
        }

        val bridge = FerrixBridge()
        val renderer = bridge.start(context, rootLayout)

        // 1. Initial State: Title + Add Button + 2 Todo Rows = 4 children
        assertEquals(4, rootLayout.childCount)

        val titleView = rootLayout.getChildAt(0) as TextView
        assertEquals("Ferrix Todo App (M3)", titleView.text.toString())

        val addBtn = rootLayout.getChildAt(1) as Button
        assertEquals("+ Add Task", addBtn.text.toString())

        // Row 1 (Todo Item 1)
        val row1 = rootLayout.getChildAt(2) as LinearLayout
        val text1 = row1.getChildAt(0) as TextView
        val toggle1 = row1.getChildAt(1) as Button
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
