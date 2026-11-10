# Performance Benchmark Log

| Date | Milestone / Task | Metric | Baseline | Result | Delta | Notes |
|---|---|---|---|---|---|---|
| 2026-11-09 | M1: Native Foundation | JNI roundtrip dispatch | N/A | TBD | TBD | Initial JNI command buffer benchmark |
| 2026-11-09 | M7: Production | JNI Calls per Interaction | React Native: 3-8 calls | **1 call** | -67% to -87% | Strictly 1 JNI call per user event |
| 2026-11-09 | M7: Production | Patch Encode Throughput | JSON: ~500k/s | **60.6M patches/s** | +120x | Binary `DirectByteBuffer` encoding |
| 2026-11-09 | M7: Production | Reconciliation (300 nodes)| React Native: 1.8-4.5 ms | **0.052 ms (52 µs)** | -97% latency | Diffing 300 nodes takes 0.3% of 16.6ms frame |
| 2026-11-09 | M7: Production | Signal Propagation | React state: 200-1000 µs | **510.74 ns** | ~1000x | Fine-grained dependency tracking |
| 2026-11-09 | M7: Production | Process Memory (PSS) | React Native: 110-165 MB | **44.1 MB** | -60% to -73% | Measured with dumpsys meminfo |
| 2026-11-09 | M7: Production | Cold Start Time (P50) | React Native: 1200-2800 ms | **640-950 ms** | 2.5x faster | Measured with `am start-activity -W` |
| 2026-11-10 | Phase 1 Styling | 10k-Node Style Resolution | React Native / CSS-in-JS: ~150-300 ms | **45.91 ms (4.59 µs/node)** | 3-6x faster | Full cascade, specificity, media queries, style registry interning |
| 2026-11-10 | Phase 1 Styling | Style Theme Switch JNI | Ad-hoc styles: 100+ calls | **1 batched call (150 B)** | -99% calls | Single `DirectByteBuffer` atomic buffer dispatch |
| 2026-11-10 | Phase 1 Styling | Unchanged Style Interaction | CSS re-render: 1-10 ms | **0 patches / 0 JNI calls** | 0 overhead | Interaction cache filter in `ReactiveStyleResolver` |
| 2026-11-10 | Phase 2 Styling | Fast-Path Native CSS Compile | LightningCSS: 1-5 ms | **0.166 ms** | Sub-millisecond | Direct Rust compilation when no PostCSS config |
| 2026-11-10 | Phase 2 Styling | Hot Style Reload (50 nodes) | React Native Fast Refresh: ~100-300 ms | **1.649 ms** | -98% latency | Atomic `PatchBatch` without restarting Activity |
