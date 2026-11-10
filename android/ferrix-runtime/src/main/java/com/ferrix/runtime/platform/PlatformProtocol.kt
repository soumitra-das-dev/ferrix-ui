package com.ferrix.runtime.platform

import java.nio.ByteBuffer
import java.nio.ByteOrder

sealed class PlatformRequest {
    data class RequestPermission(val requestId: Int, val permission: String) : PlatformRequest()
    data class StartLocationUpdates(val minIntervalMs: Long, val minDistanceM: Float) : PlatformRequest()
    object StopLocationUpdates : PlatformRequest()
    data class StartCameraPreview(val targetNodeId: Long) : PlatformRequest()
    object StopCamera : PlatformRequest()
    data class TakePicture(val requestId: Int) : PlatformRequest()
    data class ReadStorage(val requestId: Int, val key: String) : PlatformRequest()
    data class WriteStorage(val requestId: Int, val key: String, val value: String) : PlatformRequest()
    data class Vibrate(val durationMs: Long) : PlatformRequest()
    data class SetClipboard(val text: String) : PlatformRequest()
    data class GetClipboard(val requestId: Int) : PlatformRequest()
    data class Share(val title: String, val message: String) : PlatformRequest()
    data class OpenUrl(val url: String) : PlatformRequest()
    data class ShowAlert(val title: String, val message: String) : PlatformRequest()
    object ExitApp : PlatformRequest()
    object DismissKeyboard : PlatformRequest()
    data class AnnounceAccessibility(val message: String) : PlatformRequest()
    data class ScheduleBackgroundTask(val taskId: String, val delayMs: Long, val isPeriodic: Boolean, val intervalMs: Long) : PlatformRequest()
    data class CancelBackgroundTask(val taskId: String) : PlatformRequest()

    companion object {
        fun decode(buffer: ByteBuffer): PlatformRequest {
            buffer.order(ByteOrder.LITTLE_ENDIAN)
            val tag = buffer.short.toInt()
            return when (tag) {
                1 -> {
                    val requestId = buffer.int
                    val len = buffer.int
                    val bytes = ByteArray(len)
                    buffer.get(bytes)
                    RequestPermission(requestId, String(bytes, Charsets.UTF_8))
                }
                2 -> {
                    val minIntervalMs = buffer.long
                    val minDistanceM = buffer.float
                    StartLocationUpdates(minIntervalMs, minDistanceM)
                }
                3 -> StopLocationUpdates
                4 -> {
                    val targetNodeId = buffer.long
                    StartCameraPreview(targetNodeId)
                }
                5 -> StopCamera
                6 -> {
                    val requestId = buffer.int
                    TakePicture(requestId)
                }
                7 -> {
                    val requestId = buffer.int
                    val len = buffer.int
                    val bytes = ByteArray(len)
                    buffer.get(bytes)
                    ReadStorage(requestId, String(bytes, Charsets.UTF_8))
                }
                8 -> {
                    val requestId = buffer.int
                    val kLen = buffer.int
                    val kBytes = ByteArray(kLen)
                    buffer.get(kBytes)
                    val key = String(kBytes, Charsets.UTF_8)
                    val vLen = buffer.int
                    val vBytes = ByteArray(vLen)
                    buffer.get(vBytes)
                    val value = String(vBytes, Charsets.UTF_8)
                    WriteStorage(requestId, key, value)
                }
                9 -> {
                    val durationMs = buffer.long
                    Vibrate(durationMs)
                }
                10 -> {
                    val len = buffer.int
                    val bytes = ByteArray(len)
                    buffer.get(bytes)
                    SetClipboard(String(bytes, Charsets.UTF_8))
                }
                11 -> {
                    val requestId = buffer.int
                    GetClipboard(requestId)
                }
                12 -> {
                    val tLen = buffer.int
                    val tBytes = ByteArray(tLen)
                    buffer.get(tBytes)
                    val title = String(tBytes, Charsets.UTF_8)
                    val mLen = buffer.int
                    val mBytes = ByteArray(mLen)
                    buffer.get(mBytes)
                    val message = String(mBytes, Charsets.UTF_8)
                    Share(title, message)
                }
                13 -> {
                    val len = buffer.int
                    val bytes = ByteArray(len)
                    buffer.get(bytes)
                    OpenUrl(String(bytes, Charsets.UTF_8))
                }
                14 -> {
                    val tLen = buffer.int
                    val tBytes = ByteArray(tLen)
                    buffer.get(tBytes)
                    val title = String(tBytes, Charsets.UTF_8)
                    val mLen = buffer.int
                    val mBytes = ByteArray(mLen)
                    buffer.get(mBytes)
                    val message = String(mBytes, Charsets.UTF_8)
                    ShowAlert(title, message)
                }
                15 -> ExitApp
                16 -> DismissKeyboard
                17 -> {
                    val len = buffer.int
                    val bytes = ByteArray(len)
                    buffer.get(bytes)
                    AnnounceAccessibility(String(bytes, Charsets.UTF_8))
                }
                18 -> {
                    val idLen = buffer.int
                    val idBytes = ByteArray(idLen)
                    buffer.get(idBytes)
                    val taskId = String(idBytes, Charsets.UTF_8)
                    val delayMs = buffer.long
                    val isPeriodic = buffer.get() != 0.toByte()
                    val intervalMs = buffer.long
                    ScheduleBackgroundTask(taskId, delayMs, isPeriodic, intervalMs)
                }
                19 -> {
                    val idLen = buffer.int
                    val idBytes = ByteArray(idLen)
                    buffer.get(idBytes)
                    CancelBackgroundTask(String(idBytes, Charsets.UTF_8))
                }
                else -> throw IllegalArgumentException("Unknown PlatformRequest tag: $tag")
            }
        }
    }
}

