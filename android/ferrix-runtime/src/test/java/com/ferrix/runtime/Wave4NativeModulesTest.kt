package com.ferrix.runtime

import android.content.Context
import androidx.test.core.app.ApplicationProvider
import com.ferrix.runtime.modules.NativeModuleRegistry
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNotNull
import org.junit.Assert.assertTrue
import org.junit.Test
import org.junit.runner.RunWith
import org.robolectric.RobolectricTestRunner
import org.robolectric.annotation.Config
import java.io.File
import java.nio.ByteBuffer
import java.nio.ByteOrder

@RunWith(RobolectricTestRunner::class)
@Config(sdk = [34])
class Wave4NativeModulesTest {

    private fun buildCall(callId: Int, moduleId: Int, methodId: Int, payload: ByteArray = ByteArray(0)): ByteArray {
        val buf = ByteBuffer.allocate(12 + payload.size).order(ByteOrder.LITTLE_ENDIAN)
        buf.putInt(callId)
        buf.putShort(moduleId.toShort())
        buf.putShort(methodId.toShort())
        buf.putInt(payload.size)
        buf.put(payload)
        return buf.array()
    }

    @Test
    fun testModuleRegistryPopulationAndLifecycle() {
        val context = ApplicationProvider.getApplicationContext<Context>()
        val registry = NativeModuleRegistry(context)
        assertEquals(12, registry.count())

        for (i in 1..12) {
            val module = registry.get(i)
            assertNotNull("Module $i must be registered", module)
            assertTrue("Module name must not be blank", module!!.name.isNotBlank())
        }

        registry.broadcastStart()
        registry.broadcastStop()
        registry.broadcastDestroy()
    }

    @Test
    fun testCameraAndLocationModules() {
        val context = ApplicationProvider.getApplicationContext<Context>()
        val registry = NativeModuleRegistry(context)

        // 1. Camera take_picture (Module 1, Method 1)
        var respBytes: ByteArray? = null
        registry.handleCall(buildCall(1, 1, 1)) { respBytes = it }
        assertNotNull(respBytes)
        val buf1 = ByteBuffer.wrap(respBytes!!).order(ByteOrder.LITTLE_ENDIAN)
        assertEquals(1, buf1.short.toInt()) // Tag 1: Success
        assertEquals(1, buf1.int) // callId

        // 2. Camera scan_barcode (Module 1, Method 2)
        registry.handleCall(buildCall(2, 1, 2)) { respBytes = it }
        val buf2 = ByteBuffer.wrap(respBytes!!).order(ByteOrder.LITTLE_ENDIAN)
        assertEquals(1, buf2.short.toInt())
        assertEquals(2, buf2.int) // callId
        val len2 = buf2.int
        val bBytes = ByteArray(len2)
        buf2.get(bBytes)
        val barcode = String(bBytes, Charsets.UTF_8)
        assertTrue(barcode.contains("SAMPLE-BARCODE"))

        // 3. Location get_current_position (Module 2, Method 1)
        registry.handleCall(buildCall(3, 2, 1)) { respBytes = it }
        val buf3 = ByteBuffer.wrap(respBytes!!).order(ByteOrder.LITTLE_ENDIAN)
        assertEquals(1, buf3.short.toInt())
        assertEquals(3, buf3.int) // callId
        val pLen = buf3.int
        val pBuf = ByteBuffer.allocate(pLen).order(ByteOrder.LITTLE_ENDIAN)
        buf3.get(pBuf.array())
        val lat = pBuf.double
        val lng = pBuf.double
        assertEquals(37.7749, lat, 0.001)
        assertEquals(-122.4194, lng, 0.001)
    }

