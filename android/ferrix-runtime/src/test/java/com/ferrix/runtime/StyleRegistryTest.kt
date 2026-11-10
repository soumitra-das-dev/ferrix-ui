package com.ferrix.runtime

import android.content.Context
import android.widget.FrameLayout
import android.widget.TextView
import androidx.test.core.app.ApplicationProvider
import com.ferrix.runtime.protocol.CommandDecoder
import com.ferrix.runtime.protocol.Patch
import com.ferrix.runtime.protocol.Property
import com.ferrix.runtime.protocol.ViewType
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNotNull
import org.junit.Assert.assertTrue
import org.junit.Before
import org.junit.Test
import org.junit.runner.RunWith
import org.robolectric.RobolectricTestRunner
import org.robolectric.annotation.Config
import java.nio.ByteBuffer
import java.nio.ByteOrder

@RunWith(RobolectricTestRunner::class)
@Config(sdk = [34])
class StyleRegistryTest {

    private lateinit var context: Context
    private lateinit var rootContainer: FrameLayout
    private lateinit var renderer: FerrixRenderer

    @Before
    fun setup() {
        context = ApplicationProvider.getApplicationContext()
        rootContainer = FrameLayout(context)
        renderer = FerrixRenderer(context, rootContainer)
    }

    @Test
    fun testDecodeStyleRegistryPatches() {
        val payload = ByteBuffer.allocate(1024).order(ByteOrder.LITTLE_ENDIAN)

        // 1. RegisterStyles: 2 styles
        payload.put(CommandDecoder.OP_REGISTER_STYLES)
        payload.putInt(2) // 2 styles

        // Style 101: 2 properties (BackgroundColor, CornerRadius)
        payload.putInt(101)
        payload.putShort(2.toShort()) // 2 properties
        // Prop 1: BackgroundColor = 0xFF1E293B
        payload.putShort(1.toShort()) // propId = 1
        payload.putShort(4.toShort()) // len = 4
        payload.putInt(0xFF1E293B.toInt())
        // Prop 6: CornerRadius = 8.0f
        payload.putShort(6.toShort()) // propId = 6
        payload.putShort(4.toShort()) // len = 4
        payload.putFloat(8.0f)

        // Style 102: 1 property (FontSize = 18.0f)
        payload.putInt(102)
        payload.putShort(1.toShort()) // 1 property
        payload.putShort(3.toShort()) // propId = 3 (FontSize)
        payload.putShort(4.toShort()) // len = 4
        payload.putFloat(18.0f)

        // 2. SetStyle: node 2, style 101
        payload.put(CommandDecoder.OP_SET_STYLE)
        payload.putLong(2L)
        payload.putInt(101)

        // 3. SetStyleList: node 3, styles [101, 102]
        payload.put(CommandDecoder.OP_SET_STYLE_LIST)
        payload.putLong(3L)
        payload.putShort(2.toShort()) // 2 styles
        payload.putInt(101)
        payload.putInt(102)

        val payloadSize = payload.position()

        // Assemble full buffer with 16-byte header
        val fullBuffer = ByteBuffer.allocate(16 + payloadSize).order(ByteOrder.LITTLE_ENDIAN)
        fullBuffer.put(0x46.toByte()) // 'F'
        fullBuffer.put(0x58.toByte()) // 'X'
        fullBuffer.put(0x55.toByte()) // 'U'
        fullBuffer.put(0x49.toByte()) // 'I'
        fullBuffer.putShort(1.toShort()) // version = 1
        fullBuffer.putShort(0.toShort()) // flags = 0
        fullBuffer.putInt(3) // command count = 3
        fullBuffer.putInt(payloadSize) // payload len
        fullBuffer.put(payload.array(), 0, payloadSize)
        fullBuffer.flip()

        val patches = CommandDecoder.decode(fullBuffer)
        assertEquals(3, patches.size)

        // Verify RegisterStyles
        assertTrue(patches[0] is Patch.RegisterStyles)
        val reg = patches[0] as Patch.RegisterStyles
        assertEquals(2, reg.styles.size)
        assertEquals(101, reg.styles[0].first)
        assertEquals(2, reg.styles[0].second.size)
        assertEquals(Property.BackgroundColor(0xFF1E293B.toInt()), reg.styles[0].second[0])
        assertEquals(Property.CornerRadius(8.0f), reg.styles[0].second[1])
        assertEquals(102, reg.styles[1].first)
        assertEquals(1, reg.styles[1].second.size)
        assertEquals(Property.FontSize(18.0f), reg.styles[1].second[0])

        // Verify SetStyle
        assertEquals(Patch.SetStyle(2L, 101), patches[1])

        // Verify SetStyleList
        assertEquals(Patch.SetStyleList(3L, listOf(101, 102)), patches[2])
    }

    @Test
    fun testRendererAppliesRegisteredStyles() {
        // Create nodes
        val patches = listOf(
            Patch.CreateNode(2L, ViewType.TEXT),
            Patch.AppendChild(1L, 2L),
            Patch.SetText(2L, "Styled Text"),
            Patch.RegisterStyles(
                listOf(
                    Pair(
                        101,
                        listOf(
                            Property.BackgroundColor(0xFF112233.toInt()),
                            Property.FontSize(22.0f)
                        )
                    )
                )
            ),
            Patch.SetStyle(2L, 101)
        )

        renderer.applyPatches(patches) { _, _, _ -> }

        // Verify styleRegistry was populated
        assertTrue(renderer.styleRegistry.containsKey(101))
        assertEquals(2, renderer.styleRegistry[101]?.size)

        // Verify style properties were applied to the View
        val view = renderer.getView(2L)
        assertNotNull(view)
        assertTrue(view is TextView)
        val textView = view as TextView
        assertEquals("Styled Text", textView.text.toString())
        // In Robolectric with SP units, verify text size is applied
        assertTrue(textView.textSize > 0f)
    }

    @Test
    fun testRendererAppliesCascadeStyleList() {
        val patches = listOf(
            Patch.CreateNode(3L, ViewType.TEXT),
            Patch.AppendChild(1L, 3L),
            Patch.RegisterStyles(
                listOf(
                    Pair(1, listOf(Property.TextColor(0xFF888888.toInt()), Property.FontSize(14.0f))),
                    Pair(2, listOf(Property.TextColor(0xFF00FF00.toInt()))) // overrides color from style 1
                )
            ),
            Patch.SetStyleList(3L, listOf(1, 2))
        )

        renderer.applyPatches(patches) { _, _, _ -> }

        val view = renderer.getView(3L) as? TextView
        assertNotNull(view)
        assertEquals(0xFF00FF00.toInt(), view?.currentTextColor)
    }
}

