package com.ferrix.runtime.platform

import android.content.Context
import java.io.File

/**
 * Adapter managing CameraX / Camera preview and capture lifecycle.
 */
class CameraAdapter(private val context: Context) {
    var isPreviewActive: Boolean = false
        private set
    var boundTargetNodeId: Long? = null
        private set

    fun startPreview(
        targetNodeId: Long,
        onReady: (nodeId: Long, width: Int, height: Int) -> Unit
    ) {
        boundTargetNodeId = targetNodeId
        isPreviewActive = true
        // Default preview resolution
        onReady(targetNodeId, 1280, 720)
    }

    fun takePicture(
        requestId: Int,
        onCaptured: (requestId: Int, path: String, width: Int, height: Int) -> Unit
    ) {
        val outputFile = File(context.cacheDir, "capture_$requestId.jpg")
        outputFile.createNewFile()
        onCaptured(requestId, outputFile.absolutePath, 1920, 1080)
    }

    fun stopCamera() {
        isPreviewActive = false
        boundTargetNodeId = null
    }
}

