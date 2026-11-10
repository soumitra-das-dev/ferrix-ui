package com.ferrix.runtime.protocol

/**
 * Atomic UI patches emitted by the Rust runtime.
 */
sealed class Patch {
    data class CreateNode(val nodeId: Long, val viewType: ViewType) : Patch()
    data class AppendChild(val parentId: Long, val childId: Long) : Patch()
    data class InsertBefore(val parentId: Long, val childId: Long, val beforeId: Long) : Patch()
    data class RemoveChild(val parentId: Long, val childId: Long) : Patch()
    data class DestroyNode(val nodeId: Long) : Patch()
    data class SetText(val nodeId: Long, val text: String) : Patch()
    data class SetLayout(
        val nodeId: Long,
        val x: Float,
        val y: Float,
        val width: Float,
        val height: Float
    ) : Patch()
    data class SetProperty(val nodeId: Long, val property: Property) : Patch()
    data class SetVisibility(val nodeId: Long, val visibility: Visibility) : Patch()
    data class AttachEventListener(val nodeId: Long, val eventType: EventType) : Patch()
    data class DetachEventListener(val nodeId: Long, val eventType: EventType) : Patch()
    data class RegisterStyles(val styles: List<Pair<Int, List<Property>>>) : Patch()
    data class SetStyle(val nodeId: Long, val styleId: Int) : Patch()
    data class SetStyleList(val nodeId: Long, val styleIds: List<Int>) : Patch()
}
