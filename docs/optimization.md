# Vitrio Optimization Guide

This document explains the optimizations implemented in the Vitrio framework and their effects.

## Overview

Vitrio achieves a high-performance reactive UI framework by bundling WASM written in V into JavaScript.

## Optimization Changelog

### 2026-01-15: For/Show Component Fixes

#### Issues

- `<For>` and `<Show>` components were not correctly processing `children` passed from JSX.
- The JSX runtime passes `children` as an array, so they could not be called directly as functions.
- List update benchmarks resulted in 0 items.

#### Fixes

1. **flow.ts: For Component**
   - Extracted the first element as `renderFn` if `props.children` is an array.
   - Fixed to render correctly even before the fragment is mounted to the DOM.

2. **flow.ts: Show Component**
   - Similarly added `getChild()` and `getFallback()` helper functions.
   - Properly handled element extraction from arrays.

3. **jsx-runtime.ts: Type Definitions**
   - Defined `JSX.Element` as `VNode` type (including DocumentFragment).
   - Fixed For/Show to be recognized as valid JSX components.

#### Results

| Metrics | Before | After | Improvement |
| ------- | ------ | ----- | ----------- |
| List Update | 12.7ms | **5.9ms** | **53% Faster** |
| Interaction | 11.9ms | **7.8ms** | **35% Faster** |
| Final list items | 0 | **50** | ✓ Working |

---

### 2026-01-15: WASM and JS Size Optimization

#### Changes

1. **V Build Flags Optimization** (`package.json`)
   - `-d no_bounds_checking`: Disable array bounds checking.
   - `-d no_backtrace`: Disable backtrace generation.
   - `-d no_stdio`: Remove standard I/O support.
   - `wasm-opt --strip-debug`: Remove debug information.

2. **Added `@[direct_array_access]` Attribute** (`src/vsignal/signal.v`)

3. **Removed console.log/warn** (`src/core.ts`)

#### Results

| Component | Before | After | Reduction |
| --------- | ------ | ----- | --------- |
| WASM Binary | 3,901 B | 1,511 B | **61% Reduction** |
| Main JS Chunk | 12,393 B | 10,261 B | **17% Reduction** |

### 2026-01-18: Subscription Cleanup

#### Changes

- Modified `<For>` / `<Show>` to ensure reactive bindings associated with nodes are disposed when nodes are removed.
- `render` also performs cleanup of existing nodes before mounting, removing residual subscriptions upon re-mount.

#### Results

- List Update: **4.50ms → 2.95ms** (Approx. 34% faster)
- 100 Clicks: **2.68ms → 2.18ms** (Approx. 19% faster)

### 2026-01-20: Code Quality Optimization (v0.1.7)

#### Changes

1. **Replaced `forEach` with `for-of`** (`core.ts`)
   - `flush()`: Changed spread operator to `Array.from()` and `forEach` to `for-of`.
   - `scheduleUpdates()`: Changed `forEach` to `for-of`.
   - `cleanupComponent()`: Similarly optimized.
   - `getAtomState()`: Changed dependency registration loop to `for-of`.
   - `updateDerived()`: Optimized spread operator and `forEach`.

2. **`cleanupNode` Optimization** (`jsx-runtime.ts`)
   - Changed `forEach` to `for-of` + `Array.from()`.

3. **Reactivity for `Switch`/`Match`** (`flow.ts`)
   - Fixed to update on signal changes instead of only evaluating condition on initial render.
   - Used `withRenderContext` to make it reactive.
   - Unified to marker-based DOM update method.

#### Results

- Functional improvement: Switch/Match now works reactively correctly.
- Code quality: Consistent use of for-of loops.
- Performance maintenance: Maintained 349% speed advantage over SolidJS.

## Benchmark Results (Latest: 2026-01-20)

| Metrics | Vitrio | SolidJS | React |
| ------- | ------ | ------- | ----- |
| Bundle Size | **11.9KB** | 13.0KB | 144.1KB |
| Load Time (ms) | **14.05** | 41.26 | 36.26 |
| Interaction (100 clicks, ms) | **2.45** | 11.02 | 11.74 |
| List Update (50 add, 25 remove, ms) | **3.47** | 11.84 | 9.18 |

### Analysis

- **List Update**: Vitrio is **241% faster** than Solid, **165% faster** than React.
- **Interaction**: Vitrio is **349% faster** than Solid, **379% faster** than React.
- **Bundle Size**: Vitrio is about 8% smaller than Solid, about 92% smaller than React.

## Build Commands

```bash
# Build WASM (Optimized)
bun run build:wasm

# Build JS
bun run build

# Generate Type Definitions
bun run build:types

# Build All
bun run prepare
```

## WASM Rust Optimizations
The WASM backend (`src/vsignal/signal.rs`) was heavily optimized to avoid runtime memory initialization bottlenecks:
- Removed local arrays `[0; 4096]` in `propagate` which invoked `memset` on every update.
- Utilized `static mut` arrays for `QUEUE` and `VISITED_WORDS` to achieve zero-allocation reactive graph traversal.
- `VISITED_WORDS` is manually cleared only for the bits touched, ensuring O(N) cleanup rather than O(Capacity).
