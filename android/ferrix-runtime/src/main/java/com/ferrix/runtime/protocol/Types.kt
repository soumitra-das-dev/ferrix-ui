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
    SCROLL_VIEW(8);

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
    SCROLL(4);

    companion object {
        fun fromValue(value: Short): EventType? = entries.firstOrNull { it.value == value }
    }
}

/**
 * Typed properties configuring view appearance and styling.
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
}
