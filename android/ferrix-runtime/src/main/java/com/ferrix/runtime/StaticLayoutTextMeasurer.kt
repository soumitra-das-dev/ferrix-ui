package com.ferrix.runtime

import android.content.Context
import android.graphics.Typeface
import android.os.Build
import android.text.Layout
import android.text.StaticLayout
import android.text.TextPaint
import com.ferrix.runtime.protocol.TextMeasureRequest
import com.ferrix.runtime.protocol.TextMeasureResponse

/**
 * Batched intrinsic text measurement engine using native Android [StaticLayout].
 * Guarantees 100% platform font matching and OEM line-breaking without per-node JNI chatter.
 */
class StaticLayoutTextMeasurer(private val context: Context) {
    private val textPaint = TextPaint(TextPaint.ANTI_ALIAS_FLAG)

    /**
     * Measures a batch of text requests and returns their intrinsic dimensions in density-independent pixels (dp).
     */
    fun measureBatch(requests: List<TextMeasureRequest>): List<TextMeasureResponse> {
        val density = context.resources.displayMetrics.density
        val scaledDensity = context.resources.displayMetrics.scaledDensity

        val responses = ArrayList<TextMeasureResponse>(requests.size)

        for (req in requests) {
            textPaint.reset()
            textPaint.isAntiAlias = true
            textPaint.textSize = req.fontSizeSp * scaledDensity

            val style = if (req.fontWeight >= 600 && req.fontStyle == 1) {
                Typeface.BOLD_ITALIC
            } else if (req.fontWeight >= 600) {
                Typeface.BOLD
            } else if (req.fontStyle == 1) {
                Typeface.ITALIC
            } else {
                Typeface.NORMAL
            }
            textPaint.typeface = Typeface.create(Typeface.DEFAULT, style)

            val maxWidthPx = if (req.maxWidth > 0f) (req.maxWidth * density).toInt() else Int.MAX_VALUE / 2
            val targetWidth = if (maxWidthPx <= 0) 1 else maxWidthPx

            val layout = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.M) {
                StaticLayout.Builder.obtain(req.text, 0, req.text.length, textPaint, targetWidth)
                    .setAlignment(Layout.Alignment.ALIGN_NORMAL)
                    .setIncludePad(true)
                    .build()
            } else {
                @Suppress("DEPRECATION")
                StaticLayout(
                    req.text,
                    textPaint,
                    targetWidth,
                    Layout.Alignment.ALIGN_NORMAL,
                    1.0f,
                    0.0f,
                    true
                )
            }

            var maxLineWidth = 0f
            for (line in 0 until layout.lineCount) {
                val lineWidth = layout.getLineWidth(line)
                if (lineWidth > maxLineWidth) {
                    maxLineWidth = lineWidth
                }
            }

            val measuredWidthDp = maxLineWidth / density
            val measuredHeightDp = layout.height.toFloat() / density

            responses.add(
                TextMeasureResponse(
                    requestId = req.requestId,
                    width = measuredWidthDp,
                    height = measuredHeightDp
                )
            )
        }

        return responses
    }
}