sealed class PlatformResponse {
    data class PermissionResult(val requestId: Int, val granted: Boolean, val shouldShowRationale: Boolean) : PlatformResponse()
    data class LocationUpdate(val latitude: Double, val longitude: Double, val accuracy: Float, val timestamp: Long) : PlatformResponse()
    data class CameraReady(val targetNodeId: Long, val previewWidth: Int, val previewHeight: Int) : PlatformResponse()
    data class PictureTaken(val requestId: Int, val path: String, val width: Int, val height: Int) : PlatformResponse()
    data class StorageResult(val requestId: Int, val key: String, val value: String?) : PlatformResponse()
    data class Error(val requestId: Int, val message: String) : PlatformResponse()
    data class ClipboardContent(val requestId: Int, val text: String) : PlatformResponse()
    data class NetInfoUpdate(val isConnected: Boolean, val isWifi: Boolean) : PlatformResponse()
    data class AppStateChange(val state: Byte) : PlatformResponse() // 0=active, 1=background, 2=inactive
    data class KeyboardChange(val isVisible: Boolean, val heightDp: Float) : PlatformResponse()
    data class DimensionsChange(val width: Float, val height: Float, val scale: Float, val fontScale: Float) : PlatformResponse()
    object BackPressed : PlatformResponse()
    data class BackgroundTaskCompleted(val taskId: String, val success: Boolean) : PlatformResponse()

    fun encode(): ByteArray {
        val buffer = ByteBuffer.allocate(2048).order(ByteOrder.LITTLE_ENDIAN)
        when (this) {
            is PermissionResult -> {
                buffer.putShort(1)
                buffer.putInt(requestId)
                buffer.put(if (granted) 1.toByte() else 0.toByte())
                buffer.put(if (shouldShowRationale) 1.toByte() else 0.toByte())
            }
            is LocationUpdate -> {
                buffer.putShort(2)
                buffer.putDouble(latitude)
                buffer.putDouble(longitude)
                buffer.putFloat(accuracy)
                buffer.putLong(timestamp)
            }
            is CameraReady -> {
                buffer.putShort(3)
                buffer.putLong(targetNodeId)
                buffer.putInt(previewWidth)
                buffer.putInt(previewHeight)
            }
            is PictureTaken -> {
                buffer.putShort(4)
                buffer.putInt(requestId)
                buffer.putInt(width)
                buffer.putInt(height)
                val bytes = path.toByteArray(Charsets.UTF_8)
                buffer.putInt(bytes.size)
                buffer.put(bytes)
            }
            is StorageResult -> {
                buffer.putShort(5)
                buffer.putInt(requestId)
                val kBytes = key.toByteArray(Charsets.UTF_8)
                buffer.putInt(kBytes.size)
                buffer.put(kBytes)
                if (value != null) {
                    buffer.put(1.toByte())
                    val vBytes = value.toByteArray(Charsets.UTF_8)
                    buffer.putInt(vBytes.size)
                    buffer.put(vBytes)
                } else {
                    buffer.put(0.toByte())
                }
            }
            is Error -> {
                buffer.putShort(6)
                buffer.putInt(requestId)
                val bytes = message.toByteArray(Charsets.UTF_8)
                buffer.putInt(bytes.size)
                buffer.put(bytes)
            }
            is ClipboardContent -> {
                buffer.putShort(7)
                buffer.putInt(requestId)
                val bytes = text.toByteArray(Charsets.UTF_8)
                buffer.putInt(bytes.size)
                buffer.put(bytes)
            }
            is NetInfoUpdate -> {
                buffer.putShort(8)
                buffer.put(if (isConnected) 1.toByte() else 0.toByte())
                buffer.put(if (isWifi) 1.toByte() else 0.toByte())
            }
            is AppStateChange -> {
                buffer.putShort(9)
                buffer.put(state)
            }
            is KeyboardChange -> {
                buffer.putShort(10)
                buffer.put(if (isVisible) 1.toByte() else 0.toByte())
                buffer.putFloat(heightDp)
            }
            is DimensionsChange -> {
                buffer.putShort(11)
                buffer.putFloat(width)
                buffer.putFloat(height)
                buffer.putFloat(scale)
                buffer.putFloat(fontScale)
            }
            is BackPressed -> {
                buffer.putShort(12)
            }
            is BackgroundTaskCompleted -> {
                buffer.putShort(13)
                val idBytes = taskId.toByteArray(Charsets.UTF_8)
                buffer.putInt(idBytes.size)
                buffer.put(idBytes)
                buffer.put(if (success) 1.toByte() else 0.toByte())
            }
        }
        buffer.flip()
        val result = ByteArray(buffer.remaining())
        buffer.get(result)
        return result
    }