    @Test
    fun testSecureStoreAndFileSystemModules() {
        val context = ApplicationProvider.getApplicationContext<Context>()
        val registry = NativeModuleRegistry(context)

        // SecureStore set_item
        val kStr = "auth_token"
        val vStr = "jwt.sample.token.42"
        val setPayload = ByteBuffer.allocate(8 + kStr.length + vStr.length).order(ByteOrder.LITTLE_ENDIAN)
        setPayload.putInt(kStr.length)
        setPayload.put(kStr.toByteArray(Charsets.UTF_8))
        setPayload.putInt(vStr.length)
        setPayload.put(vStr.toByteArray(Charsets.UTF_8))

        var respBytes: ByteArray? = null
        registry.handleCall(buildCall(10, 6, 1, setPayload.array())) { respBytes = it }
        val setRes = ByteBuffer.wrap(respBytes!!).order(ByteOrder.LITTLE_ENDIAN)
        assertEquals(1, setRes.short.toInt())

        // SecureStore get_item
        val getPayload = ByteBuffer.allocate(4 + kStr.length).order(ByteOrder.LITTLE_ENDIAN)
        getPayload.putInt(kStr.length)
        getPayload.put(kStr.toByteArray(Charsets.UTF_8))

        registry.handleCall(buildCall(11, 6, 2, getPayload.array())) { respBytes = it }
        val getRes = ByteBuffer.wrap(respBytes!!).order(ByteOrder.LITTLE_ENDIAN)
        assertEquals(1, getRes.short.toInt())
        assertEquals(11, getRes.int) // callId
        val valLen = getRes.int
        val valBytes = ByteArray(valLen)
        getRes.get(valBytes)
        assertEquals(vStr, String(valBytes, Charsets.UTF_8))

        // FileSystem write and read
        val tempFile = File(context.cacheDir, "test_file.txt").absolutePath
        val fileContent = "Ferrix Native Module File Content"
        val writePayload = ByteBuffer.allocate(8 + tempFile.length + fileContent.length).order(ByteOrder.LITTLE_ENDIAN)
        writePayload.putInt(tempFile.length)
        writePayload.put(tempFile.toByteArray(Charsets.UTF_8))
        writePayload.putInt(fileContent.length)
        writePayload.put(fileContent.toByteArray(Charsets.UTF_8))

        registry.handleCall(buildCall(12, 7, 2, writePayload.array())) { respBytes = it }
        assertEquals(1, ByteBuffer.wrap(respBytes!!).order(ByteOrder.LITTLE_ENDIAN).short.toInt())

        val readPayload = ByteBuffer.allocate(4 + tempFile.length).order(ByteOrder.LITTLE_ENDIAN)
        readPayload.putInt(tempFile.length)
        readPayload.put(tempFile.toByteArray(Charsets.UTF_8))

        registry.handleCall(buildCall(13, 7, 1, readPayload.array())) { respBytes = it }
        val readRes = ByteBuffer.wrap(respBytes!!).order(ByteOrder.LITTLE_ENDIAN)
        assertEquals(1, readRes.short.toInt())
        assertEquals(13, readRes.int) // callId
        val rLen = readRes.int
        val rBytes = ByteArray(rLen)
        readRes.get(rBytes)
        assertEquals(fileContent, String(rBytes, Charsets.UTF_8))
    }

    @Test
    fun testHapticsNotificationsAndLocalAuthModules() {
        val context = ApplicationProvider.getApplicationContext<Context>()
        val registry = NativeModuleRegistry(context)

        // Haptics impact
        var respBytes: ByteArray? = null
        registry.handleCall(buildCall(20, 5, 1, byteArrayOf(2))) { respBytes = it }
        assertEquals(1, ByteBuffer.wrap(respBytes!!).order(ByteOrder.LITTLE_ENDIAN).short.toInt())

        // Notifications get_push_token
        registry.handleCall(buildCall(21, 11, 2)) { respBytes = it }
        val notifRes = ByteBuffer.wrap(respBytes!!).order(ByteOrder.LITTLE_ENDIAN)
        assertEquals(1, notifRes.short.toInt())
        assertEquals(21, notifRes.int) // callId
        val tLen = notifRes.int
        val tBytes = ByteArray(tLen)
        notifRes.get(tBytes)
        assertTrue(String(tBytes, Charsets.UTF_8).contains("fcm_token"))

        // LocalAuth authenticate
        registry.handleCall(buildCall(22, 12, 3)) { respBytes = it }
        val authRes = ByteBuffer.wrap(respBytes!!).order(ByteOrder.LITTLE_ENDIAN)
        assertEquals(1, authRes.short.toInt())
        assertEquals(22, authRes.int) // callId
        val aLen = authRes.int
        val aBytes = ByteArray(aLen)
        authRes.get(aBytes)
        assertEquals(1.toByte(), aBytes[0]) // Authenticated
    }
}
