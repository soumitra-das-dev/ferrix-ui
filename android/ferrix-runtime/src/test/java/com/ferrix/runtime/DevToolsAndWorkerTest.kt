package com.ferrix.runtime

import android.content.Context
import androidx.test.core.app.ApplicationProvider
import com.ferrix.runtime.platform.PlatformManager
import com.ferrix.runtime.platform.PlatformRequest
import com.ferrix.runtime.platform.PlatformResponse
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Before
import org.junit.Test
import org.junit.runner.RunWith
import org.robolectric.RobolectricTestRunner
import java.nio.ByteBuffer
import java.nio.ByteOrder

@RunWith(RobolectricTestRunner::class)
class DevToolsAndWorkerTest {

    private lateinit var context: Context
    private lateinit var platformManager: PlatformManager

    @Before
    fun setUp() {
        context = ApplicationProvider.getApplicationContext()
        platformManager = PlatformManager(context)
    }

    @Test
    fun testBackgroundTaskSchedulingAndCancellation() {
        val taskId = "periodic_analytics_sync"

        // 1. Construct binary ScheduleBackgroundTask request
        val buffer = ByteBuffer.allocate(256).order(ByteOrder.LITTLE_ENDIAN)
        buffer.putShort(18) // Tag 18
        val idBytes = taskId.toByteArray(Charsets.UTF_8)
        buffer.putInt(idBytes.size)
        buffer.put(idBytes)
        buffer.putLong(5000L) // delay 5s
        buffer.put(1.toByte()) // isPeriodic = true
        buffer.putLong(3600000L) // interval 1h
        buffer.flip()

        val reqBytes = ByteArray(buffer.remaining())
        buffer.get(reqBytes)

        platformManager.handleRequest(reqBytes) { }

        assertTrue(platformManager.scheduledTasks.contains(taskId))

        // 2. Construct binary CancelBackgroundTask request
        val cancelBuffer = ByteBuffer.allocate(256).order(ByteOrder.LITTLE_ENDIAN)
        cancelBuffer.putShort(19) // Tag 19
        cancelBuffer.putInt(idBytes.size)
        cancelBuffer.put(idBytes)
        cancelBuffer.flip()

        val cancelBytes = ByteArray(cancelBuffer.remaining())
        cancelBuffer.get(cancelBytes)

        platformManager.handleRequest(cancelBytes) { }

        assertFalse(platformManager.scheduledTasks.contains(taskId))
    }

    @Test
    fun testBackgroundTaskCompletionResponseEncoding() {
        var responseReceived: PlatformResponse? = null

        platformManager.scheduledTasks.add("data_backup")
        platformManager.completeBackgroundTask("data_backup", true) { responseBytes ->
            responseReceived = PlatformResponse.decode(ByteBuffer.wrap(responseBytes))
        }

        assertTrue(responseReceived is PlatformResponse.BackgroundTaskCompleted)
        val completed = responseReceived as PlatformResponse.BackgroundTaskCompleted
        assertEquals("data_backup", completed.taskId)
        assertTrue(completed.success)
        assertFalse(platformManager.scheduledTasks.contains("data_backup"))
    }
}
