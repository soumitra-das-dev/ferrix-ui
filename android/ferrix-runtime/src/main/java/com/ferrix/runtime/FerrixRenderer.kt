package com.ferrix.runtime

import android.content.Context
import android.graphics.Outline
import android.graphics.drawable.ShapeDrawable
import android.os.Looper
import android.util.TypedValue
import android.view.View
import android.view.ViewGroup
import android.view.ViewOutlineProvider
import android.widget.Button
import android.widget.EditText
import android.widget.ImageView
import android.widget.LinearLayout
import android.widget.TextView
import androidx.collection.LongSparseArray
import com.ferrix.runtime.protocol.EventType
import com.ferrix.runtime.protocol.Patch
import com.ferrix.runtime.protocol.Property
import com.ferrix.runtime.protocol.ViewType
import com.ferrix.runtime.protocol.Visibility

/**
 * Concrete Android Views renderer for Ferrix.
 * Enforces execution strictly on the Android Main Thread.
 */
class FerrixRenderer(
    private val context: Context,
    private val rootContainer: ViewGroup
) {
    private val viewFactory = FerrixViewFactory(context)
    private val nodes = LongSparseArray<View>()

    init {
        // Register the host root view as NodeId 1 if designated as root
        nodes.put(1L, rootContainer)
    }

    fun getView(nodeId: Long): View? = nodes.get(nodeId)

    /**
     * Applies a batch of UI patches to the native View tree.
     */
    fun applyPatches(
        patches: List<Patch>,
        eventDispatcher: (Long, EventType, ByteArray?) -> Unit
    ) {
        check(Looper.myLooper() == Looper.getMainLooper()) {
            "FerrixRenderer.applyPatches must be executed on the Android Main Thread"
        }

        for (patch in patches) {
            when (patch) {
                is Patch.CreateNode -> {
                    if (patch.nodeId == 1L) {
                        // Root container already provided by Activity
                        if (rootContainer is LinearLayout) {
                            rootContainer.orientation = if (patch.viewType == ViewType.ROW) {
                                LinearLayout.HORIZONTAL
                            } else {
                                LinearLayout.VERTICAL
                            }
                        }
                    } else {
                        val view = viewFactory.createView(patch.viewType)
                        nodes.put(patch.nodeId, view)
                    }
                }
                is Patch.AppendChild -> {
                    val parent = nodes.get(patch.parentId) as? ViewGroup
                    val child = nodes.get(patch.childId)
                    if (parent != null && child != null) {
                        if (child.parent != parent) {
                            (child.parent as? ViewGroup)?.removeView(child)
                            parent.addView(child)
                        }
                    }
                }
                is Patch.InsertBefore -> {
                    val parent = nodes.get(patch.parentId) as? ViewGroup
                    val child = nodes.get(patch.childId)
                    val before = nodes.get(patch.beforeId)
                    if (parent != null && child != null) {
                        (child.parent as? ViewGroup)?.removeView(child)
                        val index = if (before != null) parent.indexOfChild(before) else -1
                        if (index >= 0) {
                            parent.addView(child, index)
                        } else {
                            parent.addView(child)
                        }
                    }
                }
                is Patch.RemoveChild -> {
                    val parent = nodes.get(patch.parentId) as? ViewGroup
                    val child = nodes.get(patch.childId)
                    if (parent != null && child != null) {
                        parent.removeView(child)
                    }
                }
                is Patch.DestroyNode -> {
                    val view = nodes.get(patch.nodeId)
                    if (view != null) {
                        (view.parent as? ViewGroup)?.removeView(view)
                        nodes.remove(patch.nodeId)
                    }
                }
                is Patch.SetText -> {
                    val view = nodes.get(patch.nodeId)
                    if (view is TextView) {
                        if (view.text.toString() != patch.text) {
                            val prevTag = view.tag
                            view.tag = "PROGRAMMATIC_UPDATE"
                            view.text = patch.text
                            view.tag = prevTag
                        }
                    }
                }
                is Patch.SetLayout -> {
                    val view = nodes.get(patch.nodeId)
                    if (view != null) {
                        val lp = view.layoutParams ?: ViewGroup.MarginLayoutParams(
                            ViewGroup.LayoutParams.WRAP_CONTENT,
                            ViewGroup.LayoutParams.WRAP_CONTENT
                        )
                        if (patch.width > 0) lp.width = patch.width.toInt()
                        if (patch.height > 0) lp.height = patch.height.toInt()
                        view.layoutParams = lp
                    }
                }
                is Patch.SetProperty -> {
                    val view = nodes.get(patch.nodeId) ?: continue
                    when (val prop = patch.property) {
                        is Property.BackgroundColor -> view.setBackgroundColor(prop.color)
                        is Property.TextColor -> (view as? TextView)?.setTextColor(prop.color)
                        is Property.FontSize -> (view as? TextView)?.setTextSize(
                            TypedValue.COMPLEX_UNIT_SP,
                            prop.sp
                        )
                        is Property.Padding -> {
                            val density = context.resources.displayMetrics.density
                            view.setPadding(
                                (prop.left * density).toInt(),
                                (prop.top * density).toInt(),
                                (prop.right * density).toInt(),
                                (prop.bottom * density).toInt()
                            )
                        }
                        is Property.Spacing -> {
                            if (view is LinearLayout) {
                                val density = context.resources.displayMetrics.density
                                val spacePx = (prop.dp * density).toInt()
                                val shape = ShapeDrawable().apply {
                                    if (view.orientation == LinearLayout.VERTICAL) {
                                        intrinsicHeight = spacePx
                                    } else {
                                        intrinsicWidth = spacePx
                                    }
                                    alpha = 0
                                }
                                view.dividerDrawable = shape
                                view.showDividers = LinearLayout.SHOW_DIVIDER_MIDDLE
                            }
                        }
                        is Property.CornerRadius -> {
                            val density = context.resources.displayMetrics.density
                            val radiusPx = prop.dp * density
                            view.clipToOutline = true
                            view.outlineProvider = object : ViewOutlineProvider() {
                                override fun getOutline(view: View, outline: Outline) {
                                    outline.setRoundRect(0, 0, view.width, view.height, radiusPx)
                                }
                            }
                        }
                        is Property.Enabled -> view.isEnabled = prop.enabled
                        is Property.ImageUrl -> {
                            if (view is ImageView) {
                                view.tag = prop.url
                                view.contentDescription = prop.url
                                // Set visible placeholder for remote / test images
                                view.setBackgroundColor(0xFF3B82F6.toInt())
                            }
                        }
                        is Property.AnimateAlpha -> {
                            view.animate().alpha(prop.target).setDuration(prop.durationMs).start()
                        }
                        is Property.AnimateScale -> {
                            view.animate().scaleX(prop.target).scaleY(prop.target).setDuration(prop.durationMs).start()
                        }
                        is Property.AnimateTranslationY -> {
                            view.animate().translationY(prop.target).setDuration(prop.durationMs).start()
                        }
                        is Property.ContentDescription -> view.contentDescription = prop.description
                        is Property.AccessibilityRole -> view.tag = prop.role
                        is Property.AccessibilityHeading -> {
                            if (android.os.Build.VERSION.SDK_INT >= android.os.Build.VERSION_CODES.P) {
                                view.isAccessibilityHeading = prop.isHeading
                            }
                        }
                        is Property.Hint -> (view as? EditText)?.hint = prop.hint
                    }
                }
                is Patch.SetVisibility -> {
                    val view = nodes.get(patch.nodeId) ?: continue
                    view.visibility = when (patch.visibility) {
                        Visibility.VISIBLE -> View.VISIBLE
                        Visibility.INVISIBLE -> View.INVISIBLE
                        Visibility.GONE -> View.GONE
                    }
                }
                is Patch.AttachEventListener -> {
                    val view = nodes.get(patch.nodeId) ?: continue
                    when (patch.eventType) {
                        EventType.CLICK -> {
                            view.setOnClickListener {
                                eventDispatcher(patch.nodeId, EventType.CLICK, null)
                            }
                        }
                        EventType.TEXT_CHANGED -> {
                            if (view is EditText) {
                                view.addTextChangedListener(object : android.text.TextWatcher {
                                    override fun beforeTextChanged(s: CharSequence?, start: Int, count: Int, after: Int) {}
                                    override fun onTextChanged(s: CharSequence?, start: Int, before: Int, count: Int) {
                                        if (view.tag == "PROGRAMMATIC_UPDATE") return
                                        val textBytes = (s?.toString() ?: "").toByteArray(java.nio.charset.StandardCharsets.UTF_8)
                                        eventDispatcher(patch.nodeId, EventType.TEXT_CHANGED, textBytes)
                                    }
                                    override fun afterTextChanged(s: android.text.Editable?) {}
                                })
                            }
                        }
                        else -> {
                            // Additional event handlers
                        }
                    }
                }
                is Patch.DetachEventListener -> {
                    val view = nodes.get(patch.nodeId) ?: continue
                    when (patch.eventType) {
                        EventType.CLICK -> view.setOnClickListener(null)
                        else -> {}
                    }
                }
            }
        }
    }
}
