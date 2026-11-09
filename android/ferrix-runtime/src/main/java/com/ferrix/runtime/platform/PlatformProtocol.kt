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

    fun encode(): ByteArray {
        val buffer = ByteBuffer.allocate(1024).order(ByteOrder.LITTLE_ENDIAN)
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
        }
        buffer.flip()
        val result = ByteArray(buffer.remaining())
        buffer.get(result)
        return result
    }
}
