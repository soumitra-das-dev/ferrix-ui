# Modal Component Specification

`Modal` renders a native modal dialog overlay on top of the parent view tree.

## 1. Native Mapping
- **Android**: `android.app.Dialog` with `android.R.style.Theme_Translucent_NoTitleBar_Fullscreen`.
- Intercepts hardware back button presses and notifies Rust via `onRequestClose`.

## 2. Props & Properties
```fsx
type ModalAnimationType = "none" | "slide" | "fade";

type ModalProps = {
    visible: bool,
    transparent?: bool,
    animationType?: ModalAnimationType,
    onRequestClose?: () => void,
    onShow?: () => void,
    children?: Element | Element[],
    testID?: String,
};
```
