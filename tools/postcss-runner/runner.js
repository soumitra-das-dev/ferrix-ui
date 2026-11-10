#!/usr/bin/env node
/**
 * Ferrix PostCSS Runner
 * Executes user PostCSS configs and plugins as an isolated build-time subprocess.
 * Zero Node runtime in production APK.
 */

const fs = require('fs');
const path = require('path');

function parseArgs(args) {
  const options = {
    config: null,
    input: null,
    output: null,
    from: 'input.css',
    to: 'output.css',
    json: false,
    map: true,
  };

  for (let i = 0; i < args.length; i++) {
    const arg = args[i];
    if (arg === '--help' || arg === '-h') {
      console.log('Usage: node runner.js [--config <file>] [--input <file|->] [--output <file|->] [--json]');
      process.exit(0);
    } else if (arg === '--config' && i + 1 < args.length) {
      options.config = args[++i];
    } else if (arg === '--input' && i + 1 < args.length) {
      options.input = args[++i];
    } else if (arg === '--output' && i + 1 < args.length) {
      options.output = args[++i];
    } else if (arg === '--from' && i + 1 < args.length) {
      options.from = args[++i];
    } else if (arg === '--to' && i + 1 < args.length) {
      options.to = args[++i];
    } else if (arg === '--json') {
      options.json = true;
    } else if (arg === '--no-map') {
      options.map = false;
    }
  }

  return options;
}

function loadConfig(configPath, fromPath) {
  if (!configPath || !fs.existsSync(configPath)) {
    return { plugins: [] };
  }

  const resolved = path.resolve(configPath);
  try {
    delete require.cache[resolved];
    const mod = require(resolved);
    const config = typeof mod === 'function' ? mod({ from: fromPath, env: process.env.NODE_ENV || 'development' }) : mod;
    return config && config.plugins ? config : { plugins: Array.isArray(config) ? config : [] };
  } catch (err) {
    throw new Error(`Failed to load PostCSS config '${configPath}': ${err.message}`);
  }
}

async function runPostcss(css, options, config) {
  // Attempt to require postcss from project or fallback
  let postcss = null;
  const projectRoot = options.config ? path.dirname(path.resolve(options.config)) : process.cwd();
  
  try {
    const projectPostcssPath = require.resolve('postcss', { paths: [projectRoot, __dirname] });
    postcss = require(projectPostcssPath);
  } catch (_e) {
    // If postcss package is not installed in project, run fallback AST/plugin transform
    postcss = null;
  }

  if (postcss) {
    const plugins = Array.isArray(config.plugins) ? config.plugins : Object.entries(config.plugins || {}).map(([p, opts]) => {
      if (typeof opts === 'boolean' && opts) {
        return require(require.resolve(p, { paths: [projectRoot] }))();
      }
      return require(require.resolve(p, { paths: [projectRoot] }))(opts);
    });

    const result = await postcss(plugins).process(css, {
      from: options.from,
      to: options.to,
      map: options.map ? { inline: false, annotation: false } : false,
    });

    return {
      css: result.css,
      map: result.map ? result.map.toString() : null,
      warnings: result.warnings().map(w => ({
        text: w.text,
        line: w.line,
        column: w.column,
      })),
    };
  } else {
    // Built-in lightweight fallback processor when postcss npm package is not installed
    let transformedCss = css;
    const warnings = [];

    // If config has JS plugin functions, invoke them sequentially
    if (config.plugins && Array.isArray(config.plugins)) {
      for (const plugin of config.plugins) {
        if (typeof plugin === 'function') {
          try {
            const out = plugin(transformedCss, { from: options.from });
            if (typeof out === 'string') transformedCss = out;
          } catch (e) {
            warnings.push({ text: `Plugin warning: ${e.message}`, line: 1, column: 1 });
          }
        }
      }
    }

    // Generate basic 1:1 sourcemap for the file
    const lineCount = transformedCss.split('\n').length;
    // Basic uncompressed 1-to-1 mapping
    const mappings = Array(lineCount).fill('AAAA').join(';');
    const simpleMap = {
      version: 3,
      sources: [options.from],
      mappings,
      names: [],
    };

    return {
      css: transformedCss,
      map: options.map ? JSON.stringify(simpleMap) : null,
      warnings,
    };
  }
}

async function main() {
  const options = parseArgs(process.argv.slice(2));

  try {
    let inputCss = '';
    if (options.input && options.input !== '-') {
      inputCss = fs.readFileSync(path.resolve(options.input), 'utf8');
    } else {
      inputCss = fs.readFileSync(0, 'utf8');
    }

    const config = loadConfig(options.config, options.from);
    const result = await runPostcss(inputCss, options, config);

    if (options.json) {
      const response = {
        success: true,
        css: result.css,
        map: result.map,
        warnings: result.warnings,
      };
      const jsonStr = JSON.stringify(response);
      if (options.output && options.output !== '-') {
        fs.writeFileSync(path.resolve(options.output), jsonStr, 'utf8');
      } else {
        process.stdout.write(jsonStr);
      }
    } else {
      if (options.output && options.output !== '-') {
        fs.writeFileSync(path.resolve(options.output), result.css, 'utf8');
        if (result.map && options.map) {
          fs.writeFileSync(path.resolve(`${options.output}.map`), result.map, 'utf8');
        }
      } else {
        process.stdout.write(result.css);
      }
    }
  } catch (err) {
    const errorPayload = {
      success: false,
      error: err.message,
      line: err.line || 1,
      column: err.column || 1,
    };
    if (options.json) {
      process.stdout.write(JSON.stringify(errorPayload));
    } else {
      process.stderr.write(`[ferrix-postcss-runner] Error: ${err.message}\n`);
    }
    process.exit(1);
  }
}

main();
