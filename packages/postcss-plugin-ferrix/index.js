/**
 * @ferrix/postcss-plugin
 * Normalizes CSS and Tailwind utilities into the Ferrix-compatible mobile subset.
 */

const UNSUPPORTED_PROPERTIES = new Set([
  'float',
  'clear',
  'cursor',
  'user-select',
  'table',
  'table-layout',
  'columns',
  'column-count',
  'white-space',
  'grid-template-columns',
  'grid-template-rows',
  'grid-gap',
]);

const UNSUPPORTED_PSEUDOS = [
  '::before',
  '::after',
  '::placeholder',
  '::selection',
  '::-webkit-',
];

function convertUnits(val) {
  // Convert rem to px points (1rem = 16px)
  return val.replace(/([0-9.]+)rem\b/g, (_m, n) => {
    const px = parseFloat(n) * 16.0;
    return `${px}px`;
  });
}

const ferrixPostcssPlugin = (opts = {}) => {
  const rootFontSize = opts.rootFontSize || 16.0;

  return {
    postcssPlugin: 'postcss-plugin-ferrix',

    Declaration(decl, { result }) {
      const prop = decl.prop.toLowerCase();

      // Check unsupported properties
      if (UNSUPPORTED_PROPERTIES.has(prop)) {
        decl.warn(result, `CSS property '${prop}' is not supported in native mobile layouts.`, {
          word: decl.prop,
        });
      }

      // Convert rem units
      if (decl.value && decl.value.includes('rem')) {
        decl.value = decl.value.replace(/([0-9.]+)rem\b/g, (_m, n) => {
          const px = parseFloat(n) * rootFontSize;
          return `${px}px`;
        });
      }

      // Disallow position: fixed
      if (prop === 'position' && decl.value.trim().toLowerCase() === 'fixed') {
        decl.warn(result, `position: fixed is not supported in mobile layout; use absolute positioning instead.`, {
          word: decl.value,
        });
        decl.value = 'absolute';
      }
    },

    Rule(rule, { result }) {
      const sel = rule.selector;
      for (const pseudo of UNSUPPORTED_PSEUDOS) {
        if (sel.includes(pseudo)) {
          rule.warn(result, `Pseudo-element '${pseudo}' is not supported in native mobile layouts.`, {
            word: pseudo,
          });
          break;
        }
      }
    },
  };
};

// Standalone fallback execution function
ferrixPostcssPlugin.processString = function(css, _opts = {}) {
  // Simple regex-based fallback when postcss npm package is not installed
  let transformed = convertUnits(css);
  // Warn / adjust fixed positioning
  transformed = transformed.replace(/position\s*:\s*fixed/gi, 'position: absolute');
  return transformed;
};

ferrixPostcssPlugin.postcss = true;
module.exports = ferrixPostcssPlugin;
