package com.ferrix.runtime

import android.view.ViewGroup
import android.widget.FrameLayout
import android.widget.TextView
import androidx.recyclerview.widget.RecyclerView

/**
 * High-performance virtualized adapter for Ferrix LazyList / RecyclerView nodes.
 * Maintains a window of recycled item views with bounded memory.
 */
class FerrixRecyclerAdapter(
    private var count: Int = 0,
    private var itemHeightPx: Int = -1
) : RecyclerView.Adapter<FerrixRecyclerAdapter.ViewHolder>() {

    class ViewHolder(val container: FrameLayout, val textView: TextView) :
        RecyclerView.ViewHolder(container)

    fun setItemCount(newCount: Int) {
        if (count != newCount) {
            count = newCount
            notifyDataSetChanged()
        }
    }

    fun setItemHeight(heightPx: Int) {
        if (itemHeightPx != heightPx) {
            itemHeightPx = heightPx
            notifyDataSetChanged()
        }
    }

    override fun getItemCount(): Int = count

    override fun onCreateViewHolder(parent: ViewGroup, viewType: Int): ViewHolder {
        val container = FrameLayout(parent.context).apply {
            val h = if (itemHeightPx > 0) itemHeightPx else ViewGroup.LayoutParams.WRAP_CONTENT
            layoutParams = ViewGroup.LayoutParams(ViewGroup.LayoutParams.MATCH_PARENT, h)
        }
        val textView = TextView(parent.context).apply {
            setPadding(32, 24, 32, 24)
            textSize = 16f
        }
        container.addView(textView)
        return ViewHolder(container, textView)
    }

    override fun onBindViewHolder(holder: ViewHolder, position: Int) {
        if (itemHeightPx > 0) {
            holder.container.layoutParams.height = itemHeightPx
        }
        holder.textView.text = "Item #$position"
    }
}
