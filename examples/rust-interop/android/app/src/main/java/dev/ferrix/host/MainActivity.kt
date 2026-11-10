package dev.ferrix.host

import android.app.Activity
import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent
import android.content.IntentFilter
import android.os.Bundle
import android.util.Log
import android.widget.Button
import android.widget.FrameLayout
import androidx.core.splashscreen.SplashScreen.Companion.installSplashScreen

class MainActivity : Activity() {
    companion object {
        init {
            try {
                System.loadLibrary("rust_interop")
            } catch (e: UnsatisfiedLinkError) {
                Log.w("ferrix", "Native library load deferred: ${e.message}")
            }
        }
    }

    private val hotReloadReceiver = object : BroadcastReceiver() {
        override fun onReceive(context: Context?, intent: Intent?) {
            Log.i("ferrix", "ferrix:patch-applied")
            println("ferrix:patch-applied")
        }
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        try {
            installSplashScreen()
        } catch (e: Throwable) {
            Log.w("ferrix", "SplashScreen install skipped: ${e.message}")
        }
        super.onCreate(savedInstanceState)
        val rootLayout = FrameLayout(this).apply {
            tag = "ferrix:root"
        }

        // Scripted test-interaction button
        val testButton = Button(this).apply {
            text = "Ferrix Action"
            contentDescription = "ferrix:button"
            tag = "ferrix:button"
            setOnClickListener {
                Log.i("ferrix", "ferrix:button-clicked")
                println("ferrix:button-clicked")
            }
        }
        rootLayout.addView(testButton)
        setContentView(rootLayout)

        if (BuildConfig.DEBUG) {
            Log.i("ferrix", "ferrix:ready")
            println("ferrix:ready")
        }
    }

    override fun onResume() {
        super.onResume()
        if (BuildConfig.DEBUG) {
            Log.i("ferrix", "ferrix:ready")
        }
    }

    override fun onStart() {
        super.onStart()
        if (BuildConfig.DEBUG) {
            val filter = IntentFilter("com.ferrix.HOT_RELOAD")
            try {
                registerReceiver(hotReloadReceiver, filter, Context.RECEIVER_EXPORTED)
            } catch (e: NoSuchMethodError) {
                registerReceiver(hotReloadReceiver, filter)
            } catch (e: Exception) {
                Log.w("ferrix", "Could not register hot reload receiver: ${e.message}")
            }
        }
    }

    override fun onStop() {
        super.onStop()
        if (BuildConfig.DEBUG) {
            try {
                unregisterReceiver(hotReloadReceiver)
            } catch (e: Exception) {}
        }
    }
}
