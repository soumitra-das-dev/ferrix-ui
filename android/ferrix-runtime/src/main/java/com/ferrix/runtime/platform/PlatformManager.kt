package com.ferrix.runtime.platform

import android.app.Activity
import android.app.AlertDialog
import android.content.ClipData
import android.content.ClipboardManager
import android.content.Context
import android.content.Intent
import android.net.Uri
import android.os.Build
import android.os.VibrationEffect
import android.os.Vibrator
import android.os.VibratorManager
import android.view.View
import android.view.inputmethod.InputMethodManager
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
            is PlatformRequest.Vibrate -> {
                try {
                    if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
                        val vibratorManager = context.getSystemService(Context.VIBRATOR_MANAGER_SERVICE) as? VibratorManager
                        val vibrator = vibratorManager?.defaultVibrator
                        vibrator?.vibrate(VibrationEffect.createOneShot(request.durationMs, VibrationEffect.DEFAULT_AMPLITUDE))
                    } else {
                        @Suppress("DEPRECATION")
                        val vibrator = context.getSystemService(Context.VIBRATOR_SERVICE) as? Vibrator
                        @Suppress("DEPRECATION")
                        vibrator?.vibrate(request.durationMs)
                    }
                } catch (_: Exception) {
                    // Suppress vibration permission / device failures gracefully
                }
            }
            is PlatformRequest.SetClipboard -> {
                val clipboard = context.getSystemService(Context.CLIPBOARD_SERVICE) as? ClipboardManager
                val clip = ClipData.newPlainText("Ferrix", request.text)
                clipboard?.setPrimaryClip(clip)
            }
            is PlatformRequest.GetClipboard -> {
                val clipboard = context.getSystemService(Context.CLIPBOARD_SERVICE) as? ClipboardManager
                val text = clipboard?.primaryClip?.getItemAt(0)?.text?.toString() ?: ""
                val resp = PlatformResponse.ClipboardContent(request.requestId, text)
                onResponse(resp.encode())
            }
            is PlatformRequest.Share -> {
                val intent = Intent(Intent.ACTION_SEND).apply {
                    type = "text/plain"
                    putExtra(Intent.EXTRA_TITLE, request.title)
                    putExtra(Intent.EXTRA_TEXT, request.message)
                    addFlags(Intent.FLAG_ACTIVITY_NEW_TASK)
                }
                val chooser = Intent.createChooser(intent, request.title).apply {
                    addFlags(Intent.FLAG_ACTIVITY_NEW_TASK)
                }
                context.startActivity(chooser)
            }
            is PlatformRequest.OpenUrl -> {
                try {
                    val intent = Intent(Intent.ACTION_VIEW, Uri.parse(request.url)).apply {
                        addFlags(Intent.FLAG_ACTIVITY_NEW_TASK)
                    }
                    context.startActivity(intent)
                } catch (_: Exception) {
                    // Ignore malformed URL / no activity found
                }
            }
            is PlatformRequest.ShowAlert -> {
                try {
                    AlertDialog.Builder(context)
                        .setTitle(request.title)
                        .setMessage(request.message)
                        .setPositiveButton("OK", null)
                        .show()
                } catch (_: Exception) {
                    // Ignore window token issues in headless/background tests
                }
            }
            is PlatformRequest.ExitApp -> {
                if (context is Activity) {
                    context.finishAffinity()
                }
            }
            is PlatformRequest.DismissKeyboard -> {
                val imm = context.getSystemService(Context.INPUT_METHOD_SERVICE) as? InputMethodManager
                val view = (context as? Activity)?.currentFocus ?: View(context)
                imm?.hideSoftInputFromWindow(view.windowToken, 0)
            }
            is PlatformRequest.AnnounceAccessibility -> {
                val view = (context as? Activity)?.window?.decorView ?: View(context)
                view.announceForAccessibility(request.message)
            }
        }
    }

    /**
     * Dispatches dimension metrics to Rust.
     */
    fun sendDimensions(width: Float, height: Float, scale: Float, fontScale: Float, onResponse: (ByteArray) -> Unit) {
        val resp = PlatformResponse.DimensionsChange(width, height, scale, fontScale)
        onResponse(resp.encode())
    }

    /**
     * Dispatches keyboard appearance change to Rust.
     */
    fun sendKeyboardChange(isVisible: Boolean, heightDp: Float, onResponse: (ByteArray) -> Unit) {
        val resp = PlatformResponse.KeyboardChange(isVisible, heightDp)
        onResponse(resp.encode())
    }

    /**
     * Dispatches app lifecycle state to Rust (0=active, 1=background, 2=inactive).
     */
    fun sendAppState(state: Byte, onResponse: (ByteArray) -> Unit) {
        val resp = PlatformResponse.AppStateChange(state)
        onResponse(resp.encode())
    }

    /**
     * Dispatches network status changes to Rust.
     */
    fun sendNetInfo(isConnected: Boolean, isWifi: Boolean, onResponse: (ByteArray) -> Unit) {
        val resp = PlatformResponse.NetInfoUpdate(isConnected, isWifi)
        onResponse(resp.encode())
    }

    /**
     * Dispatches hardware back press event to Rust.
     */
    fun sendBackPressed(onResponse: (ByteArray) -> Unit) {
        val resp = PlatformResponse.BackPressed
        onResponse(resp.encode())
    }

    /**
     * Tears down all active hardware and provider subscriptions.
     */
    fun destroy() {
        locationAdapter.stopUpdates()
        cameraAdapter.stopCamera()
    }
}
