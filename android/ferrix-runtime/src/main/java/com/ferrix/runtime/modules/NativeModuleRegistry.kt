package com.ferrix.runtime.modules

import android.content.Context
import java.nio.ByteBuffer
import java.nio.ByteOrder

/**
 * Static native module registry dispatching typed binary calls without reflection.
 */
class NativeModuleRegistry(val context: Context) {
    private val modules = mutableMapOf<Int, NativeModule>()

    init {
        register(CameraNativeModule(context))
        register(LocationNativeModule(context))
        register(SensorsNativeModule(context))
        register(AudioNativeModule(context))
        register(HapticsNativeModule(context))
        register(SecureStoreNativeModule(context))
        register(FileSystemNativeModule(context))
        register(ImagePickerNativeModule(context))
        register(DocumentPickerNativeModule(context))
        register(MediaLibraryNativeModule(context))
        register(NotificationsNativeModule(context))
        register(LocalAuthNativeModule(context))
    }

    fun register(module: NativeModule) {
        modules[module.moduleId] = module
    }

    fun get(moduleId: Int): NativeModule? = modules[moduleId]

    fun count(): Int = modules.size

    fun broadcastStart() {
        modules.values.forEach { it.onStart() }
    }

    fun broadcastStop() {
        modules.values.forEach { it.onStop() }
    }

    fun broadcastDestroy() {
        modules.values.forEach { it.onDestroy() }
    }

    /**
     * Decodes a binary ModuleCall buffer, routes to the registered module,
     * and returns the binary ModuleResult buffer.
     */
    fun handleCall(callBytes: ByteArray, onResponse: (ByteArray) -> Unit) {
        if (callBytes.size < 12) {
            onResponse(ModuleResultEncoder.error(0, "ERR_INVALID_BUFFER", "Buffer size < 12 bytes"))
            return
        }

        val buf = ByteBuffer.wrap(callBytes).order(ByteOrder.LITTLE_ENDIAN)
        val callId = buf.int
        val moduleId = buf.short.toInt()
        val methodId = buf.short.toInt()
        val payloadLen = buf.int
        val payload = ByteArray(payloadLen)
        if (buf.remaining() >= payloadLen) {
            buf.get(payload)
        }

        val module = modules[moduleId]
        if (module == null) {
            onResponse(ModuleResultEncoder.error(callId, "ERR_MODULE_NOT_FOUND", "Native module $moduleId not found"))
            return
        }

        module.invoke(callId, methodId, payload, onResponse)
    }
}
