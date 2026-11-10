package com.ferrix.runtime.navigation

import android.content.Context
import android.widget.FrameLayout

/**
 * Screen lifecycle states.
 */
enum class ScreenLifecycleState {
    ACTIVE,
    INACTIVE,
    TRANSITIONING
}

/**
 * Native Android container representing a single Ferrix Screen.
 */
class FerrixScreenView(context: Context) : FrameLayout(context) {
    var nodeId: Long = 0L
    var screenTitle: String = ""
    var headerShown: Boolean = true
    var presentation: Byte = 0 // 0 = Card, 1 = Modal, 2 = TransparentModal
    var lifecycleState: ScreenLifecycleState = ScreenLifecycleState.ACTIVE

    init {
        layoutParams = LayoutParams(LayoutParams.MATCH_PARENT, LayoutParams.MATCH_PARENT)
    }

    fun onScreenFocused() {
        lifecycleState = ScreenLifecycleState.ACTIVE
        visibility = VISIBLE
    }

    fun onScreenBlurred() {
        lifecycleState = ScreenLifecycleState.INACTIVE
        visibility = GONE
    }
}

