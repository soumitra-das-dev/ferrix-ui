# Image & ImageBackground Component Specification

`Image` renders raster and vector graphics from network URLs, local Android resources/assets, and Base64 data URIs. `ImageBackground` renders child components on top of an image layer.

## 1. Native Mapping
- **Android**: `android.widget.ImageView` with image loading, memory caching, and disk caching backed by Coil / standard bitmap decoders.
- Supports tint color, blur radius, rounded corner clipping, and resize modes (`cover`, `contain`, `stretch`, `center`).

## 2. Props & Properties
```fsx
type ImageResizeMode = "cover" | "contain" | "stretch" | "repeat" | "center";

type ImageSource = {
    uri: String,
    headers?: Record<String, String>,
};

type ImageProps = {
    source: String | ImageSource,
    style?: Style | Style[],
    resizeMode?: ImageResizeMode,
    blurRadius?: f32,
    tintColor?: u32,
    defaultSource?: String,
    onLoad?: () => void,
    onError?: (error: String) => void,
    onProgress?: (loaded: i64, total: i64) => void,
    accessible?: bool,
    accessibilityLabel?: String,
    testID?: String,
};
```
