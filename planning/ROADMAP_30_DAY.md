# 30-Day Roadmap — Language · Compiler · VM

**Theme:** Stand up a working core: a small VyroLang program compiles to bytecode and runs on the VM.

**Target releases:** [v0.1.0](../versions/v0.1.0-foundation.md) → [v0.2.0](../versions/v0.2.0-compiler-frontend.md) → [v0.3.0](../versions/v0.3.0-vm-runtime.md)

## Week 1 — Foundation & Language Spec
- [ ] Freeze VyroLang core grammar (v0.1) — [Language Spec](../docs/02-vyrolang/LANGUAGE_SPEC.md).
- [ ] Audit `VyroCoding` + `VyroOs`; map reusable code ([Dependency Graph](../docs/01-architecture/DEPENDENCY_GRAPH.md)).
- [ ] Set up Rust workspace, CI (build/test/lint), repo rules.
- [ ] Define bytecode format draft.

## Week 2 — Compiler Frontend
- [ ] Lexer with source spans + tests.
- [ ] Recursive-descent + Pratt parser → AST.
- [ ] Diagnostics framework (codes, spans, hints).
- [ ] Golden tests for AST.

## Week 3 — Semantic Analysis + Codegen
- [ ] Symbol table + scope resolution.
- [ ] Type inference + checking (basic).
- [ ] IR lowering + naive codegen to bytecode.
- [ ] `vyroc build` produces `.vbc`.

## Week 4 — VM Core
- [ ] Operand/call stacks, constant pool.
- [ ] Core opcodes (arith, locals, control flow, CALL/RETURN).
- [ ] Heap + objects + arrays.
- [ ] End-to-end: `.vy` → compile → run → correct output.

## Exit Criteria
- Hello-world + arithmetic + functions + loops compile and run.
- CI green; golden tests cover the pipeline.
- Tag **v0.3.0**.

## Risks
| Risk | Mitigation |
|---|---|
| Frontend slips | cut grammar to essentials; defer classes/async |
| Codegen/VM drift | shared bytecode spec + golden tests |
