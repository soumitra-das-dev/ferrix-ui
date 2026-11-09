package com.ferrix.runtime

import android.content.Context
import android.os.Handler
import android.os.Looper
import android.view.ViewGroup
import com.ferrix.runtime.protocol.CommandDecoder
import com.ferrix.runtime.protocol.EventType
import java.nio.ByteBuffer

/**
 * JNI bridge between the Kotlin UI renderer and the native Rust Ferrix runtime.
 */
class FerrixBridge {
    private var nativeHandle: Long = 0L
    private val mainHandler = Handler(Looper.getMainLooper())
    private var renderer: FerrixRenderer? = null

    companion object {
        init {
            try {
                System.loadLibrary("ferrix_android")
            } catch (e: UnsatisfiedLinkError) {
                // Allows running tests in environments where native .so is loaded dynamically or mocked
            }
        }
    }

    /**
     * Initializes the Rust runtime and mounts the UI into [rootContainer].
     */
    fun start(context: Context, rootContainer: ViewGroup): FerrixRenderer {
        nativeHandle = nativeInit()
        check(nativeHandle != 0L) { "Failed to initialize native Ferrix runtime" }

        val activeRenderer = FerrixRenderer(context, rootContainer)
        renderer = activeRenderer

        // Retrieve and apply initial patch buffer
        val initialBuffer = nativeGetInitialPatches(nativeHandle)
        if (initialBuffer != null) {
            val patches = CommandDecoder.decode(initialBuffer)
            activeRenderer.applyPatches(patches) { nodeId, eventType, payload ->
                dispatchEvent(nodeId, eventType, payload)
            }
        }

        return activeRenderer
    }

    /**
     * Dispatches a UI event to Rust and applies any resulting patch buffer on the main thread.
     */
    fun dispatchEvent(nodeId: Long, eventType: EventType, payload: ByteArray?) {
        if (nativeHandle == 0L) return

        val responseBuffer = nativeDispatchEvent(
            nativeHandle,
            nodeId,
            eventType.value.toInt(),
            payload
        )

        if (responseBuffer != null) {
            val patches = CommandDecoder.decode(responseBuffer)
            if (Looper.myLooper() == Looper.getMainLooper()) {
                renderer?.applyPatches(patches) { id, type, data ->
                    dispatchEvent(id, type, data)
                }
            } else {
                mainHandler.post {
                    renderer?.applyPatches(patches) { id, type, data ->
                        dispatchEvent(id, type, data)
                    }
                }
            }
        }
    }

    /**
     * Tears down the native Rust runtime.
     */
    fun destroy() {
        if (nativeHandle != 0L) {
            nativeDestroy(nativeHandle)
            nativeHandle = 0L
        }
        renderer = null
    }

    // Native JNI functions implemented in crates/ferrix-android
    private external fun nativeInit(): Long
    private external fun nativeGetInitialPatches(handle: Long): ByteBuffer?
    private external fun nativeDispatchEvent(
        handle: Long,
        nodeId: Long,
        eventType: Int,
        payload: ByteArray?
    ): ByteBuffer?
    private external fun nativeDestroy(handle: Long)
}
