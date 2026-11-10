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
    MODAL(13),
    RECYCLER_VIEW(14),
    SAFE_AREA_VIEW(15),
    REFRESH_CONTROL(16),
    PORTAL(17),
    STACK(18),
    SPACER(19),
    DIVIDER(20),
    CENTER(21),
    GRID(22),
    CHECKBOX(23),
    RADIO_GROUP(24),
    SLIDER(25),
    PICKER(26),
    PROGRESS_BAR(27),
    BADGE(28),
    LINEAR_GRADIENT(29),
    ICON(30),
    NAV_CONTAINER(31),
    SCREEN(32),
    HEADER(33),
    TAB_BAR(34),
    DRAWER(35);

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
    LAYOUT(8),
    END_REACHED(9),
    VALUE_CHANGE(10),
    SUBMIT_EDITING(11),
    SELECTION_CHANGE(12),
    REQUEST_CLOSE(13),
    REFRESH(14),
    SLIDER_CHANGE(15);

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

    // Wave 1 Extensions: Controlled Text & Inputs
    data class ControlledText(val version: Long, val text: String) : Property()
    data class Placeholder(val placeholder: String) : Property()
    data class PlaceholderTextColor(val color: Int) : Property()
    data class SecureTextEntry(val secure: Boolean) : Property()
    data class KeyboardType(val keyboardType: Int) : Property()
    data class ReturnKeyType(val returnKeyType: Int) : Property()
    data class MaxLength(val maxLength: Int) : Property()
    data class Multiline(val multiline: Boolean) : Property()
    data class AutoCapitalize(val mode: Int) : Property()
    data class AutoCorrect(val enabled: Boolean) : Property()
    data class Editable(val editable: Boolean) : Property()
    data class Selection(val start: Int, val end: Int) : Property()

    // Image
    data class ImageResizeMode(val mode: Int) : Property()
    data class BlurRadius(val radius: Float) : Property()
    data class TintColor(val color: Int) : Property()

    // ScrollView & Lists
    data class ScrollHorizontal(val horizontal: Boolean) : Property()
    data class PagingEnabled(val enabled: Boolean) : Property()
    data class ShowsScrollIndicator(val shows: Boolean) : Property()
    data class ContentOffset(val x: Float, val y: Float) : Property()
    data class ItemCount(val count: Int) : Property()
    data class ItemHeight(val height: Float) : Property()

    // Switch
    data class SwitchValue(val value: Boolean) : Property()
    data class TrackColor(val active: Int, val inactive: Int) : Property()
    data class ThumbColor(val color: Int) : Property()

    // ActivityIndicator
    data class IndicatorColor(val color: Int) : Property()
    data class IndicatorSize(val size: Int) : Property()
    data class Animating(val animating: Boolean) : Property()

    // Modal
    data class ModalVisible(val visible: Boolean) : Property()
    data class ModalAnimationType(val animationType: Int) : Property()
    data class ModalTransparent(val transparent: Boolean) : Property()

    // RefreshControl
    data class Refreshing(val refreshing: Boolean) : Property()
    data class ProgressBackgroundColor(val color: Int) : Property()

    // Wave 2 Extensions: Inputs, Overlays, Gradients, Media
    data class Checked(val checked: Boolean) : Property()
    data class SliderValue(val value: Float) : Property()
    data class SliderRange(val min: Float, val max: Float, val step: Float) : Property()
    data class Progress(val progress: Float) : Property()
    data class GradientColors(val startColor: Int, val endColor: Int, val angleDeg: Float) : Property()
    data class IconName(val name: String) : Property()
    data class IconSize(val size: Float) : Property()
    data class BadgeContent(val content: String) : Property()
    data class GridColumns(val columns: Int) : Property()
}

/**
 * Imperative command dispatched from Rust runtime to native views.
 */
sealed class Command {
    data class FocusNode(val nodeId: Long) : Command()
    data class BlurNode(val nodeId: Long) : Command()
    data class ScrollTo(val nodeId: Long, val x: Float, val y: Float, val animated: Boolean) : Command()
    data class MeasureNode(val nodeId: Long) : Command()
    data class Navigate(val containerId: Long, val screenId: Long, val animation: Byte) : Command()
    data class GoBack(val containerId: Long, val animation: Byte) : Command()
    data class SetScreenOptions(val screenId: Long, val title: String, val headerShown: Boolean, val presentation: Byte) : Command()
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
