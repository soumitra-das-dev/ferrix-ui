package com.ferrix.runtime.navigation

import android.animation.Animator
import android.animation.AnimatorListenerAdapter
import android.content.Context
import android.view.View
import android.widget.FrameLayout
import android.widget.LinearLayout

/**
 * Root native navigation container managing screen stacks and transitions.
 */
class FerrixNavContainer(context: Context) : LinearLayout(context) {
    val headerView = FerrixHeaderView(context)
    val screenContainer = FrameLayout(context)
    val tabBarView = FerrixTabBarView(context)
    private val stack = ArrayList<FerrixScreenView>()

    init {
        orientation = VERTICAL
        layoutParams = LayoutParams(LayoutParams.MATCH_PARENT, LayoutParams.MATCH_PARENT)

        addView(headerView)

        screenContainer.layoutParams = LayoutParams(LayoutParams.MATCH_PARENT, 0, 1.0f)
        addView(screenContainer)

        tabBarView.visibility = GONE
        addView(tabBarView)

        headerView.onBackClickListener = {
            popScreen(0)
        }
    }

    fun canGoBack(): Boolean = stack.size > 1

    fun getActiveScreen(): FerrixScreenView? = stack.lastOrNull()

    fun getStackSize(): Int = stack.size

    fun pushScreen(screen: FerrixScreenView, animation: Byte = 0) {
        val prevScreen = stack.lastOrNull()
        stack.add(screen)

        if (screenContainer.indexOfChild(screen) == -1) {
            screenContainer.addView(screen)
        }

        screen.onScreenFocused()
        updateHeaderForScreen(screen)

        // Perform hardware-accelerated transition
        when (animation.toInt()) {
            1 -> { // Fade
                screen.alpha = 0f
                screen.animate().alpha(1f).setDuration(250).start()
                prevScreen?.let { prev ->
                    prev.animate().alpha(0f).setDuration(250).setListener(object : AnimatorListenerAdapter() {
                        override fun onAnimationEnd(animation: Animator) {
                            prev.onScreenBlurred()
                        }
                    }).start()
                }
            }
            3 -> { // SlideFromBottom (Modal)
                screen.translationY = screenContainer.height.toFloat().takeIf { it > 0 } ?: 1200f
                screen.animate().translationY(0f).setDuration(300).start()
            }
            4 -> { // None
                screen.translationX = 0f
                screen.translationY = 0f
                screen.alpha = 1f
                prevScreen?.onScreenBlurred()
            }
            else -> { // Default: SlideFromRight
                val width = screenContainer.width.toFloat().takeIf { it > 0 } ?: 1080f
                screen.translationX = width
                screen.animate().translationX(0f).setDuration(280).start()
                prevScreen?.let { prev ->
                    prev.animate().translationX(-width / 3f).setDuration(280).setListener(object : AnimatorListenerAdapter() {
                        override fun onAnimationEnd(animation: Animator) {
                            prev.onScreenBlurred()
                        }
                    }).start()
                }
            }
        }
    }

    fun popScreen(animation: Byte = 0): Boolean {
        if (stack.size <= 1) return false

        val outgoing = stack.removeAt(stack.size - 1)
        val incoming = stack.last()

        incoming.onScreenFocused()
        incoming.translationX = 0f
        incoming.translationY = 0f
        incoming.alpha = 1f
        updateHeaderForScreen(incoming)

        when (animation.toInt()) {
            1 -> { // Fade
                outgoing.animate().alpha(0f).setDuration(200).setListener(object : AnimatorListenerAdapter() {
                    override fun onAnimationEnd(anim: Animator) {
                        screenContainer.removeView(outgoing)
                    }
                }).start()
            }
            3 -> { // SlideFromBottom
                val height = screenContainer.height.toFloat().takeIf { it > 0 } ?: 1200f
                outgoing.animate().translationY(height).setDuration(250).setListener(object : AnimatorListenerAdapter() {
                    override fun onAnimationEnd(anim: Animator) {
                        screenContainer.removeView(outgoing)
                    }
                }).start()
            }
            4 -> { // None
                screenContainer.removeView(outgoing)
            }
            else -> { // Slide to right
                val width = screenContainer.width.toFloat().takeIf { it > 0 } ?: 1080f
                outgoing.animate().translationX(width).setDuration(250).setListener(object : AnimatorListenerAdapter() {
                    override fun onAnimationEnd(anim: Animator) {
                        screenContainer.removeView(outgoing)
                    }
                }).start()
            }
        }
        return true
    }

    fun replaceScreen(screen: FerrixScreenView, animation: Byte = 0) {
        if (stack.isNotEmpty()) {
            val old = stack.removeAt(stack.size - 1)
            screenContainer.removeView(old)
        }
        pushScreen(screen, animation)
    }

    fun setScreenOptions(screenId: Long, title: String, headerShown: Boolean, presentation: Byte) {
        val screen = stack.find { it.nodeId == screenId } ?: return
        screen.screenTitle = title
        screen.headerShown = headerShown
        screen.presentation = presentation

        if (screen == stack.lastOrNull()) {
            updateHeaderForScreen(screen)
        }
    }

    private fun updateHeaderForScreen(screen: FerrixScreenView) {
        headerView.visibility = if (screen.headerShown) VISIBLE else GONE
        headerView.setTitle(screen.screenTitle)
        headerView.setBackButtonVisible(stack.size > 1)
    }
}
