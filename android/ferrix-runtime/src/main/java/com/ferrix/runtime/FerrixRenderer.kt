package com.ferrix.runtime

import android.content.Context
import android.os.Looper
import android.util.TypedValue
import android.view.View
import android.view.ViewGroup
import android.widget.Button
import android.widget.LinearLayout
import android.widget.TextView
import androidx.collection.LongSparseArray
import com.ferrix.runtime.protocol.EventType
import com.ferrix.runtime.protocol.Patch
import com.ferrix.runtime.protocol.Property
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
                    if (patch.nodeId == 1L && rootContainer is LinearLayout) {
                        // Root already provided
                    } else {
                        val view = viewFactory.createView(patch.viewType)
                        nodes.put(patch.nodeId, view)
                    }
                }
                is Patch.AppendChild -> {
                    val parent = nodes.get(patch.parentId) as? ViewGroup
                    val child = nodes.get(patch.childId)
                    if (parent != null && child != null) {
                        if (child.parent == null) {
                            parent.addView(child)
                        }
                    }
                }
                is Patch.InsertBefore -> {
                    val parent = nodes.get(patch.parentId) as? ViewGroup
                    val child = nodes.get(patch.childId)
                    val before = nodes.get(patch.beforeId)
                    if (parent != null && child != null) {
                        val index = if (before != null) parent.indexOfChild(before) else -1
                        if (child.parent != null) {
                            (child.parent as? ViewGroup)?.removeView(child)
                        }
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
                        view.text = patch.text
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
                                view.dividerPadding = (prop.dp * density).toInt()
                            }
                        }
                        is Property.CornerRadius -> {
                            // Can be mapped to outlineProvider or background drawable
                        }
                        is Property.Enabled -> view.isEnabled = prop.enabled
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
                        else -> {
                            // Additional events handled as needed
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
