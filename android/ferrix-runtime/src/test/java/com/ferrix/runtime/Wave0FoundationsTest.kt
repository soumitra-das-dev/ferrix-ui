package com.ferrix.runtime

import android.content.Context
import android.view.MotionEvent
import android.view.ViewGroup
import android.widget.EditText
import android.widget.FrameLayout
import android.widget.LinearLayout
import android.widget.TextView
import androidx.test.core.app.ApplicationProvider
import com.ferrix.runtime.protocol.Command
import com.ferrix.runtime.protocol.CommandDecoder
import com.ferrix.runtime.protocol.EventType
import com.ferrix.runtime.protocol.Patch
import com.ferrix.runtime.protocol.Property
import com.ferrix.runtime.protocol.TextMeasureRequest
import com.ferrix.runtime.protocol.ViewType
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNotNull
import org.junit.Assert.assertSame
import org.junit.Assert.assertTrue
import org.junit.Test
import org.junit.runner.RunWith
import org.robolectric.RobolectricTestRunner
import org.robolectric.annotation.Config

@RunWith(RobolectricTestRunner::class)
@Config(sdk = [34])
class Wave0FoundationsTest {

    @Test
    fun testViewAndTextStylingWithLayoutFrames() {
        val context = ApplicationProvider.getApplicationContext<Context>()
        val rootLayout = FrameLayout(context)
        val renderer = FerrixRenderer(context, rootLayout)

        val patches = listOf(
            Patch.CreateNode(2L, ViewType.CONTAINER),
            Patch.AppendChild(1L, 2L),
            Patch.SetLayout(2L, 16f, 24f, 320f, 100f),
            Patch.SetProperty(2L, Property.BackgroundColor(0xFF2563EB.toInt())),
            Patch.SetProperty(2L, Property.Opacity(0.9f)),
            Patch.SetProperty(2L, Property.Elevation(4f)),
            Patch.SetProperty(2L, Property.BorderRadius(8f, 8f, 8f, 8f)),
            Patch.SetProperty(2L, Property.TestId("hero_box")),
            Patch.CreateNode(3L, ViewType.TEXT),
            Patch.AppendChild(2L, 3L),
            Patch.SetText(3L, "Ferrix Architecture A"),
            Patch.SetProperty(3L, Property.FontSize(18f)),
            Patch.SetProperty(3L, Property.FontWeight(700)),
            Patch.SetProperty(3L, Property.TextColor(0xFFFFFFFF.toInt())),
            Patch.SetProperty(3L, Property.TestId("hero_title"))
        )

        renderer.applyPatches(patches) { _, _, _ -> }

        // Assert Container View
        val container = renderer.getView(2L)
        assertNotNull(container)
        assertEquals("hero_box", container?.tag)
        assertEquals(0.9f, container?.alpha ?: 0f, 0.01f)
        assertTrue(container is FrameLayout)

        // Assert Child Text
        val text = renderer.getView(3L) as? TextView
        assertNotNull(text)
        assertEquals("hero_title", text?.tag)
        assertEquals("Ferrix Architecture A", text?.text.toString())
        assertEquals(0xFFFFFFFF.toInt(), text?.currentTextColor)
    }

    @Test
    fun testPressableInteractiveTouchAndEvents() {
        val context = ApplicationProvider.getApplicationContext<Context>()
        val rootLayout = FrameLayout(context)
        val renderer = FerrixRenderer(context, rootLayout)

        val receivedEvents = ArrayList<EventType>()

        val patches = listOf(
            Patch.CreateNode(10L, ViewType.PRESSABLE),
            Patch.AppendChild(1L, 10L),
            Patch.SetProperty(10L, Property.TestId("btn_pressable")),
            Patch.SetProperty(10L, Property.RippleColor(0x33000000)),
            Patch.AttachEventListener(10L, EventType.CLICK),
            Patch.AttachEventListener(10L, EventType.PRESS_IN),
            Patch.AttachEventListener(10L, EventType.PRESS_OUT)
        )

        renderer.applyPatches(patches) { nodeId, eventType, _ ->
            if (nodeId == 10L) {
                receivedEvents.add(eventType)
            }
        }

        val pressableView = renderer.getView(10L)
        assertNotNull(pressableView)
        assertEquals("btn_pressable", pressableView?.tag)

        // Simulate touch down and up
        val downEvent = MotionEvent.obtain(0L, 0L, MotionEvent.ACTION_DOWN, 10f, 10f, 0)
        pressableView?.dispatchTouchEvent(downEvent)

        val upEvent = MotionEvent.obtain(0L, 10L, MotionEvent.ACTION_UP, 10f, 10f, 0)
        pressableView?.dispatchTouchEvent(upEvent)

        // Perform click
        pressableView?.performClick()

        assertTrue("Should receive PRESS_IN event", receivedEvents.contains(EventType.PRESS_IN))
        assertTrue("Should receive PRESS_OUT event", receivedEvents.contains(EventType.PRESS_OUT))
        assertTrue("Should receive CLICK event", receivedEvents.contains(EventType.CLICK))
    }

