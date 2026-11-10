package com.ferrix.runtime

import android.widget.Button
import android.widget.LinearLayout
import android.widget.TextView
import androidx.test.core.app.ApplicationProvider
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
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
class PlatformSDKTest {

    companion object {
        @JvmStatic
        @BeforeClass
        fun loadNativeLibrary() {
            NativeLibLoader.ensureLoaded()
        }
    }

    /**
     * Milestone 5 Gate Verification:
     * Camera + Location demo app with runtime permissions, fused location,
     * camera preview binding, and lifecycle teardown / cancellation.
     */
    @Test
    fun testPlatformSDKCameraAndLocationDemoAppLifecycle() {
        val context = ApplicationProvider.getApplicationContext<android.content.Context>()
        val rootLayout = LinearLayout(context).apply {
            orientation = LinearLayout.VERTICAL
        }

        val bridge = FerrixBridge()
        bridge.start(context, rootLayout, "platform")

        // 1. Initial UI Tree Check (7 children)
        assertEquals(7, rootLayout.childCount)

        val headerText = rootLayout.getChildAt(0) as TextView
        assertEquals("Ferrix Platform SDK (M5)", headerText.text.toString())

        val permText = rootLayout.getChildAt(1) as TextView
        assertEquals("Permission: Not Requested", permText.text.toString())
        val permBtn = rootLayout.getChildAt(2) as Button
        assertEquals("Request Camera Permission", permBtn.text.toString())

        val locText = rootLayout.getChildAt(3) as TextView
        assertEquals("Location: Inactive", locText.text.toString())
        val locBtn = rootLayout.getChildAt(4) as Button
        assertEquals("Start Location Updates", locBtn.text.toString())

        val camText = rootLayout.getChildAt(5) as TextView
        assertEquals("Camera: Inactive", camText.text.toString())
        val camBtn = rootLayout.getChildAt(6) as Button
        assertEquals("Start Camera Preview", camBtn.text.toString())

        val platformMgr = bridge.platformManager
        assertNotNull("PlatformManager must be initialized", platformMgr)

        // 2. Request Camera Permission
        platformMgr!!.permissionAdapter.setMockPermission("android.permission.CAMERA", true)
        permBtn.performClick()
        assertEquals("Permission: Granted", permText.text.toString())

        // 3. Start Location Updates
        locBtn.performClick()
        assertTrue("LocationAdapter must be tracking", platformMgr.locationAdapter.isTracking)

        // Emit simulated location update
        platformMgr.locationAdapter.emitLocation(37.7749, -122.4194, 4.5f, System.currentTimeMillis())
        assertTrue("Location text must update with latitude", locText.text.toString().contains("37.7749"))
        assertTrue("Location text must update with longitude", locText.text.toString().contains("-122.4194"))

        // 4. Start Camera Preview
        camBtn.performClick()
        assertTrue("CameraAdapter must be active", platformMgr.cameraAdapter.isPreviewActive)
        assertEquals("Camera: Active (1280x720)", camText.text.toString())

        // 5. Component Teardown & Lifecycle Cancellation
        bridge.destroy()
        assertFalse("LocationAdapter must stop tracking on unmount", platformMgr.locationAdapter.isTracking)
        assertFalse("CameraAdapter preview must stop on unmount", platformMgr.cameraAdapter.isPreviewActive)
    }
}

