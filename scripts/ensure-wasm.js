#!/usr/bin/env node
const fs = require('fs');
const path = require('path');

const root = process.cwd();
const distWasm = path.join(root, 'dist', 'vsignal.wasm');
const srcWasm = path.join(root, 'src', 'vsignal.wasm');
const srcRs = path.join(root, 'src', 'vsignal', 'signal.rs');

if (fs.existsSync(distWasm)) {
  console.log('OK: dist/vsignal.wasm exists');
  process.exit(0);
}

if (fs.existsSync(srcWasm)) {
  console.log('OK: src/vsignal.wasm exists (build step can package it)');
  process.exit(0);
}

if (fs.existsSync(srcRs)) {
  console.error('WASM binary not found. Build it with `npm run build:wasm` before packaging.');
  process.exit(1);
}

console.error('Missing wasm source and binary artifacts.');
process.exit(1);