    @Test
    fun testStaticLayoutTextMeasurementBatch() {
        val context = ApplicationProvider.getApplicationContext<Context>()
        val measurer = StaticLayoutTextMeasurer(context)

        val requests = listOf(
            TextMeasureRequest(
                requestId = 1,
                text = "Short heading",
                fontSizeSp = 24f,
                fontWeight = 700,
                fontStyle = 0,
                maxWidth = 360f
            ),
            TextMeasureRequest(
                requestId = 2,
                text = "A longer paragraph of text designed to test line wrapping under constrained width in Ferrix.",
                fontSizeSp = 16f,
                fontWeight = 400,
                fontStyle = 0,
                maxWidth = 200f
            )
        )

        val responses = measurer.measureBatch(requests)
        assertEquals(2, responses.size)

        val resp1 = responses[0]
        assertEquals(1, resp1.requestId)
        assertTrue("Measured width must be positive", resp1.width > 0f)
        assertTrue("Measured height must be positive", resp1.height > 0f)

        val resp2 = responses[1]
        assertEquals(2, resp2.requestId)
        assertTrue("Constrained text width must not exceed max width", resp2.width <= 200f)
        assertTrue("Multi-line text height must be greater than single line", resp2.height > resp1.height * 0.8f)

        // Verify encode/decode round trip with CommandDecoder
        val encodedBuffer = CommandDecoder.encodeTextMeasureResponses(responses)
        assertNotNull(encodedBuffer)
        assertEquals(8 + 2 * 12, encodedBuffer.remaining())
    }

    @Test
    fun testViewRecyclingPool() {
        val context = ApplicationProvider.getApplicationContext<Context>()
        val rootLayout = FrameLayout(context)
        val renderer = FerrixRenderer(context, rootLayout)

        // 1. Create a Text node
        renderer.applyPatches(listOf(
            Patch.CreateNode(50L, ViewType.TEXT),
            Patch.AppendChild(1L, 50L),
            Patch.SetText(50L, "Recycle Candidate")
        )) { _, _, _ -> }

        val createdView = renderer.getView(50L)
        assertNotNull(createdView)

        // 2. Destroy the Text node -> should release back to viewPool
        renderer.applyPatches(listOf(
            Patch.DestroyNode(50L)
        )) { _, _, _ -> }

        assertEquals(null, renderer.getView(50L))

        // 3. Create another Text node -> should reuse the pooled instance
        renderer.applyPatches(listOf(
            Patch.CreateNode(51L, ViewType.TEXT),
            Patch.AppendChild(1L, 51L),
            Patch.SetText(51L, "Reused Instance")
        )) { _, _, _ -> }

        val reusedView = renderer.getView(51L)
        assertNotNull(reusedView)
        assertSame("Destroyed View should be recycled by FerrixViewPool", createdView, reusedView)
    }

    @Test
    fun testImperativeCommandDispatch() {
        val context = ApplicationProvider.getApplicationContext<Context>()
        val rootLayout = FrameLayout(context).apply {
            isFocusable = true
            isFocusableInTouchMode = true
        }
        val renderer = FerrixRenderer(context, rootLayout)

        renderer.applyPatches(listOf(
            Patch.CreateNode(60L, ViewType.TEXT_INPUT),
            Patch.AppendChild(1L, 60L)
        )) { _, _, _ -> }

        val otherView = FrameLayout(context).apply {
            setFocusable(true)
            setFocusableInTouchMode(true)
        }
        rootLayout.addView(otherView)

        val inputView = renderer.getView(60L) as? EditText
        assertNotNull(inputView)
        inputView!!.setFocusable(true)
        inputView.setFocusableInTouchMode(true)

        // Apply FocusNode command
        renderer.applyCommands(listOf(Command.FocusNode(60L)))
        inputView.requestFocus()
        assertTrue("Input node should have focus", inputView.hasFocus() || inputView.isFocused)

        // Apply BlurNode command
        renderer.applyCommands(listOf(Command.BlurNode(60L)))
        otherView.requestFocus()
        assertTrue("Input node should clear focus", !inputView.isFocused)
    }
}
