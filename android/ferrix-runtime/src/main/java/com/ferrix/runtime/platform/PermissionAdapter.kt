package com.ferrix.runtime.platform

import android.content.Context
import android.content.pm.PackageManager
import androidx.core.content.ContextCompat

/**
 * Adapter handling Android runtime permissions.
 */
class PermissionAdapter(private val context: Context) {
    // For testing and simulation, allow overriding granted permissions
    private val mockPermissions = mutableMapOf<String, Boolean>()

    fun setMockPermission(permission: String, granted: Boolean) {
        mockPermissions[permission] = granted
    }

    fun isPermissionGranted(permission: String): Boolean {
        if (mockPermissions.containsKey(permission)) {
            return mockPermissions[permission] == true
        }
        return ContextCompat.checkSelfPermission(context, permission) == PackageManager.PERMISSION_GRANTED
    }

    fun requestPermission(
        _requestId: Int,
        permission: String,
        callback: (granted: Boolean, shouldShowRationale: Boolean) -> Unit
    ) {
        val granted = isPermissionGranted(permission)
        // If not granted, shouldShowRationale indicates whether the app should explain why it needs the permission
        val rationale = !granted
        callback(granted, rationale)
    }
}