    companion object {
        fun decode(buffer: ByteBuffer): PlatformResponse {
            buffer.order(ByteOrder.LITTLE_ENDIAN)
            val tag = buffer.short.toInt()
            return when (tag) {
                1 -> {
                    val reqId = buffer.int
                    val granted = buffer.get() != 0.toByte()
                    val rationale = buffer.get() != 0.toByte()
                    PermissionResult(reqId, granted, rationale)
                }
                2 -> {
                    val lat = buffer.double
                    val lng = buffer.double
                    val acc = buffer.float
                    val time = buffer.long
                    LocationUpdate(lat, lng, acc, time)
                }
                3 -> {
                    val nodeId = buffer.long
                    val w = buffer.int
                    val h = buffer.int
                    CameraReady(nodeId, w, h)
                }
                4 -> {
                    val reqId = buffer.int
                    val w = buffer.int
                    val h = buffer.int
                    val len = buffer.int
                    val bytes = ByteArray(len)
                    buffer.get(bytes)
                    PictureTaken(reqId, String(bytes, Charsets.UTF_8), w, h)
                }
                5 -> {
                    val reqId = buffer.int
                    val kLen = buffer.int
                    val kBytes = ByteArray(kLen)
                    buffer.get(kBytes)
                    val key = String(kBytes, Charsets.UTF_8)
                    val hasVal = buffer.get() != 0.toByte()
                    val value = if (hasVal) {
                        val vLen = buffer.int
                        val vBytes = ByteArray(vLen)
                        buffer.get(vBytes)
                        String(vBytes, Charsets.UTF_8)
                    } else null
                    StorageResult(reqId, key, value)
                }
                6 -> {
                    val reqId = buffer.int
                    val len = buffer.int
                    val bytes = ByteArray(len)
                    buffer.get(bytes)
                    Error(reqId, String(bytes, Charsets.UTF_8))
                }
                7 -> {
                    val reqId = buffer.int
                    val len = buffer.int
                    val bytes = ByteArray(len)
                    buffer.get(bytes)
                    ClipboardContent(reqId, String(bytes, Charsets.UTF_8))
                }
                8 -> {
                    val isConnected = buffer.get() != 0.toByte()
                    val isWifi = buffer.get() != 0.toByte()
                    NetInfoUpdate(isConnected, isWifi)
                }
                9 -> {
                    val state = buffer.get()
                    AppStateChange(state)
                }
                10 -> {
                    val isVisible = buffer.get() != 0.toByte()
                    val heightDp = buffer.float
                    KeyboardChange(isVisible, heightDp)
                }
                11 -> {
                    val width = buffer.float
                    val height = buffer.float
                    val scale = buffer.float
                    val fontScale = buffer.float
                    DimensionsChange(width, height, scale, fontScale)
                }
                12 -> BackPressed
                13 -> {
                    val idLen = buffer.int
                    val idBytes = ByteArray(idLen)
                    buffer.get(idBytes)
                    val taskId = String(idBytes, Charsets.UTF_8)
                    val success = buffer.get() != 0.toByte()
                    BackgroundTaskCompleted(taskId, success)
                }
                else -> throw IllegalArgumentException("Unknown PlatformResponse tag: $tag")
            }
        }
    }
}
