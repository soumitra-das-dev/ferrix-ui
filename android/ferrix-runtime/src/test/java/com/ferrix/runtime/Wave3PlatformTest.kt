package com.ferrix.runtime

import android.content.Context
import androidx.test.core.app.ApplicationProvider
import com.ferrix.runtime.platform.PlatformManager
import com.ferrix.runtime.platform.PlatformRequest
import com.ferrix.runtime.platform.PlatformResponse
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNotNull
import org.junit.Assert.assertTrue
import org.junit.Test
import org.junit.runner.RunWith
import org.robolectric.RobolectricTestRunner
import org.robolectric.annotation.Config
import java.nio.ByteBuffer

@RunWith(RobolectricTestRunner::class)
@Config(sdk = [34])
class Wave3PlatformTest {

    @Test
    fun testPlatformProtocolRequestsRoundtrip() {
        val requests: List<PlatformRequest> = listOf(
            PlatformRequest.Vibrate(500L),
            PlatformRequest.SetClipboard("Hello Ferrix Clipboard"),
            PlatformRequest.GetClipboard(42),
            PlatformRequest.Share("Title", "Message"),
            PlatformRequest.OpenUrl("https://ferrix.dev"),
            PlatformRequest.ShowAlert("Dialog Title", "Dialog Message"),
            PlatformRequest.ExitApp,
            PlatformRequest.DismissKeyboard,
            PlatformRequest.AnnounceAccessibility("Screen announced")
        )

        for (req in requests) {
            // Encode via simulated Rust buffer format or Java buffer
            val buffer = ByteBuffer.allocate(1024).order(java.nio.ByteOrder.LITTLE_ENDIAN)
            when (req) {
                is PlatformRequest.Vibrate -> {
                    buffer.putShort(9)
                    buffer.putLong(req.durationMs)
                }
                is PlatformRequest.SetClipboard -> {
                    buffer.putShort(10)
                    val bytes = req.text.toByteArray(Charsets.UTF_8)
                    buffer.putInt(bytes.size)
                    buffer.put(bytes)
                }
                is PlatformRequest.GetClipboard -> {
                    buffer.putShort(11)
                    buffer.putInt(req.requestId)
                }
                is PlatformRequest.Share -> {
                    buffer.putShort(12)
                    val tBytes = req.title.toByteArray(Charsets.UTF_8)
                    buffer.putInt(tBytes.size)
                    buffer.put(tBytes)
                    val mBytes = req.message.toByteArray(Charsets.UTF_8)
                    buffer.putInt(mBytes.size)
                    buffer.put(mBytes)
                }
                is PlatformRequest.OpenUrl -> {
                    buffer.putShort(13)
                    val bytes = req.url.toByteArray(Charsets.UTF_8)
                    buffer.putInt(bytes.size)
                    buffer.put(bytes)
                }
                is PlatformRequest.ShowAlert -> {
                    buffer.putShort(14)
                    val tBytes = req.title.toByteArray(Charsets.UTF_8)
                    buffer.putInt(tBytes.size)
                    buffer.put(tBytes)
                    val mBytes = req.message.toByteArray(Charsets.UTF_8)
                    buffer.putInt(mBytes.size)
                    buffer.put(mBytes)
                }
                is PlatformRequest.ExitApp -> buffer.putShort(15)
                is PlatformRequest.DismissKeyboard -> buffer.putShort(16)
                is PlatformRequest.AnnounceAccessibility -> {
                    buffer.putShort(17)
                    val bytes = req.message.toByteArray(Charsets.UTF_8)
                    buffer.putInt(bytes.size)
                    buffer.put(bytes)
                }
                else -> {}
            }
            buffer.flip()
            val decoded = PlatformRequest.decode(buffer)
            assertEquals("Roundtrip request must match", req, decoded)
        }
    }

