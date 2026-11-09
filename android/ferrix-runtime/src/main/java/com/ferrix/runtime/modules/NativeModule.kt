package com.ferrix.runtime.modules

import java.nio.ByteBuffer
import java.nio.ByteOrder

/**
 * Common contract for all Expo-style native Android modules under Architecture A.
 */
interface NativeModule {
    val moduleId: Int
    val name: String
    val requiredPermissions: List<String> get() = emptyList()

    fun onStart() {}
    fun onStop() {}
    fun onDestroy() {}

    /**
     * Dispatches a method invocation without reflection.
     */
    fun invoke(callId: Int, methodId: Int, payload: ByteArray, onResult: (ByteArray) -> Unit)
}

/**
 * Utility for formatting binary module responses.
 */
object ModuleResultEncoder {
    fun success(callId: Int, payload: ByteArray = ByteArray(0)): ByteArray {
        val buffer = ByteBuffer.allocate(10 + payload.size).order(ByteOrder.LITTLE_ENDIAN)
        buffer.putShort(1) // Tag 1: Success
        buffer.putInt(callId)
        buffer.putInt(payload.size)
        buffer.put(payload)
        return buffer.array()
    }

    fun error(callId: Int, code: String, message: String): ByteArray {
        val cBytes = code.toByteArray(Charsets.UTF_8)
        val mBytes = message.toByteArray(Charsets.UTF_8)
        val buffer = ByteBuffer.allocate(14 + cBytes.size + mBytes.size).order(ByteOrder.LITTLE_ENDIAN)
        buffer.putShort(2) // Tag 2: Error
        buffer.putInt(callId)
        buffer.putInt(cBytes.size)
        buffer.put(cBytes)
        buffer.putInt(mBytes.size)
        buffer.put(mBytes)
        return buffer.array()
    }

    fun cancelled(callId: Int): ByteArray {
        val buffer = ByteBuffer.allocate(6).order(ByteOrder.LITTLE_ENDIAN)
        buffer.putShort(3) // Tag 3: Cancelled
        buffer.putInt(callId)
        return buffer.array()
    }
}
