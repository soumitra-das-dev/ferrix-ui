# @ferrix/postcss-plugin

Official PostCSS plugin for Ferrix native mobile applications.

## Features
- **Unit Normalization**: Automatically converts `rem` and `em` to density-independent points (`dp`/`px`).
- **Validation**: Detects web-only CSS properties (`float`, `cursor`, `columns`, `table`) and emits build-time diagnostics.
- **Position Normalization**: Warns on and normalizes `position: fixed` to `position: absolute`.
- **Selector Validation**: Guards against unsupported pseudo-elements (`::before`, `::after`, `::placeholder`).

## Usage
In your `postcss.config.js`:
```javascript
module.exports = {
  plugins: [
    require('@ferrix/postcss-plugin')({
      rootFontSize: 16.0,
    }),
  ],
};
```
