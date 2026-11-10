// PostCSS configuration for Ferrix native styling pipeline
const path = require('path');
const ferrixPlugin = require('../../packages/postcss-plugin-ferrix');

module.exports = {
  plugins: [
    ferrixPlugin({
      rootFontSize: 16.0,
    }),
  ],
};

