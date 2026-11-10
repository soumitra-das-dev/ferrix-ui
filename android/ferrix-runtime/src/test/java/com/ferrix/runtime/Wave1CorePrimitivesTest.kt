package com.ferrix.runtime

import android.content.Context
import android.widget.CompoundButton
import android.widget.EditText
import android.widget.FrameLayout
import android.widget.ProgressBar
import androidx.appcompat.widget.SwitchCompat
import androidx.recyclerview.widget.RecyclerView
import androidx.swiperefreshlayout.widget.SwipeRefreshLayout
import androidx.test.core.app.ApplicationProvider
import com.ferrix.runtime.protocol.EventType
import com.ferrix.runtime.protocol.Patch
import com.ferrix.runtime.protocol.Property
import com.ferrix.runtime.protocol.ViewType
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertNotNull
import org.junit.Assert.assertTrue
import org.junit.Test
import org.junit.runner.RunWith
import org.robolectric.RobolectricTestRunner
import org.robolectric.annotation.Config

@RunWith(RobolectricTestRunner::class)
@Config(sdk = [34])
class Wave1CorePrimitivesTest {

    @Test
    fun testLazyListRecyclerViewVirtualization() {
        val context = ApplicationProvider.getApplicationContext<Context>()
        val rootLayout = FrameLayout(context)
        val renderer = FerrixRenderer(context, rootLayout)

        val patches = listOf(
            Patch.CreateNode(10L, ViewType.RECYCLER_VIEW),
            Patch.AppendChild(1L, 10L),
            Patch.SetProperty(10L, Property.ItemCount(10000)),
            Patch.SetProperty(10L, Property.ItemHeight(64f)),
            Patch.AttachEventListener(10L, EventType.END_REACHED)
        )

        var endReachedFired = false
        renderer.applyPatches(patches) { nodeId, eventType, _ ->
            if (nodeId == 10L && eventType == EventType.END_REACHED) {
                endReachedFired = true
            }
        }

        val rv = renderer.getView(10L) as? RecyclerView
        assertNotNull("RecyclerView must be created", rv)
        assertEquals(10000, rv?.adapter?.itemCount)
        assertFalse(endReachedFired)
    }

    @Test
    fun testTextInputControlledVersionLoop() {
        val context = ApplicationProvider.getApplicationContext<Context>()
        val rootLayout = FrameLayout(context)
        val renderer = FerrixRenderer(context, rootLayout)

        // 1. Initial controlled text version 1
        renderer.applyPatches(
            listOf(
                Patch.CreateNode(20L, ViewType.TEXT_INPUT),
                Patch.AppendChild(1L, 20L),
                Patch.SetProperty(20L, Property.ControlledText(1L, "Hello")),
                Patch.SetProperty(20L, Property.Placeholder("Enter name...")),
                Patch.SetProperty(20L, Property.SecureTextEntry(false))
            )
        ) { _, _, _ -> }

        val input = renderer.getView(20L) as? EditText
        assertNotNull(input)
        assertEquals("Hello", input?.text.toString())
        assertEquals("Enter name...", input?.hint)

        // Set cursor position to 3
        input?.setSelection(3)

        // 2. Incoming update with same version/text should not overwrite or jump cursor
        renderer.applyPatches(
            listOf(
                Patch.SetProperty(20L, Property.ControlledText(1L, "Hello"))
            )
        ) { _, _, _ -> }
        assertEquals(3, input?.selectionStart)

        // 3. Stale version 0 must be ignored
        renderer.applyPatches(
            listOf(
                Patch.SetProperty(20L, Property.ControlledText(0L, "Stale"))
            )
        ) { _, _, _ -> }
        assertEquals("Hello", input?.text.toString())

        // 4. Newer version 2 applies change
        renderer.applyPatches(
            listOf(
                Patch.SetProperty(20L, Property.ControlledText(2L, "Hello World"))
            )
        ) { _, _, _ -> }
        assertEquals("Hello World", input?.text.toString())
    }

    @Test
    fun testSwitchValueAndEvent() {
        val context = ApplicationProvider.getApplicationContext<Context>()
        val rootLayout = FrameLayout(context)
        val renderer = FerrixRenderer(context, rootLayout)

        var lastEventValue: Byte? = null

        renderer.applyPatches(
            listOf(
                Patch.CreateNode(30L, ViewType.SWITCH),
                Patch.AppendChild(1L, 30L),
                Patch.SetProperty(30L, Property.SwitchValue(true)),
                Patch.AttachEventListener(30L, EventType.VALUE_CHANGE)
            )
        ) { nodeId, eventType, payload ->
            if (nodeId == 30L && eventType == EventType.VALUE_CHANGE) {
                lastEventValue = payload?.firstOrNull()
            }
        }

        val switchView = renderer.getView(30L) as? SwitchCompat
        assertNotNull(switchView)
        assertTrue(switchView!!.isChecked)

        // Trigger change
        switchView.isChecked = false
        assertEquals(0.toByte(), lastEventValue)
    }

    @Test
    fun testRefreshControl() {
        val context = ApplicationProvider.getApplicationContext<Context>()
        val rootLayout = FrameLayout(context)
        val renderer = FerrixRenderer(context, rootLayout)

        var refreshTriggered = false

        renderer.applyPatches(
            listOf(
                Patch.CreateNode(40L, ViewType.REFRESH_CONTROL),
                Patch.AppendChild(1L, 40L),
                Patch.SetProperty(40L, Property.Refreshing(true)),
                Patch.AttachEventListener(40L, EventType.REFRESH)
            )
        ) { nodeId, eventType, _ ->
            if (nodeId == 40L && eventType == EventType.REFRESH) {
                refreshTriggered = true
            }
        }

        val refreshLayout = renderer.getView(40L) as? SwipeRefreshLayout
        assertNotNull(refreshLayout)
        assertTrue(refreshLayout!!.isRefreshing)
        assertFalse(refreshTriggered)
    }

    @Test
    fun testModalAndActivityIndicator() {
        val context = ApplicationProvider.getApplicationContext<Context>()
        val rootLayout = FrameLayout(context)
        val renderer = FerrixRenderer(context, rootLayout)

        renderer.applyPatches(
            listOf(
                Patch.CreateNode(50L, ViewType.MODAL),
                Patch.AppendChild(1L, 50L),
                Patch.SetProperty(50L, Property.ModalVisible(false)),
                Patch.CreateNode(51L, ViewType.ACTIVITY_INDICATOR),
                Patch.AppendChild(1L, 51L),
                Patch.SetProperty(51L, Property.Animating(true)),
                Patch.SetProperty(51L, Property.IndicatorColor(0xFF3B82F6.toInt()))
            )
        ) { _, _, _ -> }

        val modal = renderer.getView(50L)
        val indicator = renderer.getView(51L) as? ProgressBar

        assertNotNull(modal)
        assertNotNull(indicator)
        assertEquals(android.view.View.GONE, modal?.visibility)
        assertEquals(android.view.View.VISIBLE, indicator?.visibility)
    }
}
