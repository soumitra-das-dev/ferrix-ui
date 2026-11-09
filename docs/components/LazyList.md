# LazyList (FlatList) Component Specification

`LazyList` (equivalent to React Native `FlatList`) is a high-performance virtualized list component designed to render arbitrarily large datasets (e.g. 10,000+ items) with constant memory footprint and 60 FPS scrolling.

## 1. Native Mapping & Virtualization
- **Android**: `androidx.recyclerview.widget.RecyclerView` with custom `LinearLayoutManager` and item view recycling.
- Rust owns the backing item data array, state updates, and item component templates.
- Kotlin maintains a windowed active view pool; items entering the visible window are populated from Rust-computed item states.

## 2. Props & Properties
```fsx
type LazyListProps<T> = {
    data: T[],
    renderItem: (info: { item: T, index: i32 }) => Element,
    keyExtractor?: (item: T, index: i32) => String,
    itemHeight?: f32, // Optional fixed height for getItemLayout optimization
    onEndReached?: () => void,
    onEndReachedThreshold?: f32,
    ListHeaderComponent?: Element,
    ListFooterComponent?: Element,
    ListEmptyComponent?: Element,
    refreshControl?: Element,
    testID?: String,
};
```
