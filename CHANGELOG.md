# Changelog

All notable changes to the Vyro Ecosystem are documented here. Format based on [Keep a Changelog](https://keepachangelog.com); versioning follows [SemVer](https://semver.org).

## [Unreleased]

### Changed
- Analyzed existing repos (VyroCoding, VyroOs) and re-based the plan on reality: VyroCoding is the built IDE/Cloud/AI platform (evolve, not rebuild); VyroOs is a built OS (reference). Updated Component Map statuses and Dependency Graph reuse mapping.

### Added
- **Maps and `match`:** map literals `{ "a": 1 }` with `m[key]` get/set (missing →
  `null`), `keys(m)`, `has(m, key)`, and `len` on maps; a `match` expression with
  literal patterns and `_` wildcard. New opcodes `Dup`/`NewMap`, `Value::Map`. Tests
  in `tests/features.rs` (31 total). New `examples/maps_match.vy` and IDE examples.
- **Self-contained Vyro stack (the architecture diagram, in this repo):**
  - `impl/src/server.rs` — zero-dependency **Compiler API** (`vyro serve`): `GET /`,
    `/health`, `POST /api/run`, `POST /api/compile`.
  - `impl/web/index.html` — your own **VyroIDE** (editor, examples, stdin, console,
    Ctrl+Enter to run), embedded into the binary.
  - **Sandboxed VyroVM**: per-request instruction budget (50M) + wall-clock deadline
    (5s), captured stdout, request-fed `input()` (no host stdio). New `Vm::sandboxed`.
  - **Docker Sandbox**: `impl/Dockerfile` + `impl/scripts/sandbox-run.sh` (CPU/RAM/PID
    limits, read-only fs, cap-drop, non-root).
  - `docs/01-architecture/STANDALONE_STACK.md` — box → code mapping for the diagram.
- **VyroLang runs in VyroCoding on the native VyroVM** (Judge0 bypassed for `.vy`):
  added an `input()` native to the language; added a Vyro execution service +
  language dispatcher in VyroCoding's API (`Language.Vyro`, `runner.service.ts`,
  `vyro.service.ts`), wired into the BullMQ worker and run/submit routes, with a
  starter template in the language picker. See
  [VyroCoding Integration](docs/10-cloud/VYROCODING_INTEGRATION.md). New
  `examples/sum_stdin.vy`. Verified end to end (accepted / wrong_answer /
  compile_error / runtime_error).
- **Language core expanded:** arrays (literals, indexing, index-assignment), a standard
  library of built-ins (`len`, `push`, `pop`, `str`, `int`, `float`, `abs`, `sqrt`,
  `floor`, `ceil`, `pow`, `min`, `max`, `upper`, `lower`, `type`), string indexing, and
  **classes** (fields, `init`, methods, `self`, property get/set, method calls). VM call
  convention reworked to clox-style frames (slot 0 = callee/`self`) to support methods.
  25 tests passing (`tests/run.rs`, `tests/features.rs`). New `examples/todo.vy`.
- **`impl/` — first working Vyro toolchain (Rust):** lexer → parser → bytecode compiler → stack VM, exposed as the `vyro` CLI (`run` / `check` / `version`). Supports variables, Int/Float/Bool/String/null, arithmetic + string concat, comparisons, short-circuit `&&`/`||`, `if/else if/else`, `while`, `for i in a..b`, functions with recursion and first-class values, and `print`. 12 end-to-end tests passing.
- `examples/` — runnable VyroLang programs: `hello.vy`, `fib.vy`, `loops.vy` (factorial + FizzBuzz).
- `docs/00-overview/EXISTING_REPO_ANALYSIS.md` — analysis + integration plan for VyroCoding and VyroOs.
- Complete ecosystem planning and specification set.
- Architecture documents: system architecture, component map, dependency graph, scalability strategy.
- Component designs: VyroLang, Compiler, VM, OS layer, vpm, IDE, AI, Cloud.
- Security model, VPS deployment plan, CI/CD pipeline plan.
- Reference application designs: Todo, Calculator, Tic-Tac-Toe.
- Time-boxed roadmaps (30/90/180/365 day) and per-release version plans (v0.1.0 → v1.0.0).
- Project rules: CONTRIBUTING, SECURITY, CODE_OF_CONDUCT, LICENSE (MIT).

## [0.0.0] - 2026-06-15
### Added
- Repository initialized as the ecosystem planning hub.
