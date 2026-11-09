package com.ferrix.runtime

import android.content.Context
import android.view.View
import android.widget.Button
import android.widget.EditText
import android.widget.FrameLayout
import android.widget.ImageView
import android.widget.LinearLayout
import android.widget.TextView
import androidx.core.widget.NestedScrollView
import com.ferrix.runtime.protocol.ViewType

/**
 * Factory responsible for instantiating concrete native Android Views for Ferrix nodes.
 */
class FerrixViewFactory(private val context: Context) {

    fun createView(viewType: ViewType): View {
        return when (viewType) {
            ViewType.CONTAINER -> FrameLayout(context)
            ViewType.COLUMN -> LinearLayout(context).apply {
                orientation = LinearLayout.VERTICAL
            }
            ViewType.ROW -> LinearLayout(context).apply {
                orientation = LinearLayout.HORIZONTAL
            }
            ViewType.TEXT -> TextView(context)
            ViewType.BUTTON -> Button(context)
            ViewType.IMAGE -> ImageView(context)
            ViewType.TEXT_INPUT -> EditText(context)
            ViewType.SCROLL_VIEW -> NestedScrollView(context)
        }
    }
}
