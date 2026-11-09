package com.ferrix.runtime.platform

import android.content.Context
import java.nio.ByteBuffer

/**
 * Central coordinator for native Android platform capability adapters.
 */
class PlatformManager(val context: Context) {
    val permissionAdapter = PermissionAdapter(context)
    val locationAdapter = LocationAdapter(context)
    val cameraAdapter = CameraAdapter(context)
    val storageAdapter = StorageAdapter(context)

    /**
     * Processes an inbound binary platform request buffer from Rust.
     */
    fun handleRequest(requestBytes: ByteArray, onResponse: (ByteArray) -> Unit) {
        val request = PlatformRequest.decode(ByteBuffer.wrap(requestBytes))
        when (request) {
            is PlatformRequest.RequestPermission -> {
                permissionAdapter.requestPermission(request.requestId, request.permission) { granted, rationale ->
                    val resp = PlatformResponse.PermissionResult(request.requestId, granted, rationale)
                    onResponse(resp.encode())
                }
            }
            is PlatformRequest.StartLocationUpdates -> {
                locationAdapter.startUpdates(request.minIntervalMs, request.minDistanceM) { lat, lng, acc, time ->
                    val resp = PlatformResponse.LocationUpdate(lat, lng, acc, time)
                    onResponse(resp.encode())
                }
            }
            is PlatformRequest.StopLocationUpdates -> {
                locationAdapter.stopUpdates()
            }
            is PlatformRequest.StartCameraPreview -> {
                cameraAdapter.startPreview(request.targetNodeId) { nodeId, width, height ->
                    val resp = PlatformResponse.CameraReady(nodeId, width, height)
                    onResponse(resp.encode())
                }
            }
            is PlatformRequest.StopCamera -> {
                cameraAdapter.stopCamera()
            }
            is PlatformRequest.TakePicture -> {
                cameraAdapter.takePicture(request.requestId) { reqId, path, width, height ->
                    val resp = PlatformResponse.PictureTaken(reqId, path, width, height)
                    onResponse(resp.encode())
                }
            }
            is PlatformRequest.ReadStorage -> {
                val value = storageAdapter.read(request.key)
                val resp = PlatformResponse.StorageResult(request.requestId, request.key, value)
                onResponse(resp.encode())
            }
            is PlatformRequest.WriteStorage -> {
                storageAdapter.write(request.key, request.value)
                val resp = PlatformResponse.StorageResult(request.requestId, request.key, request.value)
                onResponse(resp.encode())
            }
        }
    }

    /**
     * Tears down all active hardware and provider subscriptions.
     */
    fun destroy() {
        locationAdapter.stopUpdates()
        cameraAdapter.stopCamera()
    }
}

