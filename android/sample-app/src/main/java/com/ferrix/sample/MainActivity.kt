package com.ferrix.sample

import android.os.Bundle
import android.widget.LinearLayout
import androidx.appcompat.app.AppCompatActivity
import com.ferrix.runtime.FerrixBridge

class MainActivity : AppCompatActivity() {
    private val bridge = FerrixBridge()

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        // Root container for native Ferrix elements
        val rootLayout = LinearLayout(this).apply {
            orientation = LinearLayout.VERTICAL
            layoutParams = LinearLayout.LayoutParams(
                LinearLayout.LayoutParams.MATCH_PARENT,
                LinearLayout.LayoutParams.MATCH_PARENT
            )
        }
        setContentView(rootLayout)

        // Mount Rust UI into native View container
        bridge.start(this, rootLayout, "platform")
    }

    override fun onDestroy() {
        super.onDestroy()
        bridge.destroy()
    }
}
