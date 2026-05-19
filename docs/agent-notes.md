# Agent Notes

- Updated `h()` in `src/jsx-runtime.ts` to avoid `children.flat()` by manually flattening one level into a `flat` array before calling `createElement`.
- Ran benchmark builds for Vitrio, Solid, and React and refreshed `results.md` plus benchmark summary numbers in `docs/benchmarks.md`.
- Clarified benchmark write-up to reflect which metrics are leading vs still behind.
- Optimized `<For>` update paths and adjusted benchmark harness/load timing, then reran benchmarks.
- Updated README performance tables and copy to mirror `results.md`, aligning English/Japanese sections and the license year.
- Added `batch(fn)` to core API (`src/core.ts`, exported from `src/index.ts`) to group multiple updates into a single subscriber flush.
- Aligned docs around implemented capabilities (Suspense/ErrorBoundary/SSR string rendering) and refreshed API/getting-started references in EN/JA docs.
- Added commercial-readiness docs: `SECURITY.md`, `SUPPORT.md`, `CONTRIBUTING.md`, `CHANGELOG.md`.
- Added baseline CI workflow (`.github/workflows/ci.yml`) and replaced broken/missing wasm scripts with Node-based checks (`scripts/ensure-wasm.js`, `scripts/check-wasm.js`).
- Added `startTransition(fn)` to schedule non-urgent updates (internally batched) for smoother interactions in heavy update paths.
- Enhanced `createResource` with production-oriented options (`initialValue`, `retries`, `retryDelayMs`, `onError`) and updated EN/JA resource docs accordingly.
- Updated README/API/getting-started/capabilities docs to include transition and resource resilience guidance.
- Optimized `propagate` WASM function in `signal.rs` by switching from expensive stack-allocated array zeroing to static mutable global arrays (`QUEUE` and `VISITED_WORDS`). This recovers WASM port speed regressions and avoids `memset` bottlenecks.
