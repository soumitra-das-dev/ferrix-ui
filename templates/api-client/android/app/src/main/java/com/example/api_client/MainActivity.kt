package com.example.api_client

import android.app.Activity
import android.os.Bundle
import android.widget.FrameLayout

class MainActivity : Activity() {
    companion object {
        init {
            System.loadLibrary("api-client")
        }
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        val rootLayout = FrameLayout(this)
        setContentView(rootLayout)
    }
}