    @Test
    fun testPlatformProtocolResponsesRoundtrip() {
        val responses: List<PlatformResponse> = listOf(
            PlatformResponse.ClipboardContent(10, "Copied content"),
            PlatformResponse.NetInfoUpdate(isConnected = true, isWifi = false),
            PlatformResponse.AppStateChange(1.toByte()),
            PlatformResponse.KeyboardChange(isVisible = true, heightDp = 275.5f),
            PlatformResponse.DimensionsChange(width = 390f, height = 844f, scale = 3.0f, fontScale = 1.0f),
            PlatformResponse.BackPressed
        )

        for (resp in responses) {
            val encoded = resp.encode()
            val decoded = PlatformResponse.decode(ByteBuffer.wrap(encoded))
            assertEquals("Roundtrip response must match", resp, decoded)
        }
    }

    @Test
    fun testPlatformManagerOperations() {
        val context = ApplicationProvider.getApplicationContext<Context>()
        val manager = PlatformManager(context)

        // 1. Test Clipboard Set & Get
        val setBuffer = ByteBuffer.allocate(256).order(java.nio.ByteOrder.LITTLE_ENDIAN)
        setBuffer.putShort(10) // SetClipboard
        val clipText = "PlatformManager Test String"
        val clipBytes = clipText.toByteArray(Charsets.UTF_8)
        setBuffer.putInt(clipBytes.size)
        setBuffer.put(clipBytes)
        setBuffer.flip()

        manager.handleRequest(setBuffer.array().copyOf(setBuffer.limit())) { /* No response expected */ }

        // Get clipboard
        val getBuffer = ByteBuffer.allocate(256).order(java.nio.ByteOrder.LITTLE_ENDIAN)
        getBuffer.putShort(11) // GetClipboard
        getBuffer.putInt(99)
        getBuffer.flip()

        var receivedResp: PlatformResponse? = null
        manager.handleRequest(getBuffer.array().copyOf(getBuffer.limit())) { responseBytes ->
            receivedResp = PlatformResponse.decode(ByteBuffer.wrap(responseBytes))
        }

        assertNotNull("Response must be received for GetClipboard", receivedResp)
        assertTrue(receivedResp is PlatformResponse.ClipboardContent)
        val clipContent = receivedResp as PlatformResponse.ClipboardContent
        assertEquals(99, clipContent.requestId)
        assertEquals(clipText, clipContent.text)

        // 2. Test Dimension Dispatch
        var dimResp: PlatformResponse? = null
        manager.sendDimensions(412f, 915f, 2.625f, 1.0f) { bytes ->
            dimResp = PlatformResponse.decode(ByteBuffer.wrap(bytes))
        }
        assertEquals(PlatformResponse.DimensionsChange(412f, 915f, 2.625f, 1.0f), dimResp)

        // 3. Test Keyboard Dispatch
        var kbResp: PlatformResponse? = null
        manager.sendKeyboardChange(true, 280f) { bytes ->
            kbResp = PlatformResponse.decode(ByteBuffer.wrap(bytes))
        }
        assertEquals(PlatformResponse.KeyboardChange(true, 280f), kbResp)

        // 4. Test AppState Dispatch
        var appResp: PlatformResponse? = null
        manager.sendAppState(0.toByte()) { bytes ->
            appResp = PlatformResponse.decode(ByteBuffer.wrap(bytes))
        }
        assertEquals(PlatformResponse.AppStateChange(0.toByte()), appResp)

        // 5. Test NetInfo Dispatch
        var netResp: PlatformResponse? = null
        manager.sendNetInfo(isConnected = true, isWifi = true) { bytes ->
            netResp = PlatformResponse.decode(ByteBuffer.wrap(bytes))
        }
        assertEquals(PlatformResponse.NetInfoUpdate(true, true), netResp)

        // 6. Test BackPressed Dispatch
        var backResp: PlatformResponse? = null
        manager.sendBackPressed { bytes ->
            backResp = PlatformResponse.decode(ByteBuffer.wrap(bytes))
        }
        assertEquals(PlatformResponse.BackPressed, backResp)
    }
}
