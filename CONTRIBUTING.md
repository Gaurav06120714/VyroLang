# Contributing & Engineering Rules

This document defines how the Vyro Ecosystem is built. These rules are mandatory — they keep the ecosystem coherent, secure, and production-grade as it grows across repositories.

## 1. Golden Rules

1. **Evolve, never rewrite.** Treat existing code in `VyroCoding` and `VyroOs` as the foundation. Reuse before rebuilding; preserve compatibility; write migration plans for breaking changes.
2. **Spec before code.** Every component ships a design doc (in `VyroLang/docs`) before implementation.
3. **Security is not optional.** Sandbox all untrusted execution; default-deny networking; least privilege everywhere.
4. **Small, reversible changes.** Prefer incremental PRs that can be reverted cleanly.
5. **Everything is testable.** No feature is "done" without tests and docs.

## 2. Output Standard for Every Feature

Each feature design (issue, RFC, or doc) must answer all of:

- Goal
- Architecture
- Folder Structure
- Technology Choice
- Database Schema (if stateful)
- API Design
- Security Considerations
- Implementation Steps
- Example Code
- Testing Strategy
- Future Improvements
- Estimated Development Time
- Risk Analysis

## 3. Branching & Commits

- Default branch: `main` (protected).
- Feature branches: `feat/<scope>-<short-desc>`, fixes: `fix/<scope>-<desc>`, docs: `docs/<scope>`.
- **Conventional Commits**: `feat:`, `fix:`, `docs:`, `refactor:`, `test:`, `chore:`, `ci:`, `perf:`, `sec:`.
- Commit after each completed, working task — do not batch unrelated work.
- Tag releases per the [versioning plan](./versions).

## 4. Code Review

- Every change to a protected branch goes through a PR.
- Required checks must pass: build, lint, test, security scan.
- Reviews focus on: correctness, security, simplicity, and adherence to the spec.

## 5. Coding Conventions

- **Languages**: Rust (compiler/VM), TypeScript/Next.js (IDE/Cloud/Deploy), Python (tooling/AI glue), VyroLang (reference apps).
- Match surrounding code style; keep functions small and named clearly.
- Public APIs are documented; internal invariants are commented where non-obvious.
- No secrets in source. Use environment variables and the deployment secret store.

## 6. Testing Strategy

- **Unit** tests for pure logic (lexer, parser, optimizer passes, VM opcodes).
- **Golden/snapshot** tests for compiler output and bytecode.
- **Integration** tests for end-to-end compile→run flows.
- **Fuzzing** for the lexer/parser and the sandbox boundary.
- **Security** tests for the sandbox (escape attempts must fail).
- Target ≥ 80% coverage on core compiler/VM crates.

## 7. Documentation

- Update the relevant `docs/` file in the same PR as the code change.
- User-facing changes update [CHANGELOG.md](./CHANGELOG.md).
- Keep diagrams and component maps in sync with reality.

## 8. Definition of Done

A task is done when: code merged, tests passing, docs updated, changelog noted, and (for releases) tagged.

## 9. Single-Maintainer Note

This ecosystem is currently authored and maintained solely by **Gaurav**. External contributions are not being accepted at this stage; the rules above govern the maintainer's own workflow to keep quality high.
