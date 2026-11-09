package com.ferrix.runtime

import android.content.Context
import android.os.Handler
import android.os.Looper
import android.view.ViewGroup
import com.ferrix.runtime.platform.PlatformManager
import com.ferrix.runtime.protocol.CommandDecoder
import com.ferrix.runtime.protocol.EventType
import com.ferrix.runtime.protocol.Patch
import java.nio.ByteBuffer

/**
 * JNI bridge between the Kotlin UI renderer and the native Rust Ferrix runtime.
 */
class FerrixBridge {
    private var nativeHandle: Long = 0L
    private val mainHandler = Handler(Looper.getMainLooper())
    private var renderer: FerrixRenderer? = null

    /**
     * Platform capabilities coordinator (permissions, location, camera, storage).
     */
    var platformManager: PlatformManager? = null
        private set

    /**
     * Counter for JNI calls made during event and platform dispatches.
     */
    var jniCallCount: Int = 0
        private set

    /**
     * Last batch of patches received from a JNI event dispatch.
     */
    var lastEmittedPatches: List<Patch> = emptyList()
        private set

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
    fun start(context: Context, rootContainer: ViewGroup, appName: String = "counter"): FerrixRenderer {
        nativeHandle = if (appName == "counter") {
            nativeInit()
        } else {
            nativeInitApp(appName)
        }
        check(nativeHandle != 0L) { "Failed to initialize native Ferrix runtime for $appName" }

        val activeRenderer = FerrixRenderer(context, rootContainer)
        renderer = activeRenderer
        platformManager = PlatformManager(context)

        // Retrieve and apply initial patch buffer
        val initialBuffer = nativeGetInitialPatches(nativeHandle)
        if (initialBuffer != null) {
            val patches = CommandDecoder.decode(initialBuffer)
            activeRenderer.applyPatches(patches) { nodeId, eventType, payload ->
                dispatchEvent(nodeId, eventType, payload)
            }
        }

        // Poll any initial platform requests
        pollAndHandlePlatformRequests()

        return activeRenderer
    }

    /**
     * Dispatches a UI event to Rust and applies any resulting patch buffer on the main thread.
     */
    fun dispatchEvent(nodeId: Long, eventType: EventType, payload: ByteArray?) {
        if (nativeHandle == 0L) return

        jniCallCount++

        val responseBuffer = nativeDispatchEvent(
            nativeHandle,
            nodeId,
            eventType.value.toInt(),
            payload
        )

        applyPatchBuffer(responseBuffer)
        pollAndHandlePlatformRequests()
    }

    /**
     * Delivers a native platform response back to Rust and applies any resulting UI updates.
     */
    fun dispatchPlatformResponse(responseBytes: ByteArray) {
        if (nativeHandle == 0L) return

        jniCallCount++

        val patchBuffer = nativeDispatchPlatformResponse(nativeHandle, responseBytes)
        applyPatchBuffer(patchBuffer)
        pollAndHandlePlatformRequests()
    }

    private fun applyPatchBuffer(buffer: ByteBuffer?) {
        if (buffer != null) {
            val patches = CommandDecoder.decode(buffer)
            lastEmittedPatches = patches
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
        } else {
            lastEmittedPatches = emptyList()
        }
    }

    /**
     * Checks if Rust has queued any outbound platform requests and executes them.
     */
    fun pollAndHandlePlatformRequests() {
        if (nativeHandle == 0L) return
        val mgr = platformManager ?: return

        while (true) {
            val reqBuffer = nativePollPlatformRequests(nativeHandle) ?: break
            val reqBytes = ByteArray(reqBuffer.remaining())
            reqBuffer.get(reqBytes)

            mgr.handleRequest(reqBytes) { respBytes ->
                dispatchPlatformResponse(respBytes)
            }
        }
    }

    /**
     * Tears down the native Rust runtime and releases hardware resources.
     */
    fun destroy() {
        platformManager?.destroy()
        platformManager = null

        if (nativeHandle != 0L) {
            nativeDestroy(nativeHandle)
            nativeHandle = 0L
        }
        renderer = null
    }

    // Native JNI functions implemented in crates/ferrix-android
    private external fun nativeInit(): Long
    private external fun nativeInitApp(appName: String): Long
    private external fun nativePollPlatformRequests(handle: Long): ByteBuffer?
    private external fun nativeDispatchPlatformResponse(handle: Long, responseBytes: ByteArray): ByteBuffer?
    private external fun nativeGetInitialPatches(handle: Long): ByteBuffer?
    private external fun nativeDispatchEvent(
        handle: Long,
        nodeId: Long,
        eventType: Int,
        payload: ByteArray?
    ): ByteBuffer?
    private external fun nativeDestroy(handle: Long)
}
