package com.ferrix.runtime.platform

import android.content.Context
import android.content.SharedPreferences

/**
 * Adapter managing key-value storage persistence backed by Android SharedPreferences.
 */
class StorageAdapter(context: Context) {
    private val prefs: SharedPreferences = context.getSharedPreferences("ferrix_storage", Context.MODE_PRIVATE)

    fun read(key: String): String? {
        return prefs.getString(key, null)
    }

    fun write(key: String, value: String) {
        prefs.edit().putString(key, value).apply()
    }
}

