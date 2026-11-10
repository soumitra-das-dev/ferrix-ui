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
import android.text.InputFilter
import android.text.InputType
import android.text.TextUtils
import android.text.method.PasswordTransformationMethod
import android.view.inputmethod.EditorInfo
import android.util.TypedValue
import android.view.MotionEvent
import android.view.View
import android.view.ViewGroup
import android.view.ViewOutlineProvider
import android.widget.CompoundButton
import android.widget.EditText
import android.widget.GridLayout
import android.widget.ImageView
import android.widget.LinearLayout
import android.widget.ProgressBar
import android.widget.SeekBar
import android.widget.TextView
import androidx.appcompat.widget.SwitchCompat
import androidx.collection.LongSparseArray
import androidx.core.widget.NestedScrollView
import androidx.recyclerview.widget.LinearLayoutManager
import androidx.recyclerview.widget.PagerSnapHelper
import androidx.recyclerview.widget.RecyclerView
import androidx.swiperefreshlayout.widget.SwipeRefreshLayout
import com.ferrix.runtime.protocol.Command
import com.ferrix.runtime.protocol.EventType
import com.ferrix.runtime.protocol.Patch
import com.ferrix.runtime.protocol.Property
import com.ferrix.runtime.protocol.ViewType
import com.ferrix.runtime.protocol.Visibility
import java.nio.ByteBuffer
import java.nio.ByteOrder
import java.nio.charset.StandardCharsets

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
    private val nodeTextVersions = HashMap<Long, Long>()
    val styleRegistry = HashMap<Int, List<Property>>()

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
                is Command.Navigate -> {
                    val container = nodes.get(cmd.containerId) as? com.ferrix.runtime.navigation.FerrixNavContainer
                    val screen = nodes.get(cmd.screenId) as? com.ferrix.runtime.navigation.FerrixScreenView
                    if (container != null && screen != null) {
                        container.pushScreen(screen, cmd.animation)
                    }
                }
                is Command.GoBack -> {
                    val container = nodes.get(cmd.containerId) as? com.ferrix.runtime.navigation.FerrixNavContainer
                    container?.popScreen(cmd.animation)
                }
                is Command.SetScreenOptions -> {
                    val screen = nodes.get(cmd.screenId) as? com.ferrix.runtime.navigation.FerrixScreenView
                    if (screen != null) {
                        screen.nodeId = cmd.screenId
                        screen.screenTitle = cmd.title
                        screen.headerShown = cmd.headerShown
                        screen.presentation = cmd.presentation
                        var parentView = screen.parent
                        while (parentView != null) {
                            if (parentView is com.ferrix.runtime.navigation.FerrixNavContainer) {
                                parentView.setScreenOptions(cmd.screenId, cmd.title, cmd.headerShown, cmd.presentation)
                                break
                            }
                            parentView = parentView.parent
                        }
                    }
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
                        nodeTextVersions.remove(patch.nodeId)
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
                    applyPropertyToView(patch.nodeId, view, patch.property, density)
                }
                is Patch.RegisterStyles -> {
                    for ((styleId, props) in patch.styles) {
                        styleRegistry[styleId] = props
                    }
                }
                is Patch.SetStyle -> {
                    val view = nodes.get(patch.nodeId) ?: continue
                    val props = styleRegistry[patch.styleId] ?: continue
                    for (prop in props) {
                        applyPropertyToView(patch.nodeId, view, prop, density)
                    }
                }
                is Patch.SetStyleList -> {
                    val view = nodes.get(patch.nodeId) ?: continue
                    for (styleId in patch.styleIds) {
                        val props = styleRegistry[styleId] ?: continue
                        for (prop in props) {
                            applyPropertyToView(patch.nodeId, view, prop, density)
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
                        EventType.END_REACHED -> {
                            (view as? RecyclerView)?.addOnScrollListener(object : RecyclerView.OnScrollListener() {
                                override fun onScrolled(recyclerView: RecyclerView, dx: Int, dy: Int) {
                                    val lm = recyclerView.layoutManager as? LinearLayoutManager ?: return
                                    val lastVisible = lm.findLastVisibleItemPosition()
                                    val total = lm.itemCount
                                    if (total > 0 && lastVisible >= total - 2) {
                                        eventDispatcher(patch.nodeId, EventType.END_REACHED, null)
                                    }
                                }
                            })
                        }
                        EventType.VALUE_CHANGE -> {
                            (view as? CompoundButton)?.setOnCheckedChangeListener { _, isChecked ->
                                val payload = byteArrayOf(if (isChecked) 1 else 0)
                                eventDispatcher(patch.nodeId, EventType.VALUE_CHANGE, payload)
                            }
                        }
                        EventType.SUBMIT_EDITING -> {
                            (view as? EditText)?.setOnEditorActionListener { _, _, _ ->
                                val textBytes = (view.text?.toString() ?: "").toByteArray(StandardCharsets.UTF_8)
                                eventDispatcher(patch.nodeId, EventType.SUBMIT_EDITING, textBytes)
                                false
                            }
                        }
                        EventType.REFRESH -> {
                            (view as? SwipeRefreshLayout)?.setOnRefreshListener {
                                eventDispatcher(patch.nodeId, EventType.REFRESH, null)
                            }
                        }
                        EventType.SLIDER_CHANGE -> {
                            (view as? SeekBar)?.setOnSeekBarChangeListener(object : SeekBar.OnSeekBarChangeListener {
                                override fun onProgressChanged(seekBar: SeekBar?, progress: Int, fromUser: Boolean) {
                                    if (fromUser) {
                                        val normVal = progress / 1000f
                                        val payload = ByteBuffer.allocate(4).order(ByteOrder.LITTLE_ENDIAN)
                                            .putFloat(normVal).array()
                                        eventDispatcher(patch.nodeId, EventType.SLIDER_CHANGE, payload)
                                    }
                                }
                                override fun onStartTrackingTouch(seekBar: SeekBar?) {}
                                override fun onStopTrackingTouch(seekBar: SeekBar?) {}
                            })
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

    fun applyPropertyToView(nodeId: Long, view: View, prop: Property, density: Float) {
        when (prop) {
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

                        // Wave 1 Extensions: Controlled Text & Inputs
                        is Property.ControlledText -> {
                            if (view is EditText) {
                                val lastVer = if (nodeId != 0L) nodeTextVersions[nodeId] ?: -1L else -1L
                                if (prop.version >= lastVer) {
                                    if (nodeId != 0L) nodeTextVersions[nodeId] = prop.version
                                    if (view.text.toString() != prop.text) {
                                        val prevTag = view.tag
                                        view.tag = "PROGRAMMATIC_UPDATE"
                                        val selStart = view.selectionStart
                                        val selEnd = view.selectionEnd
                                        view.setText(prop.text)
                                        val newLen = prop.text.length
                                        val newStart = selStart.coerceIn(0, newLen)
                                        val newEnd = selEnd.coerceIn(0, newLen)
                                        view.setSelection(newStart, newEnd)
                                        view.tag = prevTag
                                    }
                                }
                            }
                        }
                        is Property.Placeholder -> (view as? EditText)?.hint = prop.placeholder
                        is Property.PlaceholderTextColor -> (view as? EditText)?.setHintTextColor(prop.color)
                        is Property.SecureTextEntry -> {
                            (view as? EditText)?.let { et ->
                                et.transformationMethod = if (prop.secure) {
                                    PasswordTransformationMethod.getInstance()
                                } else {
                                    null
                                }
                            }
                        }
                        is Property.KeyboardType -> {
                            (view as? EditText)?.let { et ->
                                et.inputType = when (prop.keyboardType) {
                                    1 -> InputType.TYPE_CLASS_NUMBER
                                    2 -> InputType.TYPE_CLASS_TEXT or InputType.TYPE_TEXT_VARIATION_EMAIL_ADDRESS
                                    3 -> InputType.TYPE_CLASS_PHONE
                                    4 -> InputType.TYPE_CLASS_TEXT or InputType.TYPE_TEXT_VARIATION_URI
                                    else -> InputType.TYPE_CLASS_TEXT
                                }
                            }
                        }
                        is Property.ReturnKeyType -> {
                            (view as? EditText)?.let { et ->
                                et.imeOptions = when (prop.returnKeyType) {
                                    1 -> EditorInfo.IME_ACTION_GO
                                    2 -> EditorInfo.IME_ACTION_SEARCH
                                    3 -> EditorInfo.IME_ACTION_SEND
                                    4 -> EditorInfo.IME_ACTION_NEXT
                                    else -> EditorInfo.IME_ACTION_DONE
                                }
                            }
                        }
                        is Property.MaxLength -> {
                            (view as? EditText)?.let { et ->
                                if (prop.maxLength > 0) {
                                    et.filters = arrayOf(InputFilter.LengthFilter(prop.maxLength))
                                } else {
                                    et.filters = arrayOf()
                                }
                            }
                        }
                        is Property.Multiline -> {
                            (view as? EditText)?.let { et ->
                                et.isSingleLine = !prop.multiline
                                if (prop.multiline) {
                                    et.inputType = et.inputType or InputType.TYPE_TEXT_FLAG_MULTI_LINE
                                }
                            }
                        }
                        is Property.AutoCapitalize -> {
                            (view as? EditText)?.let { et ->
                                val flag = when (prop.mode) {
                                    1 -> InputType.TYPE_TEXT_FLAG_CAP_CHARACTERS
                                    2 -> InputType.TYPE_TEXT_FLAG_CAP_WORDS
                                    3 -> InputType.TYPE_TEXT_FLAG_CAP_SENTENCES
                                    else -> 0
                                }
                                et.inputType = (et.inputType and InputType.TYPE_TEXT_FLAG_CAP_CHARACTERS.inv()
                                    and InputType.TYPE_TEXT_FLAG_CAP_WORDS.inv()
                                    and InputType.TYPE_TEXT_FLAG_CAP_SENTENCES.inv()) or flag
                            }
                        }
                        is Property.AutoCorrect -> {
                            (view as? EditText)?.let { et ->
                                if (prop.enabled) {
                                    et.inputType = et.inputType or InputType.TYPE_TEXT_FLAG_AUTO_CORRECT
                                } else {
                                    et.inputType = et.inputType and InputType.TYPE_TEXT_FLAG_AUTO_CORRECT.inv()
                                }
                            }
                        }
                        is Property.Editable -> {
                            (view as? EditText)?.let { et ->
                                et.isEnabled = prop.editable
                                et.isFocusable = prop.editable
                                et.isFocusableInTouchMode = prop.editable
                            }
                        }
                        is Property.Selection -> {
                            (view as? EditText)?.let { et ->
                                val len = et.text?.length ?: 0
                                val s = prop.start.coerceIn(0, len)
                                val e = prop.end.coerceIn(0, len)
                                et.setSelection(s, e)
                            }
                        }
                        is Property.ImageResizeMode -> {
                            (view as? ImageView)?.let { iv ->
                                iv.scaleType = when (prop.mode) {
                                    1 -> ImageView.ScaleType.CENTER_CROP
                                    2 -> ImageView.ScaleType.FIT_XY
                                    3 -> ImageView.ScaleType.CENTER_INSIDE
                                    else -> ImageView.ScaleType.FIT_CENTER
                                }
                            }
                        }
                        is Property.BlurRadius -> view.tag = "blur:${prop.radius}"
                        is Property.TintColor -> (view as? ImageView)?.setColorFilter(prop.color)
                        is Property.ScrollHorizontal -> {
                            (view as? RecyclerView)?.let { rv ->
                                (rv.layoutManager as? LinearLayoutManager)?.orientation =
                                    if (prop.horizontal) LinearLayoutManager.HORIZONTAL else LinearLayoutManager.VERTICAL
                            }
                        }
                        is Property.PagingEnabled -> {
                            (view as? RecyclerView)?.let { rv ->
                                if (prop.enabled && rv.onFlingListener == null) {
                                    PagerSnapHelper().attachToRecyclerView(rv)
                                }
                            }
                        }
                        is Property.ShowsScrollIndicator -> {
                            view.isVerticalScrollBarEnabled = prop.shows
                            view.isHorizontalScrollBarEnabled = prop.shows
                        }
                        is Property.ContentOffset -> {
                            view.scrollTo((prop.x * density).toInt(), (prop.y * density).toInt())
                        }
                        is Property.ItemCount -> {
                            (view as? RecyclerView)?.let { rv ->
                                val adapter = (rv.adapter as? FerrixRecyclerAdapter)
                                    ?: FerrixRecyclerAdapter().also { rv.adapter = it }
                                adapter.setItemCount(prop.count)
                            }
                        }
                        is Property.ItemHeight -> {
                            (view as? RecyclerView)?.let { rv ->
                                val adapter = (rv.adapter as? FerrixRecyclerAdapter)
                                    ?: FerrixRecyclerAdapter().also { rv.adapter = it }
                                adapter.setItemHeight((prop.height * density).toInt())
                            }
                        }
                        is Property.SwitchValue -> (view as? CompoundButton)?.isChecked = prop.value
                        is Property.TrackColor -> {
                            (view as? SwitchCompat)?.trackTintList = ColorStateList(
                                arrayOf(intArrayOf(android.R.attr.state_checked), intArrayOf()),
                                intArrayOf(prop.active, prop.inactive)
                            )
                        }
                        is Property.ThumbColor -> {
                            (view as? SwitchCompat)?.thumbTintList = ColorStateList.valueOf(prop.color)
                        }
                        is Property.IndicatorColor -> {
                            (view as? ProgressBar)?.indeterminateTintList = ColorStateList.valueOf(prop.color)
                        }
                        is Property.IndicatorSize -> {}
                        is Property.Animating -> {
                            (view as? ProgressBar)?.visibility = if (prop.animating) View.VISIBLE else View.GONE
                        }
                        is Property.ModalVisible -> {
                            view.visibility = if (prop.visible) View.VISIBLE else View.GONE
                        }
                        is Property.ModalAnimationType -> {}
                        is Property.ModalTransparent -> {
                            if (prop.transparent) {
                                view.setBackgroundColor(0x00000000)
                            }
                        }
                        is Property.Refreshing -> {
                            (view as? SwipeRefreshLayout)?.isRefreshing = prop.refreshing
                        }
                        is Property.ProgressBackgroundColor -> {
                            (view as? SwipeRefreshLayout)?.setProgressBackgroundColorSchemeColor(prop.color)
                        }

                        // Wave 2 Extensions: Inputs, Overlays, Gradients, Media
                        is Property.Checked -> (view as? CompoundButton)?.isChecked = prop.checked
                        is Property.SliderValue -> (view as? SeekBar)?.progress = (prop.value * 1000).toInt()
                        is Property.SliderRange -> {}
                        is Property.Progress -> (view as? ProgressBar)?.progress = (prop.progress * 100).toInt()
                        is Property.GradientColors -> {
                            val gradient = GradientDrawable(
                                GradientDrawable.Orientation.LEFT_RIGHT,
                                intArrayOf(prop.startColor, prop.endColor)
                            )
                            view.background = gradient
                        }
                        is Property.IconName -> (view as? ImageView)?.tag = prop.name
                        is Property.IconSize -> {
                            (view as? ImageView)?.let { iv ->
                                val px = (prop.size * density).toInt()
                                val lp = iv.layoutParams ?: ViewGroup.LayoutParams(px, px)
                                lp.width = px
                                lp.height = px
                                iv.layoutParams = lp
                            }
                        }
                        is Property.BadgeContent -> (view as? TextView)?.text = prop.content
                        is Property.GridColumns -> (view as? GridLayout)?.columnCount = prop.columns
                    }
                
    }

}
