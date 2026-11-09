package com.ferrix.runtime.protocol

import java.nio.ByteBuffer
import java.nio.ByteOrder
import java.nio.charset.StandardCharsets

/**
 * Decodes packed binary UI patch buffers, imperative command buffers, and text measure buffers.
 */
object CommandDecoder {
    private const val MAGIC_FXUI = 0x49555846 // "FXUI" in Little-Endian
    private const val MAGIC_FXCM = 0x4D435846 // "FXCM" in Little-Endian
    private const val MAGIC_FXTM = 0x4D545846 // "FXTM" in Little-Endian
    private const val MAGIC_FXTR = 0x52545846 // "FXTR" in Little-Endian
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

        buf.short // flags
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
                        12 -> {
                            val strBytes = ByteArray(valLen)
                            buf.get(strBytes)
                            Property.ContentDescription(String(strBytes, StandardCharsets.UTF_8))
                        }
                        13 -> {
                            val strBytes = ByteArray(valLen)
                            buf.get(strBytes)
                            Property.AccessibilityRole(String(strBytes, StandardCharsets.UTF_8))
                        }
                        14 -> Property.AccessibilityHeading(buf.get() != 0.toByte())
                        15 -> {
                            val strBytes = ByteArray(valLen)
                            buf.get(strBytes)
                            Property.Hint(String(strBytes, StandardCharsets.UTF_8))
                        }
                        16 -> Property.Opacity(buf.float)
                        17 -> {
                            val left = buf.float
                            val top = buf.float
                            val right = buf.float
                            val bottom = buf.float
                            Property.BorderWidth(left, top, right, bottom)
                        }
                        18 -> Property.BorderColor(buf.int)
                        19 -> {
                            val tl = buf.float
                            val tr = buf.float
                            val br = buf.float
                            val bl = buf.float
                            Property.BorderRadius(tl, tr, br, bl)
                        }
                        20 -> Property.Elevation(buf.float)
                        21 -> {
                            val tx = buf.float
                            val ty = buf.float
                            val sx = buf.float
                            val sy = buf.float
                            val rot = buf.float
                            Property.Transform(tx, ty, sx, sy, rot)
                        }
                        22 -> Property.FontWeight(buf.short.toInt() and 0xFFFF)
                        23 -> Property.FontStyle(buf.get().toInt())
                        24 -> Property.LineHeight(buf.float)
                        25 -> Property.LetterSpacing(buf.float)
                        26 -> Property.TextAlign(buf.get().toInt())
                        27 -> Property.TextDecoration(buf.get().toInt())
                        28 -> Property.NumberOfLines(buf.int)
                        29 -> {
                            val strBytes = ByteArray(valLen)
                            buf.get(strBytes)
                            Property.TestId(String(strBytes, StandardCharsets.UTF_8))
                        }
                        30 -> {
                            val strBytes = ByteArray(valLen)
                            buf.get(strBytes)
                            Property.AccessibilityLabel(String(strBytes, StandardCharsets.UTF_8))
                        }
                        31 -> {
                            val strBytes = ByteArray(valLen)
                            buf.get(strBytes)
                            Property.AccessibilityHint(String(strBytes, StandardCharsets.UTF_8))
                        }
                        32 -> {
                            val dis = buf.get() != 0.toByte()
                            val sel = buf.get() != 0.toByte()
                            val chk = buf.get().toInt() and 0xFF
                            val busy = buf.get() != 0.toByte()
                            val exp = buf.get().toInt() and 0xFF
                            Property.AccessibilityState(dis, sel, chk, busy, exp)
                        }
                        33 -> Property.ImportantForAccessibility(buf.get().toInt() and 0xFF)
                        34 -> Property.RippleColor(buf.int)
                        35 -> {
                            val left = buf.float
                            val top = buf.float
                            val right = buf.float
                            val bottom = buf.float
                            Property.HitSlop(left, top, right, bottom)
                        }
                        36 -> {
                            val textVersion = buf.int.toLong() and 0xFFFFFFFFL
                            val textLen = valLen - 4
                            val strBytes = ByteArray(textLen)
                            buf.get(strBytes)
                            Property.ControlledText(textVersion, String(strBytes, StandardCharsets.UTF_8))
                        }
                        37 -> {
                            val strBytes = ByteArray(valLen)
                            buf.get(strBytes)
                            Property.Placeholder(String(strBytes, StandardCharsets.UTF_8))
                        }
                        38 -> Property.PlaceholderTextColor(buf.int)
                        39 -> Property.SecureTextEntry(buf.get() != 0.toByte())
                        40 -> Property.KeyboardType(buf.get().toInt() and 0xFF)
                        41 -> Property.ReturnKeyType(buf.get().toInt() and 0xFF)
                        42 -> Property.MaxLength(buf.int)
                        43 -> Property.Multiline(buf.get() != 0.toByte())
                        44 -> Property.AutoCapitalize(buf.get().toInt() and 0xFF)
                        45 -> Property.AutoCorrect(buf.get() != 0.toByte())
                        46 -> Property.Editable(buf.get() != 0.toByte())
                        47 -> {
                            val start = buf.int
                            val end = buf.int
                            Property.Selection(start, end)
                        }
                        48 -> Property.ImageResizeMode(buf.get().toInt() and 0xFF)
                        49 -> Property.BlurRadius(buf.float)
                        50 -> Property.TintColor(buf.int)
                        51 -> Property.ScrollHorizontal(buf.get() != 0.toByte())
                        52 -> Property.PagingEnabled(buf.get() != 0.toByte())
                        53 -> Property.ShowsScrollIndicator(buf.get() != 0.toByte())
                        54 -> {
                            val x = buf.float
                            val y = buf.float
                            Property.ContentOffset(x, y)
                        }
                        55 -> Property.ItemCount(buf.int)
                        56 -> Property.ItemHeight(buf.float)
                        57 -> Property.SwitchValue(buf.get() != 0.toByte())
                        58 -> {
                            val active = buf.int
                            val inactive = buf.int
                            Property.TrackColor(active, inactive)
                        }
                        59 -> Property.ThumbColor(buf.int)
                        60 -> Property.IndicatorColor(buf.int)
                        61 -> Property.IndicatorSize(buf.get().toInt() and 0xFF)
                        62 -> Property.Animating(buf.get() != 0.toByte())
                        63 -> Property.ModalVisible(buf.get() != 0.toByte())
                        64 -> Property.ModalAnimationType(buf.get().toInt() and 0xFF)
                        65 -> Property.ModalTransparent(buf.get() != 0.toByte())
                        66 -> Property.Refreshing(buf.get() != 0.toByte())
                        67 -> Property.ProgressBackgroundColor(buf.int)
                        else -> {
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

    /**
     * Decodes a binary command buffer into a list of [Command]s.
     */
    fun decodeCommands(buffer: ByteBuffer): List<Command> {
        val buf = buffer.duplicate().order(ByteOrder.LITTLE_ENDIAN)
        if (buf.remaining() < 8) return emptyList()

        val magic = buf.int
        if (magic != MAGIC_FXCM) {
            throw IllegalArgumentException("Invalid command magic header: 0x${Integer.toHexString(magic)}")
        }

        val count = buf.int
        val commands = ArrayList<Command>(count)

        for (i in 0 until count) {
            val op = buf.get().toInt()
            when (op) {
                1 -> commands.add(Command.FocusNode(buf.long))
                2 -> commands.add(Command.BlurNode(buf.long))
                3 -> {
                    val nodeId = buf.long
                    val x = buf.float
                    val y = buf.float
                    val animated = buf.get() != 0.toByte()
                    commands.add(Command.ScrollTo(nodeId, x, y, animated))
                }
                4 -> commands.add(Command.MeasureNode(buf.long))
            }
        }
        return commands
    }

    /**
     * Decodes text measurement requests sent from Rust.
     */
    fun decodeTextMeasureRequests(buffer: ByteBuffer): List<TextMeasureRequest> {
        val buf = buffer.duplicate().order(ByteOrder.LITTLE_ENDIAN)
        if (buf.remaining() < 8) return emptyList()

        val magic = buf.int
        if (magic != MAGIC_FXTM) {
            throw IllegalArgumentException("Invalid text measure magic header: 0x${Integer.toHexString(magic)}")
        }

        val count = buf.int
        val requests = ArrayList<TextMeasureRequest>(count)

        for (i in 0 until count) {
            val reqId = buf.int
            val fontSize = buf.float
            val weight = buf.short.toInt() and 0xFFFF
            val style = buf.get().toInt() and 0xFF
            val maxWidth = buf.float
            val textLen = buf.int
            val textBytes = ByteArray(textLen)
            buf.get(textBytes)
            val text = String(textBytes, StandardCharsets.UTF_8)

            requests.add(TextMeasureRequest(reqId, text, fontSize, weight, style, maxWidth))
        }
        return requests
    }

    /**
     * Encodes text measurement responses to be returned to Rust.
     */
    fun encodeTextMeasureResponses(responses: List<TextMeasureResponse>): ByteBuffer {
        val buf = ByteBuffer.allocateDirect(8 + responses.size * 12).order(ByteOrder.LITTLE_ENDIAN)
        buf.putInt(MAGIC_FXTR)
        buf.putInt(responses.size)

        for (resp in responses) {
            buf.putInt(resp.requestId)
            buf.putFloat(resp.width)
            buf.putFloat(resp.height)
        }
        buf.flip()
        return buf
    }
}
