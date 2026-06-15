# 90-Day Roadmap — IDE · Apps · Cloud Runtime

**Theme:** A developer writes VyroLang in the browser, runs it in a sandbox, and sees output — proven by three real apps.

**Target releases:** [v0.4.0](../versions/v0.4.0-optimizer-stdlib.md) → [v0.6.0](../versions/v0.6.0-ide.md) → [v0.7.0](../versions/v0.7.0-cloud.md)

## Month 1 — Language Completeness + Optimizer
- [ ] Classes, async/await, error handling, modules.
- [ ] Optimizer passes: constant folding, DCE, inlining, loop opt.
- [ ] Standard library v1 (math, string, array, json, fs, http).
- [ ] GC (mark-and-sweep) + scheduler/async runtime in VM.

## Month 2 — Browser IDE
- [ ] Next.js + Monaco shell; `.vy` syntax highlighting.
- [ ] Compiler API; inline diagnostics.
- [ ] Integrated terminal (xterm) + run loop over WebSocket.
- [ ] File explorer + workspace persistence.
- [ ] LSP (completions/hover) backed by the compiler.

## Month 3 — Cloud Runtime + Reference Apps
- [ ] Docker sandbox runner with CPU/RAM/time/FS limits + no-net default.
- [ ] Orchestrator + Redis job queue; stream results.
- [ ] Build [Todo, Calculator, Tic-Tac-Toe](../docs/09-applications/REFERENCE_APPS.md) in VyroLang.
- [ ] Debugger (breakpoints/step) via VM debug protocol.

## Exit Criteria
- Write → compile → run in browser sandbox, end to end.
- Three reference apps run in VyroCloud.
- Sandbox escape tests pass. Tag **v0.7.0**.

## Risks
| Risk | Mitigation |
|---|---|
| Sandbox security | defense-in-depth + escape test suite |
| IDE scope | ship editor+run first; debugger/git after |
| Async runtime complexity | sync core first |
