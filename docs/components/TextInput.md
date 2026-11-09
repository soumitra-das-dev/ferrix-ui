# TextInput Component Specification

`TextInput` is the primitive component for interactive text entry via the soft keyboard or physical keyboard.

## 1. Native Mapping & Architecture
- **Android**: `android.widget.EditText`.
- **Controlled Input with Version Counter**:
  To completely eliminate cursor jumps and infinite feedback loops between native `TextWatcher` events and Rust state reconciliations, each text update carries a monotonically increasing `version: u32`. If Rust sends back a text patch with the same version as the native view's current typing session, the native cursor position is preserved verbatim.

## 2. Props & Properties
```fsx
type KeyboardType = "default" | "email-address" | "numeric" | "phone-pad" | "ascii-capable" | "url";
type ReturnKeyType = "done" | "go" | "next" | "search" | "send";

type TextInputProps = {
    value?: String,
    defaultValue?: String,
    placeholder?: String,
    placeholderTextColor?: u32,
    secureTextEntry?: bool,
    multiline?: bool,
    numberOfLines?: i32,
    maxLength?: i32,
    editable?: bool,
    autoFocus?: bool,
    keyboardType?: KeyboardType,
    returnKeyType?: ReturnKeyType,
    onChangeText?: (text: String) => void,
    onSubmitEditing?: () => void,
    onFocus?: () => void,
    onBlur?: () => void,
    onSelectionChange?: (start: i32, end: i32) => void,
    style?: TextStyle | TextStyle[],
    testID?: String,
};
```
