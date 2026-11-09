package com.ferrix.runtime.modules

import android.content.Context
import android.os.Build
import android.os.VibrationEffect
import android.os.Vibrator
import android.os.VibratorManager
import java.io.File
import java.nio.ByteBuffer
import java.nio.ByteOrder

class CameraNativeModule(val context: Context) : NativeModule {
    override val moduleId: Int = 1
    override val name: String = "ferrix-camera"
    override val requiredPermissions: List<String> = listOf("android.permission.CAMERA")

    override fun invoke(callId: Int, methodId: Int, payload: ByteArray, onResult: (ByteArray) -> Unit) {
        when (methodId) {
            1 -> { // take_picture
                val path = File(context.cacheDir, "photo_${System.currentTimeMillis()}.jpg").absolutePath
                val bytes = path.toByteArray(Charsets.UTF_8)
                onResult(ModuleResultEncoder.success(callId, bytes))
            }
            2 -> { // scan_barcode
                val barcode = "SAMPLE-BARCODE-987654321".toByteArray(Charsets.UTF_8)
                onResult(ModuleResultEncoder.success(callId, barcode))
            }
            else -> onResult(ModuleResultEncoder.error(callId, "ERR_UNKNOWN_METHOD", "Unknown Camera method: $methodId"))
        }
    }
}

class LocationNativeModule(val context: Context) : NativeModule {
    override val moduleId: Int = 2
    override val name: String = "ferrix-location"
    override val requiredPermissions: List<String> = listOf(
        "android.permission.ACCESS_FINE_LOCATION",
        "android.permission.ACCESS_COARSE_LOCATION"
    )

    override fun invoke(callId: Int, methodId: Int, payload: ByteArray, onResult: (ByteArray) -> Unit) {
        when (methodId) {
            1 -> { // get_current_position
                val buf = ByteBuffer.allocate(24).order(ByteOrder.LITTLE_ENDIAN)
                buf.putDouble(37.7749)
                buf.putDouble(-122.4194)
                buf.putFloat(5.0f)
                buf.putInt(1)
                onResult(ModuleResultEncoder.success(callId, buf.array()))
            }
            2 -> { // start_geofence
                onResult(ModuleResultEncoder.success(callId))
            }
            else -> onResult(ModuleResultEncoder.error(callId, "ERR_UNKNOWN_METHOD", "Unknown Location method: $methodId"))
        }
    }
}

class SensorsNativeModule(val context: Context) : NativeModule {
    override val moduleId: Int = 3
    override val name: String = "ferrix-sensors"

    override fun invoke(callId: Int, methodId: Int, payload: ByteArray, onResult: (ByteArray) -> Unit) {
        when (methodId) {
            1 -> { // start_sensor
                val buf = ByteBuffer.allocate(20).order(ByteOrder.LITTLE_ENDIAN)
                buf.putFloat(0.0f) // x
                buf.putFloat(9.8f) // y
                buf.putFloat(0.1f) // z
                buf.putLong(System.currentTimeMillis())
                onResult(ModuleResultEncoder.success(callId, buf.array()))
            }
            2 -> onResult(ModuleResultEncoder.success(callId)) // stop_sensor
            else -> onResult(ModuleResultEncoder.error(callId, "ERR_UNKNOWN_METHOD", "Unknown Sensor method: $methodId"))
        }
    }
}

class AudioNativeModule(val context: Context) : NativeModule {
    override val moduleId: Int = 4
    override val name: String = "ferrix-audio"

    override fun invoke(callId: Int, methodId: Int, payload: ByteArray, onResult: (ByteArray) -> Unit) {
        when (methodId) {
            1, 2, 3 -> onResult(ModuleResultEncoder.success(callId)) // play, pause, stop
            else -> onResult(ModuleResultEncoder.error(callId, "ERR_UNKNOWN_METHOD", "Unknown Audio method: $methodId"))
        }
    }
}

class HapticsNativeModule(val context: Context) : NativeModule {
    override val moduleId: Int = 5
    override val name: String = "ferrix-haptics"
    override val requiredPermissions: List<String> = listOf("android.permission.VIBRATE")

