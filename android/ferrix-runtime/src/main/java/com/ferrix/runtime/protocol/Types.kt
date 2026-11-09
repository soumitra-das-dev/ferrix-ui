package com.ferrix.runtime.protocol

/**
 * Native view types supported by the Ferrix Android renderer.
 */
enum class ViewType(val value: Short) {
    CONTAINER(1),
    COLUMN(2),
    ROW(3),
    TEXT(4),
    BUTTON(5),
    IMAGE(6),
    TEXT_INPUT(7),
    SCROLL_VIEW(8),
    PRESSABLE(9),
    FRAGMENT(10),
    SWITCH(11),
    ACTIVITY_INDICATOR(12),
    MODAL(13);

    companion object {
        fun fromValue(value: Short): ViewType? = entries.firstOrNull { it.value == value }
    }
}

/**
 * Android View visibility states.
 */
enum class Visibility(val value: Byte) {
    VISIBLE(0),
    INVISIBLE(1),
    GONE(2);

    companion object {
        fun fromValue(value: Byte): Visibility? = entries.firstOrNull { it.value == value }
    }
}

/**
 * Event types dispatched from Kotlin to Rust.
 */
enum class EventType(val value: Short) {
    CLICK(1),
    TEXT_CHANGED(2),
    FOCUS_CHANGED(3),
    SCROLL(4),
    PRESS_IN(5),
    PRESS_OUT(6),
    LONG_PRESS(7),
    LAYOUT(8);

    companion object {
        fun fromValue(value: Short): EventType? = entries.firstOrNull { it.value == value }
    }
}

/**
 * Typed properties configuring view appearance, typography, accessibility and styling.
 */
sealed class Property {
    data class BackgroundColor(val color: Int) : Property()
    data class TextColor(val color: Int) : Property()
    data class FontSize(val sp: Float) : Property()
    data class Padding(
        val left: Float,
        val top: Float,
        val right: Float,
        val bottom: Float
    ) : Property()
    data class Spacing(val dp: Float) : Property()
    data class CornerRadius(val dp: Float) : Property()
    data class Enabled(val enabled: Boolean) : Property()
    data class ImageUrl(val url: String) : Property()
    data class AnimateAlpha(val target: Float, val durationMs: Long) : Property()
    data class AnimateScale(val target: Float, val durationMs: Long) : Property()
    data class AnimateTranslationY(val target: Float, val durationMs: Long) : Property()
    data class ContentDescription(val description: String) : Property()
    data class AccessibilityRole(val role: String) : Property()
    data class AccessibilityHeading(val isHeading: Boolean) : Property()
    data class Hint(val hint: String) : Property()

    // Wave 0 Extensions: Visual, Typography, Accessibility, Touch
    data class Opacity(val opacity: Float) : Property()
    data class BorderWidth(
        val left: Float,
        val top: Float,
        val right: Float,
        val bottom: Float
    ) : Property()
    data class BorderColor(val color: Int) : Property()
    data class BorderRadius(
        val topLeft: Float,
        val topRight: Float,
        val bottomRight: Float,
        val bottomLeft: Float
    ) : Property()
    data class Elevation(val elevation: Float) : Property()
    data class Transform(
        val tx: Float,
        val ty: Float,
        val sx: Float,
        val sy: Float,
        val rotateDeg: Float
    ) : Property()
    data class FontWeight(val weight: Int) : Property()
    data class FontStyle(val style: Int) : Property()
    data class LineHeight(val lineHeight: Float) : Property()
    data class LetterSpacing(val letterSpacing: Float) : Property()
    data class TextAlign(val align: Int) : Property()
    data class TextDecoration(val decoration: Int) : Property()
    data class NumberOfLines(val lines: Int) : Property()
    data class TestId(val id: String) : Property()
    data class AccessibilityLabel(val label: String) : Property()
    data class AccessibilityHint(val hint: String) : Property()
    data class AccessibilityState(
        val disabled: Boolean,
        val selected: Boolean,
        val checked: Int,
        val busy: Boolean,
        val expanded: Int
    ) : Property()
    data class ImportantForAccessibility(val mode: Int) : Property()
    data class RippleColor(val color: Int) : Property()
    data class HitSlop(
        val left: Float,
        val top: Float,
        val right: Float,
        val bottom: Float
    ) : Property()
}

/**
 * Imperative command dispatched from Rust runtime to native views.
 */
sealed class Command {
    data class FocusNode(val nodeId: Long) : Command()
    data class BlurNode(val nodeId: Long) : Command()
    data class ScrollTo(val nodeId: Long, val x: Float, val y: Float, val animated: Boolean) : Command()
    data class MeasureNode(val nodeId: Long) : Command()
}

/**
 * Text measurement request payload.
 */
data class TextMeasureRequest(
    val requestId: Int,
    val text: String,
    val fontSizeSp: Float,
    val fontWeight: Int,
    val fontStyle: Int,
    val maxWidth: Float
)

/**
 * Text measurement response payload.
 */
data class TextMeasureResponse(
    val requestId: Int,
    val width: Float,
    val height: Float
)
