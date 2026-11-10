package com.ferrix.runtime

import android.content.Context
import android.widget.FrameLayout
import android.widget.ImageView
import android.widget.ProgressBar
import android.widget.SeekBar
import android.widget.TextView
import androidx.appcompat.widget.AppCompatCheckBox
import androidx.test.core.app.ApplicationProvider
import com.ferrix.runtime.protocol.EventType
import com.ferrix.runtime.protocol.Patch
import com.ferrix.runtime.protocol.Property
import com.ferrix.runtime.protocol.ViewType
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNotNull
import org.junit.Assert.assertTrue
import org.junit.Test
import org.junit.runner.RunWith
import org.robolectric.RobolectricTestRunner
import org.robolectric.annotation.Config
import java.nio.ByteBuffer
import java.nio.ByteOrder

@RunWith(RobolectricTestRunner::class)
@Config(sdk = [34])
class Wave2ExtendedPrimitivesTest {

    @Test
    fun testCheckboxCheckedAndEventDispatch() {
        val context = ApplicationProvider.getApplicationContext<Context>()
        val rootLayout = FrameLayout(context)
        val renderer = FerrixRenderer(context, rootLayout)

        var lastEventValue: Byte? = null

        val patches = listOf(
            Patch.CreateNode(100L, ViewType.CHECKBOX),
            Patch.AppendChild(1L, 100L),
            Patch.SetProperty(100L, Property.Checked(true)),
            Patch.AttachEventListener(100L, EventType.VALUE_CHANGE)
        )

        renderer.applyPatches(patches) { nodeId, eventType, payload ->
            if (nodeId == 100L && eventType == EventType.VALUE_CHANGE) {
                lastEventValue = payload?.firstOrNull()
            }
        }

        val checkbox = renderer.getView(100L) as? AppCompatCheckBox
        assertNotNull(checkbox)
        assertTrue(checkbox!!.isChecked)

        // Toggle
        checkbox.isChecked = false
        assertEquals(0.toByte(), lastEventValue)
    }

    @Test
    fun testSliderValueAndProgress() {
        val context = ApplicationProvider.getApplicationContext<Context>()
        val rootLayout = FrameLayout(context)
        val renderer = FerrixRenderer(context, rootLayout)

        val patches = listOf(
            Patch.CreateNode(101L, ViewType.SLIDER),
            Patch.AppendChild(1L, 101L),
            Patch.SetProperty(101L, Property.SliderValue(0.75f)),
            Patch.CreateNode(102L, ViewType.PROGRESS_BAR),
            Patch.AppendChild(1L, 102L),
            Patch.SetProperty(102L, Property.Progress(0.85f))
        )

        renderer.applyPatches(patches) { _, _, _ -> }

        val slider = renderer.getView(101L) as? SeekBar
        val progressBar = renderer.getView(102L) as? ProgressBar

        assertNotNull(slider)
        assertNotNull(progressBar)
        assertEquals(750, slider?.progress)
        assertEquals(85, progressBar?.progress)
    }

    @Test
    fun testBadgeAndLinearGradientAndIcon() {
        val context = ApplicationProvider.getApplicationContext<Context>()
        val rootLayout = FrameLayout(context)
        val renderer = FerrixRenderer(context, rootLayout)

        val patches = listOf(
            Patch.CreateNode(103L, ViewType.BADGE),
            Patch.AppendChild(1L, 103L),
            Patch.SetProperty(103L, Property.BadgeContent("NEW")),
            Patch.CreateNode(104L, ViewType.LINEAR_GRADIENT),
            Patch.AppendChild(1L, 104L),
            Patch.SetProperty(
                104L,
                Property.GradientColors(
                    0xFF3B82F6.toInt(),
                    0xFF8B5CF6.toInt(),
                    90.0f
                )
            ),
            Patch.CreateNode(105L, ViewType.ICON),
            Patch.AppendChild(1L, 105L),
            Patch.SetProperty(105L, Property.IconName("star")),
            Patch.SetProperty(105L, Property.IconSize(32.0f))
        )

        renderer.applyPatches(patches) { _, _, _ -> }

        val badge = renderer.getView(103L) as? TextView
        val gradient = renderer.getView(104L)
        val icon = renderer.getView(105L) as? ImageView

        assertNotNull(badge)
        assertNotNull(gradient)
        assertNotNull(icon)

        assertEquals("NEW", badge?.text)
        assertNotNull(gradient?.background)
        assertEquals("star", icon?.tag)
    }
}

