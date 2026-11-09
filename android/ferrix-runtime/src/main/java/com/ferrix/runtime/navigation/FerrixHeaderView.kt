package com.ferrix.runtime.navigation

import android.content.Context
import android.graphics.Color
import android.graphics.Typeface
import android.view.Gravity
import android.widget.LinearLayout
import android.widget.TextView

/**
 * Native navigation header view with title and back navigation button.
 */
class FerrixHeaderView(context: Context) : LinearLayout(context) {
    val backButton: TextView = TextView(context)
    val titleView: TextView = TextView(context)
    var onBackClickListener: (() -> Unit)? = null

    init {
        orientation = HORIZONTAL
        gravity = Gravity.CENTER_VERTICAL
        setPadding(24, 16, 24, 16)
        setBackgroundColor(Color.parseColor("#1e293b"))
        layoutParams = LayoutParams(LayoutParams.MATCH_PARENT, LayoutParams.WRAP_CONTENT)

        backButton.text = "←"
        backButton.textSize = 22f
        backButton.setTextColor(Color.WHITE)
        backButton.setPadding(8, 8, 24, 8)
        backButton.setOnClickListener {
            onBackClickListener?.invoke()
        }
        addView(backButton)

        titleView.textSize = 18f
        titleView.typeface = Typeface.DEFAULT_BOLD
        titleView.setTextColor(Color.WHITE)
        titleView.layoutParams = LayoutParams(0, LayoutParams.WRAP_CONTENT, 1.0f)
        addView(titleView)
    }

    fun setTitle(title: String) {
        titleView.text = title
    }

    fun setBackButtonVisible(visible: Boolean) {
        backButton.visibility = if (visible) VISIBLE else GONE
    }
}

