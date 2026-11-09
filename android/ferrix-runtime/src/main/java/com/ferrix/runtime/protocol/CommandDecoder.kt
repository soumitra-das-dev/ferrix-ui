package com.ferrix.runtime.protocol

import java.nio.ByteBuffer
import java.nio.ByteOrder
import java.nio.charset.StandardCharsets

/**
 * Decodes packed binary UI patch buffers into typed [Patch] commands.
 */
object CommandDecoder {
    private const val MAGIC_FXUI = 0x49555846 // "FXUI" in Little-Endian
    private const val PROTOCOL_VERSION: Short = 1

    const val OP_CREATE_NODE: Byte = 1
    const val OP_APPEND_CHILD: Byte = 2
    const val OP_INSERT_BEFORE: Byte = 3
    const val OP_REMOVE_CHILD: Byte = 4
    const val OP_DESTROY_NODE: Byte = 5
    const val OP_SET_TEXT: Byte = 6
    const val OP_SET_LAYOUT: Byte = 7
    const val OP_SET_PROPERTY: Byte = 8
    const val OP_SET_VISIBILITY: Byte = 9
    const val OP_ATTACH_EVENT_LISTENER: Byte = 10
    const val OP_DETACH_EVENT_LISTENER: Byte = 11

    fun decode(buffer: ByteBuffer): List<Patch> {
        val buf = buffer.duplicate().order(ByteOrder.LITTLE_ENDIAN)
        if (buf.remaining() < 16) {
            throw IllegalArgumentException("Buffer too small for Ferrix header: ${buf.remaining()} bytes")
        }

        val magic = buf.int
        if (magic != MAGIC_FXUI) {
            throw IllegalArgumentException("Invalid magic header: 0x${Integer.toHexString(magic)}")
        }

        val version = buf.short
        if (version != PROTOCOL_VERSION) {
            throw IllegalArgumentException("Unsupported protocol version: $version")
        }

        buf.short // flags (reserved for future bitflags)
        val commandCount = buf.int
        val payloadLen = buf.int

        if (buf.remaining() < payloadLen) {
            throw IllegalArgumentException("Buffer truncated: expected $payloadLen bytes payload, got ${buf.remaining()}")
        }

        val patches = ArrayList<Patch>(commandCount)

        for (i in 0 until commandCount) {
            val opcode = buf.get()
            when (opcode) {
                OP_CREATE_NODE -> {
                    val nodeId = buf.long
                    val viewTypeVal = buf.short
                    val viewType = ViewType.fromValue(viewTypeVal)
                        ?: throw IllegalArgumentException("Unknown view type: $viewTypeVal")
                    patches.add(Patch.CreateNode(nodeId, viewType))
                }
                OP_APPEND_CHILD -> {
                    val parentId = buf.long
                    val childId = buf.long
                    patches.add(Patch.AppendChild(parentId, childId))
                }
                OP_INSERT_BEFORE -> {
                    val parentId = buf.long
                    val childId = buf.long
                    val beforeId = buf.long
                    patches.add(Patch.InsertBefore(parentId, childId, beforeId))
                }
                OP_REMOVE_CHILD -> {
                    val parentId = buf.long
                    val childId = buf.long
                    patches.add(Patch.RemoveChild(parentId, childId))
                }
                OP_DESTROY_NODE -> {
                    val nodeId = buf.long
                    patches.add(Patch.DestroyNode(nodeId))
                }
                OP_SET_TEXT -> {
                    val nodeId = buf.long
                    val textLen = buf.int
                    val textBytes = ByteArray(textLen)
                    buf.get(textBytes)
                    val text = String(textBytes, StandardCharsets.UTF_8)
                    patches.add(Patch.SetText(nodeId, text))
                }
                OP_SET_LAYOUT -> {
                    val nodeId = buf.long
                    val x = buf.float
                    val y = buf.float
                    val width = buf.float
                    val height = buf.float
                    patches.add(Patch.SetLayout(nodeId, x, y, width, height))
                }
                OP_SET_PROPERTY -> {
                    val nodeId = buf.long
                    val propId = buf.short
                    val valLen = buf.short.toInt() and 0xFFFF
                    val property = when (propId.toInt()) {
                        1 -> Property.BackgroundColor(buf.int)
                        2 -> Property.TextColor(buf.int)
                        3 -> Property.FontSize(buf.float)
                        4 -> {
                            val left = buf.float
                            val top = buf.float
                            val right = buf.float
                            val bottom = buf.float
                            Property.Padding(left, top, right, bottom)
                        }
                        5 -> Property.Spacing(buf.float)
                        6 -> Property.CornerRadius(buf.float)
                        7 -> Property.Enabled(buf.get() != 0.toByte())
                        8 -> {
                            val urlBytes = ByteArray(valLen)
                            buf.get(urlBytes)
                            Property.ImageUrl(String(urlBytes, StandardCharsets.UTF_8))
                        }
                        9 -> {
                            val target = buf.float
                            val durationMs = buf.int.toLong() and 0xFFFFFFFFL
                            Property.AnimateAlpha(target, durationMs)
                        }
                        10 -> {
                            val target = buf.float
                            val durationMs = buf.int.toLong() and 0xFFFFFFFFL
                            Property.AnimateScale(target, durationMs)
                        }
                        11 -> {
                            val target = buf.float
                            val durationMs = buf.int.toLong() and 0xFFFFFFFFL
                            Property.AnimateTranslationY(target, durationMs)
                        }
                        else -> {
                            // Skip unknown property bytes
                            buf.position(buf.position() + valLen)
                            null
                        }
                    }
                    if (property != null) {
                        patches.add(Patch.SetProperty(nodeId, property))
                    }
                }
                OP_SET_VISIBILITY -> {
                    val nodeId = buf.long
                    val visVal = buf.get()
                    val visibility = Visibility.fromValue(visVal)
                        ?: throw IllegalArgumentException("Unknown visibility: $visVal")
                    patches.add(Patch.SetVisibility(nodeId, visibility))
                }
                OP_ATTACH_EVENT_LISTENER -> {
                    val nodeId = buf.long
                    val eventTypeVal = buf.short
                    val eventType = EventType.fromValue(eventTypeVal)
                        ?: throw IllegalArgumentException("Unknown event type: $eventTypeVal")
                    patches.add(Patch.AttachEventListener(nodeId, eventType))
                }
                OP_DETACH_EVENT_LISTENER -> {
                    val nodeId = buf.long
                    val eventTypeVal = buf.short
                    val eventType = EventType.fromValue(eventTypeVal)
                        ?: throw IllegalArgumentException("Unknown event type: $eventTypeVal")
                    patches.add(Patch.DetachEventListener(nodeId, eventType))
                }
                else -> throw IllegalArgumentException("Unknown opcode: $opcode")
            }
        }

        return patches
    }
}
