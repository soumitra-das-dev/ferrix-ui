package com.example.hello_counter

import android.app.Activity
import android.os.Bundle
import android.widget.FrameLayout

class MainActivity : Activity() {
    companion object {
        init {
            System.loadLibrary("hello-counter")
        }
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        val rootLayout = FrameLayout(this)
        setContentView(rootLayout)
    }
}
