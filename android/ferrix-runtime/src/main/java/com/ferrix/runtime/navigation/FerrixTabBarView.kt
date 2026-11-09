package com.ferrix.runtime.navigation

import android.content.Context
import android.graphics.Color
import android.view.Gravity
import android.widget.LinearLayout
import android.widget.TextView

/**
 * Native bottom tab bar view.
 */
class FerrixTabBarView(context: Context) : LinearLayout(context) {
    var selectedIndex: Int = 0
    var onTabSelectedListener: ((Int) -> Unit)? = null
    private val tabViews = ArrayList<TextView>()

    init {
        orientation = HORIZONTAL
        gravity = Gravity.CENTER
        setBackgroundColor(Color.parseColor("#0f172a"))
        layoutParams = LayoutParams(LayoutParams.MATCH_PARENT, 160)
    }

    fun setTabs(titles: List<String>) {
        removeAllViews()
        tabViews.clear()

        for ((index, title) in titles.withIndex()) {
            val tv = TextView(context).apply {
                text = title
                textSize = 14f
                gravity = Gravity.CENTER
                setTextColor(if (index == selectedIndex) Color.parseColor("#38bdf8") else Color.parseColor("#94a3b8"))
                layoutParams = LayoutParams(0, LayoutParams.MATCH_PARENT, 1.0f)
                setOnClickListener {
                    selectTab(index)
                    onTabSelectedListener?.invoke(index)
                }
            }
            tabViews.add(tv)
            addView(tv)
        }
    }

    fun selectTab(index: Int) {
        selectedIndex = index
        for ((i, tv) in tabViews.withIndex()) {
            tv.setTextColor(if (i == selectedIndex) Color.parseColor("#38bdf8") else Color.parseColor("#94a3b8"))
        }
    }
}
