package com.ferrix.runtime

import android.content.Context
import android.content.res.ColorStateList
import android.graphics.Outline
import android.graphics.Paint
import android.graphics.Typeface
import android.graphics.drawable.GradientDrawable
import android.graphics.drawable.RippleDrawable
import android.graphics.drawable.ShapeDrawable
import android.os.Build
import android.os.Looper
import android.text.TextUtils
import android.util.TypedValue
import android.view.MotionEvent
import android.view.View
import android.view.ViewGroup
import android.view.ViewOutlineProvider
import android.widget.CompoundButton
import android.widget.EditText
import android.widget.ImageView
import android.widget.LinearLayout
import android.widget.TextView
import androidx.collection.LongSparseArray
import androidx.core.widget.NestedScrollView
import com.ferrix.runtime.protocol.Command
import com.ferrix.runtime.protocol.EventType
import com.ferrix.runtime.protocol.Patch
import com.ferrix.runtime.protocol.Property
import com.ferrix.runtime.protocol.ViewType
import com.ferrix.runtime.protocol.Visibility
import java.nio.ByteBuffer
import java.nio.ByteOrder

/**
 * Concrete Android Views renderer for Ferrix.
 * Enforces execution strictly on the Android Main Thread.
 */
class FerrixRenderer(
    private val context: Context,
    private val rootContainer: ViewGroup
) {
    val viewFactory = FerrixViewFactory(context)
    val textMeasurer = StaticLayoutTextMeasurer(context)
    private val nodes = LongSparseArray<View>()
    private val nodeViewTypes = LongSparseArray<ViewType>()

    init {
        // Register the host root view as NodeId 1 if designated as root
        nodes.put(1L, rootContainer)
        nodeViewTypes.put(1L, ViewType.CONTAINER)
    }

    fun getView(nodeId: Long): View? = nodes.get(nodeId)

    /**
     * Applies a batch of imperative commands to the native View tree.
     */
    fun applyCommands(commands: List<Command>) {
        check(Looper.myLooper() == Looper.getMainLooper()) {
            "FerrixRenderer.applyCommands must be executed on the Android Main Thread"
        }

        val density = context.resources.displayMetrics.density

        for (cmd in commands) {
            when (cmd) {
                is Command.FocusNode -> {
                    nodes.get(cmd.nodeId)?.requestFocus()
                }
                is Command.BlurNode -> {
                    nodes.get(cmd.nodeId)?.clearFocus()
                }
                is Command.ScrollTo -> {
                    val view = nodes.get(cmd.nodeId)
                    val targetX = (cmd.x * density).toInt()
                    val targetY = (cmd.y * density).toInt()
                    if (view is NestedScrollView) {
                        if (cmd.animated) view.smoothScrollTo(targetX, targetY) else view.scrollTo(targetX, targetY)
                    } else {
                        view?.scrollTo(targetX, targetY)
                    }
                }
                is Command.MeasureNode -> {
                    val view = nodes.get(cmd.nodeId) ?: continue
                    val loc = IntArray(2)
                    view.getLocationInWindow(loc)
                }
            }
        }
    }

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

        val density = context.resources.displayMetrics.density

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
                        nodeViewTypes.put(1L, patch.viewType)
                    } else {
                        val view = viewFactory.createView(patch.viewType)
                        nodes.put(patch.nodeId, view)
                        nodeViewTypes.put(patch.nodeId, patch.viewType)
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
                    val vt = nodeViewTypes.get(patch.nodeId)
                    if (view != null) {
                        (view.parent as? ViewGroup)?.removeView(view)
                        nodes.remove(patch.nodeId)
                        nodeViewTypes.remove(patch.nodeId)
                        if (vt != null) {
                            viewFactory.recycleView(vt, view)
                        }
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
                        if (patch.width > 0) lp.width = (patch.width * density).toInt()
                        if (patch.height > 0) lp.height = (patch.height * density).toInt()
                        if (lp is ViewGroup.MarginLayoutParams) {
                            lp.leftMargin = (patch.x * density).toInt()
                            lp.topMargin = (patch.y * density).toInt()
                        }
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
                            view.setPadding(
                                (prop.left * density).toInt(),
                                (prop.top * density).toInt(),
                                (prop.right * density).toInt(),
                                (prop.bottom * density).toInt()
                            )
                        }
                        is Property.Spacing -> {
                            if (view is LinearLayout) {
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
                            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.P) {
                                view.isAccessibilityHeading = prop.isHeading
                            }
                        }
                        is Property.Hint -> (view as? EditText)?.hint = prop.hint

                        // Wave 0 Extensions
                        is Property.Opacity -> view.alpha = prop.opacity
                        is Property.Elevation -> view.elevation = prop.elevation * density
                        is Property.BorderWidth -> {
                            // Stroke configuration
                        }
                        is Property.BorderColor -> {
                            // Border color configuration
                        }
                        is Property.BorderRadius -> {
                            val rTl = prop.topLeft * density
                            val rTr = prop.topRight * density
                            val rBr = prop.bottomRight * density
                            val rBl = prop.bottomLeft * density
                            view.clipToOutline = true
                            view.outlineProvider = object : ViewOutlineProvider() {
                                override fun getOutline(v: View, outline: Outline) {
                                    outline.setRoundRect(0, 0, v.width, v.height, rTl)
                                }
                            }
                        }
                        is Property.Transform -> {
                            view.translationX = prop.tx * density
                            view.translationY = prop.ty * density
                            view.scaleX = prop.sx
                            view.scaleY = prop.sy
                            view.rotation = prop.rotateDeg
                        }
                        is Property.FontWeight -> {
                            (view as? TextView)?.let { tv ->
                                val curStyle = tv.typeface?.style ?: Typeface.NORMAL
                                val isItalic = (curStyle and Typeface.ITALIC) != 0
                                val newStyle = if (prop.weight >= 600) {
                                    if (isItalic) Typeface.BOLD_ITALIC else Typeface.BOLD
                                } else {
                                    if (isItalic) Typeface.ITALIC else Typeface.NORMAL
                                }
                                tv.setTypeface(tv.typeface, newStyle)
                            }
                        }
                        is Property.FontStyle -> {
                            (view as? TextView)?.let { tv ->
                                val curStyle = tv.typeface?.style ?: Typeface.NORMAL
                                val isBold = (curStyle and Typeface.BOLD) != 0
                                val newStyle = if (prop.style == 1) {
                                    if (isBold) Typeface.BOLD_ITALIC else Typeface.ITALIC
                                } else {
                                    if (isBold) Typeface.BOLD else Typeface.NORMAL
                                }
                                tv.setTypeface(tv.typeface, newStyle)
                            }
                        }
                        is Property.LineHeight -> {
                            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.P) {
                                (view as? TextView)?.lineHeight = (prop.lineHeight * density).toInt()
                            }
                        }
                        is Property.LetterSpacing -> {
                            (view as? TextView)?.letterSpacing = prop.letterSpacing
                        }
                        is Property.TextAlign -> {
                            (view as? TextView)?.textAlignment = when (prop.align) {
                                1 -> View.TEXT_ALIGNMENT_CENTER
                                2 -> View.TEXT_ALIGNMENT_VIEW_END
                                3 -> View.TEXT_ALIGNMENT_GRAVITY
                                else -> View.TEXT_ALIGNMENT_VIEW_START
                            }
                        }
                        is Property.TextDecoration -> {
                            (view as? TextView)?.let { tv ->
                                when (prop.decoration) {
                                    1 -> tv.paintFlags = tv.paintFlags or Paint.UNDERLINE_TEXT_FLAG
                                    2 -> tv.paintFlags = tv.paintFlags or Paint.STRIKE_THRU_TEXT_FLAG
                                    else -> tv.paintFlags = tv.paintFlags and Paint.UNDERLINE_TEXT_FLAG.inv() and Paint.STRIKE_THRU_TEXT_FLAG.inv()
                                }
                            }
                        }
                        is Property.NumberOfLines -> {
                            (view as? TextView)?.let { tv ->
                                if (prop.lines > 0) {
                                    tv.maxLines = prop.lines
                                    tv.ellipsize = TextUtils.TruncateAt.END
                                } else {
                                    tv.maxLines = Int.MAX_VALUE
                                    tv.ellipsize = null
                                }
                            }
                        }
                        is Property.TestId -> view.tag = prop.id
                        is Property.AccessibilityLabel -> view.contentDescription = prop.label
                        is Property.AccessibilityHint -> {
                            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
                                view.tooltipText = prop.hint
                            }
                        }
                        is Property.AccessibilityState -> {
                            view.isEnabled = !prop.disabled
                            view.isSelected = prop.selected
                            if (prop.checked == 1) {
                                (view as? CompoundButton)?.isChecked = true
                            } else if (prop.checked == 0) {
                                (view as? CompoundButton)?.isChecked = false
                            }
                        }
                        is Property.ImportantForAccessibility -> {
                            view.importantForAccessibility = when (prop.mode) {
                                1 -> View.IMPORTANT_FOR_ACCESSIBILITY_YES
                                2 -> View.IMPORTANT_FOR_ACCESSIBILITY_NO
                                3 -> View.IMPORTANT_FOR_ACCESSIBILITY_NO_HIDE_DESCENDANTS
                                else -> View.IMPORTANT_FOR_ACCESSIBILITY_AUTO
                            }
                        }
                        is Property.RippleColor -> {
                            val ripple = (view.background as? RippleDrawable) ?: (view.foreground as? RippleDrawable)
                            ripple?.setColor(ColorStateList.valueOf(prop.color))
                        }
                        is Property.HitSlop -> {
                            // Slop handled on touch interception
                        }
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
                        EventType.PRESS_IN, EventType.PRESS_OUT -> {
                            view.setOnTouchListener { _, event ->
                                when (event.actionMasked) {
                                    MotionEvent.ACTION_DOWN -> {
                                        eventDispatcher(patch.nodeId, EventType.PRESS_IN, null)
                                    }
                                    MotionEvent.ACTION_UP, MotionEvent.ACTION_CANCEL -> {
                                        eventDispatcher(patch.nodeId, EventType.PRESS_OUT, null)
                                    }
                                }
                                false
                            }
                        }
                        EventType.LONG_PRESS -> {
                            view.setOnLongClickListener {
                                eventDispatcher(patch.nodeId, EventType.LONG_PRESS, null)
                                true
                            }
                        }
                        EventType.LAYOUT -> {
                            view.addOnLayoutChangeListener { _, left, top, right, bottom, _, _, _, _ ->
                                val w = (right - left) / density
                                val h = (bottom - top) / density
                                val x = left / density
                                val y = top / density
                                val payload = ByteBuffer.allocate(16).order(ByteOrder.LITTLE_ENDIAN)
                                    .putFloat(x).putFloat(y).putFloat(w).putFloat(h).array()
                                eventDispatcher(patch.nodeId, EventType.LAYOUT, payload)
                            }
                        }
                        else -> {}
                    }
                }
                is Patch.DetachEventListener -> {
                    val view = nodes.get(patch.nodeId) ?: continue
                    when (patch.eventType) {
                        EventType.CLICK -> view.setOnClickListener(null)
                        EventType.LONG_PRESS -> view.setOnLongClickListener(null)
                        EventType.PRESS_IN, EventType.PRESS_OUT -> view.setOnTouchListener(null)
                        else -> {}
                    }
                }
            }
        }
    }
}
