package com.ferrix.runtime

import com.ferrix.runtime.protocol.CommandDecoder
import com.ferrix.runtime.protocol.EventType
import com.ferrix.runtime.protocol.Patch
import com.ferrix.runtime.protocol.Property
import com.ferrix.runtime.protocol.ViewType
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test
import java.nio.ByteBuffer
import java.nio.ByteOrder
import java.nio.charset.StandardCharsets

class CommandDecoderTest {

    @Test
    fun testDecodePatchBuffer() {
        val payload = ByteBuffer.allocate(1024).order(ByteOrder.LITTLE_ENDIAN)

        // 1. CreateNode(1, Column)
        payload.put(CommandDecoder.OP_CREATE_NODE)
        payload.putLong(1L)
        payload.putShort(ViewType.COLUMN.value)

        // 2. CreateNode(2, Text)
        payload.put(CommandDecoder.OP_CREATE_NODE)
        payload.putLong(2L)
        payload.putShort(ViewType.TEXT.value)

        // 3. SetText(2, "Hello Ferrix")
        val textBytes = "Hello Ferrix".toByteArray(StandardCharsets.UTF_8)
        payload.put(CommandDecoder.OP_SET_TEXT)
        payload.putLong(2L)
        payload.putInt(textBytes.size)
        payload.put(textBytes)

        // 4. SetProperty(2, FontSize(20.0f))
        payload.put(CommandDecoder.OP_SET_PROPERTY)
        payload.putLong(2L)
        payload.putShort(3.toShort()) // FontSize propId = 3
        payload.putShort(4.toShort()) // 4 bytes float
        payload.putFloat(20.0f)

        // 5. AppendChild(1, 2)
        payload.put(CommandDecoder.OP_APPEND_CHILD)
        payload.putLong(1L)
        payload.putLong(2L)

        // 6. AttachEventListener(2, Click)
        payload.put(CommandDecoder.OP_ATTACH_EVENT_LISTENER)
        payload.putLong(2L)
        payload.putShort(EventType.CLICK.value)

        val payloadSize = payload.position()

        // Assemble full buffer with 16-byte header
        val fullBuffer = ByteBuffer.allocate(16 + payloadSize).order(ByteOrder.LITTLE_ENDIAN)
        fullBuffer.put(0x46.toByte()) // 'F'
        fullBuffer.put(0x58.toByte()) // 'X'
        fullBuffer.put(0x55.toByte()) // 'U'
        fullBuffer.put(0x49.toByte()) // 'I'
        fullBuffer.putShort(1.toShort()) // version = 1
        fullBuffer.putShort(0.toShort()) // flags = 0
        fullBuffer.putInt(6) // command count = 6
        fullBuffer.putInt(payloadSize) // payload len
        fullBuffer.put(payload.array(), 0, payloadSize)
        fullBuffer.flip()

        val patches = CommandDecoder.decode(fullBuffer)
        assertEquals(6, patches.size)

        assertEquals(Patch.CreateNode(1L, ViewType.COLUMN), patches[0])
        assertEquals(Patch.CreateNode(2L, ViewType.TEXT), patches[1])
        assertEquals(Patch.SetText(2L, "Hello Ferrix"), patches[2])
        assertEquals(Patch.SetProperty(2L, Property.FontSize(20.0f)), patches[3])
        assertEquals(Patch.AppendChild(1L, 2L), patches[4])
        assertEquals(Patch.AttachEventListener(2L, EventType.CLICK), patches[5])
    }

    @Test(expected = IllegalArgumentException::class)
    fun testInvalidMagicHeader() {
        val buffer = ByteBuffer.allocate(16).order(ByteOrder.LITTLE_ENDIAN)
        buffer.putInt(0x12345678) // Invalid magic
        buffer.putShort(1)
        buffer.putShort(0)
        buffer.putInt(0)
        buffer.putInt(0)
        buffer.flip()

        CommandDecoder.decode(buffer)
    }
}