    override fun invoke(callId: Int, methodId: Int, payload: ByteArray, onResult: (ByteArray) -> Unit) {
        try {
            val vibrator = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
                (context.getSystemService(Context.VIBRATOR_MANAGER_SERVICE) as? VibratorManager)?.defaultVibrator
            } else {
                @Suppress("DEPRECATION")
                context.getSystemService(Context.VIBRATOR_SERVICE) as? Vibrator
            }

            when (methodId) {
                1 -> { // Impact
                    val style = if (payload.isNotEmpty()) payload[0].toInt() else 1
                    val duration = when (style) {
                        1 -> 20L
                        2 -> 40L
                        else -> 70L
                    }
                    vibrator?.vibrate(VibrationEffect.createOneShot(duration, VibrationEffect.DEFAULT_AMPLITUDE))
                }
                2 -> { // Notification
                    vibrator?.vibrate(VibrationEffect.createOneShot(50L, VibrationEffect.DEFAULT_AMPLITUDE))
                }
                3 -> { // Selection
                    vibrator?.vibrate(VibrationEffect.createOneShot(10L, VibrationEffect.DEFAULT_AMPLITUDE))
                }
            }
        } catch (_: Exception) {}
        onResult(ModuleResultEncoder.success(callId))
    }
}

class SecureStoreNativeModule(val context: Context) : NativeModule {
    override val moduleId: Int = 6
    override val name: String = "ferrix-secure-store"
    private val prefs = context.getSharedPreferences("ferrix_secure_store_fallback", Context.MODE_PRIVATE)

    override fun invoke(callId: Int, methodId: Int, payload: ByteArray, onResult: (ByteArray) -> Unit) {
        val buf = ByteBuffer.wrap(payload).order(ByteOrder.LITTLE_ENDIAN)
        when (methodId) {
            1 -> { // set_item
                val kLen = buf.int
                val kBytes = ByteArray(kLen)
                buf.get(kBytes)
                val key = String(kBytes, Charsets.UTF_8)
                val vLen = buf.int
                val vBytes = ByteArray(vLen)
                buf.get(vBytes)
                val value = String(vBytes, Charsets.UTF_8)
                prefs.edit().putString(key, value).apply()
                onResult(ModuleResultEncoder.success(callId))
            }
            2 -> { // get_item
                val kLen = buf.int
                val kBytes = ByteArray(kLen)
                buf.get(kBytes)
                val key = String(kBytes, Charsets.UTF_8)
                val value = prefs.getString(key, null)
                if (value != null) {
                    onResult(ModuleResultEncoder.success(callId, value.toByteArray(Charsets.UTF_8)))
                } else {
                    onResult(ModuleResultEncoder.success(callId, ByteArray(0)))
                }
            }
            3 -> { // delete_item
                val kLen = buf.int
                val kBytes = ByteArray(kLen)
                buf.get(kBytes)
                val key = String(kBytes, Charsets.UTF_8)
                prefs.edit().remove(key).apply()
                onResult(ModuleResultEncoder.success(callId))
            }
            else -> onResult(ModuleResultEncoder.error(callId, "ERR_UNKNOWN_METHOD", "Unknown SecureStore method: $methodId"))
        }
    }
}

class FileSystemNativeModule(val context: Context) : NativeModule {
    override val moduleId: Int = 7
    override val name: String = "ferrix-file-system"

    override fun invoke(callId: Int, methodId: Int, payload: ByteArray, onResult: (ByteArray) -> Unit) {
        val buf = ByteBuffer.wrap(payload).order(ByteOrder.LITTLE_ENDIAN)
        when (methodId) {
            1 -> { // read_as_string
                val len = buf.int
                val bytes = ByteArray(len)
                buf.get(bytes)
                val path = String(bytes, Charsets.UTF_8)
                val file = File(path)
                if (file.exists()) {
                    onResult(ModuleResultEncoder.success(callId, file.readBytes()))
                } else {
                    onResult(ModuleResultEncoder.error(callId, "ERR_NOT_FOUND", "File not found: $path"))
                }
            }
            2 -> { // write_as_string
                val pLen = buf.int
                val pBytes = ByteArray(pLen)
                buf.get(pBytes)
                val path = String(pBytes, Charsets.UTF_8)
                val cLen = buf.int
                val cBytes = ByteArray(cLen)
                buf.get(cBytes)
                val content = String(cBytes, Charsets.UTF_8)
                File(path).writeText(content)
                onResult(ModuleResultEncoder.success(callId))
            }
            3 -> { // delete
                val len = buf.int
                val bytes = ByteArray(len)
                buf.get(bytes)
                val path = String(bytes, Charsets.UTF_8)
                File(path).delete()
                onResult(ModuleResultEncoder.success(callId))
            }
            4 -> { // read_directory
                val len = buf.int
                val bytes = ByteArray(len)
                buf.get(bytes)
                val path = String(bytes, Charsets.UTF_8)
                val files = File(path).list() ?: emptyArray()
                val joined = files.joinToString(";")
                onResult(ModuleResultEncoder.success(callId, joined.toByteArray(Charsets.UTF_8)))
            }
            else -> onResult(ModuleResultEncoder.error(callId, "ERR_UNKNOWN_METHOD", "Unknown FileSystem method: $methodId"))
        }
    }
}

