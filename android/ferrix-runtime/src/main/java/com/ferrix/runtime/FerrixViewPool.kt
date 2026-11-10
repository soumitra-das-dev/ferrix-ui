package com.ferrix.runtime

import android.view.View
import com.ferrix.runtime.protocol.ViewType
import java.util.ArrayDeque

/**
 * Recycles and pools native Android Views by ViewType to prevent allocation churn.
 */
class FerrixViewPool(private val maxPoolSizePerType: Int = 30) {
    private val pools = HashMap<ViewType, ArrayDeque<View>>()

    /**
     * Acquires a recycled view of the given type, or null if pool is empty.
     */
    fun acquire(viewType: ViewType): View? {
        val queue = pools[viewType] ?: return null
        return if (queue.isNotEmpty()) queue.poll() else null
    }

    /**
     * Releases a destroyed view back into the pool after resetting its transient properties.
     */
    fun release(viewType: ViewType, view: View) {
        // Reset common properties before recycling
        view.translationX = 0f
        view.translationY = 0f
        view.scaleX = 1f
        view.scaleY = 1f
        view.rotation = 0f
        view.alpha = 1f
        view.elevation = 0f
        view.setOnClickListener(null)
        view.setOnTouchListener(null)
        view.tag = null
        view.contentDescription = null

        val queue = pools.getOrPut(viewType) { ArrayDeque(maxPoolSizePerType) }
        if (queue.size < maxPoolSizePerType) {
            queue.offer(view)
        }
    }

    /**
     * Clears all pooled views.
     */
    fun clear() {
        pools.clear()
    }
}

