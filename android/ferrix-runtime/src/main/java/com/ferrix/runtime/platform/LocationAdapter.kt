package com.ferrix.runtime.platform

import android.content.Context
import android.location.Location
import android.location.LocationListener
import android.location.LocationManager
import android.os.Bundle

/**
 * Adapter interfacing with Android Fused / System Location services.
 */
class LocationAdapter(private val context: Context) {
    private val locationManager = context.getSystemService(Context.LOCATION_SERVICE) as? LocationManager
    var isTracking: Boolean = false
        private set

    private var activeCallback: ((Double, Double, Float, Long) -> Unit)? = null

    private val listener = object : LocationListener {
        override fun onLocationChanged(loc: Location) {
            activeCallback?.invoke(loc.latitude, loc.longitude, loc.accuracy, loc.time)
        }

        @Deprecated("Deprecated in Java")
        override fun onStatusChanged(provider: String?, status: Int, extras: Bundle?) {}
        override fun onProviderEnabled(provider: String) {}
        override fun onProviderDisabled(provider: String) {}
    }

    fun startUpdates(
        minIntervalMs: Long,
        minDistanceM: Float,
        callback: (latitude: Double, longitude: Double, accuracy: Float, timestamp: Long) -> Unit
    ) {
        activeCallback = callback
        isTracking = true

        try {
            if (locationManager != null && locationManager.isProviderEnabled(LocationManager.GPS_PROVIDER)) {
                locationManager.requestLocationUpdates(
                    LocationManager.GPS_PROVIDER,
                    minIntervalMs,
                    minDistanceM,
                    listener
                )
            }
        } catch (_: SecurityException) {
            // Handled when permissions are missing
        }
    }

    /**
     * Emits a simulated location update (useful for testing or emulator fallback).
     */
    fun emitLocation(latitude: Double, longitude: Double, accuracy: Float, timestamp: Long) {
        if (isTracking) {
            activeCallback?.invoke(latitude, longitude, accuracy, timestamp)
        }
    }

    fun stopUpdates() {
        if (isTracking) {
            try {
                locationManager?.removeUpdates(listener)
            } catch (_: Exception) {}
            isTracking = false
            activeCallback = null
        }
    }
}