class ImagePickerNativeModule(val context: Context) : NativeModule {
    override val moduleId: Int = 8
    override val name: String = "ferrix-image-picker"

    override fun invoke(callId: Int, methodId: Int, payload: ByteArray, onResult: (ByteArray) -> Unit) {
        val mockUri = "content://media/external/images/media/42"
        onResult(ModuleResultEncoder.success(callId, mockUri.toByteArray(Charsets.UTF_8)))
    }
}

class DocumentPickerNativeModule(val context: Context) : NativeModule {
    override val moduleId: Int = 9
    override val name: String = "ferrix-document-picker"

    override fun invoke(callId: Int, methodId: Int, payload: ByteArray, onResult: (ByteArray) -> Unit) {
        val mockUri = "content://com.android.providers.downloads.documents/document/101"
        onResult(ModuleResultEncoder.success(callId, mockUri.toByteArray(Charsets.UTF_8)))
    }
}

class MediaLibraryNativeModule(val context: Context) : NativeModule {
    override val moduleId: Int = 10
    override val name: String = "ferrix-media-library"

    override fun invoke(callId: Int, methodId: Int, payload: ByteArray, onResult: (ByteArray) -> Unit) {
        val mockCount = 128
        val buf = ByteBuffer.allocate(4).order(ByteOrder.LITTLE_ENDIAN)
        buf.putInt(mockCount)
        onResult(ModuleResultEncoder.success(callId, buf.array()))
    }
}

class NotificationsNativeModule(val context: Context) : NativeModule {
    override val moduleId: Int = 11
    override val name: String = "ferrix-notifications"
    override val requiredPermissions: List<String> = listOf("android.permission.POST_NOTIFICATIONS")

    override fun invoke(callId: Int, methodId: Int, payload: ByteArray, onResult: (ByteArray) -> Unit) {
        when (methodId) {
            1 -> onResult(ModuleResultEncoder.success(callId)) // schedule_notification
            2 -> { // get_push_token
                val token = "fcm_token_ferrix_sample_12345678"
                onResult(ModuleResultEncoder.success(callId, token.toByteArray(Charsets.UTF_8)))
            }
            3 -> onResult(ModuleResultEncoder.success(callId)) // set_channel
            else -> onResult(ModuleResultEncoder.error(callId, "ERR_UNKNOWN_METHOD", "Unknown Notification method: $methodId"))
        }
    }
}

class LocalAuthNativeModule(val context: Context) : NativeModule {
    override val moduleId: Int = 12
    override val name: String = "ferrix-local-authentication"
    override val requiredPermissions: List<String> = listOf("android.permission.USE_BIOMETRIC")

    override fun invoke(callId: Int, methodId: Int, payload: ByteArray, onResult: (ByteArray) -> Unit) {
        when (methodId) {
            1 -> onResult(ModuleResultEncoder.success(callId, byteArrayOf(1))) // has_hardware: true
            2 -> onResult(ModuleResultEncoder.success(callId, byteArrayOf(1))) // is_enrolled: true
            3 -> onResult(ModuleResultEncoder.success(callId, byteArrayOf(1))) // authenticate: true (authenticated)
            else -> onResult(ModuleResultEncoder.error(callId, "ERR_UNKNOWN_METHOD", "Unknown LocalAuth method: $methodId"))
        }
    }
}
