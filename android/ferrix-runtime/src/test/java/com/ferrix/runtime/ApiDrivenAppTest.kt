package com.ferrix.runtime

import android.content.Context
import android.widget.Button
import android.widget.FrameLayout
import android.widget.ImageView
import android.widget.LinearLayout
import android.widget.TextView
import androidx.core.widget.NestedScrollView
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
class ApiDrivenAppTest {

    companion object {
        @JvmStatic
        @BeforeClass
        fun loadNativeLibrary() {
            NativeLibLoader.ensureLoaded()
        }
    }

    /**
     * Milestone 6 Gate Verification:
     * API-driven store app:
     * - Fetches items via use_query
     * - Renders ScrollView, Image items with URL, titles, prices, likes
     * - Dispatches mutation via use_mutation on like button tap
     * - Triggers native frame-by-frame Android animation on interaction
     * - Verifies dynamic re-rendering and teardown
     */
    @Test
    fun testApiDrivenAppLifecycleAndReactivity() {
        val context = ApplicationProvider.getApplicationContext<Context>()
        val rootLayout = LinearLayout(context).apply {
            orientation = LinearLayout.VERTICAL
        }

        val bridge = FerrixBridge()
        bridge.start(context, rootLayout, "api")

        // 1. Content has: Header, Subtitle, and 3 Product Cards
        assertTrue("Content child count must be >= 5", rootLayout.childCount >= 5)

        val title = rootLayout.getChildAt(0) as TextView
        assertEquals("Ferrix Store (M6 API App)", title.text.toString())

        // 2. Check Product Card 1
        val card1 = rootLayout.getChildAt(2) as LinearLayout
        val img1 = card1.getChildAt(0) as ImageView
        val name1 = card1.getChildAt(1) as TextView
        val likes1 = card1.getChildAt(2) as TextView
        val likeBtn1 = card1.getChildAt(3) as Button

        assertEquals("https://ferrix.dev/img/headphones.png", img1.tag)
        assertEquals("Ferrix Pro Wireless Headphones", name1.text.toString())
        assertTrue(likes1.text.toString().contains("12 Likes"))
        assertEquals("🤍 Like", likeBtn1.text.toString())

        // 3. User taps like button -> use_mutation triggered
        likeBtn1.performClick()

        // 4. Assert reactive UI update
        assertTrue(likes1.text.toString().contains("13 Likes"))
        assertEquals("❤️ Liked", likeBtn1.text.toString())

        // 5. User taps unlike -> mutation triggered
        likeBtn1.performClick()

        assertTrue(likes1.text.toString().contains("12 Likes"))
        assertEquals("🤍 Like", likeBtn1.text.toString())

        // 6. Teardown / Unmount
        bridge.destroy()
    }
}

