# Documentation Plan

## Goal

Documentation good enough that a newcomer ships an app on day one, and a contributor understands the internals.

## Two Tracks

### User Documentation
- **Installation** — get the toolchain / use the Cloud IDE.
- **Language Guide** — VyroLang from variables to async.
- **Tutorials** — build the Todo, Calculator, and Tic-Tac-Toe apps step by step.
- **Examples** — a gallery of small, runnable programs.
- **CLI Reference** — `vyroc`, `vyrovm`, `vpm`.
- **Cloud & Deploy Guide** — run and ship apps.

### Developer Documentation
- **Compiler Internals** — pipeline, IR, passes, diagnostics.
- **VM Design** — opcodes, GC, scheduler, async.
- **OS Architecture** — capability model, FS/Process/Memory/Net.
- **Security Model** — sandbox + hardening.
- **Cloud Infrastructure** — orchestrator, runners, deploy.
- **Contributing & Rules** — workflow, conventions, testing.

## Sources of Truth

| Topic | Lives in |
|---|---|
| Architecture | [docs/01-architecture](../01-architecture) |
| Language | [docs/02-vyrolang](../02-vyrolang) |
| Compiler | [docs/03-compiler](../03-compiler) |
| VM | [docs/04-vm](../04-vm) |
| OS | [docs/05-vyroos](../05-vyroos) |
| Security | [docs/11-security](../11-security) |

## Tooling

- Markdown in-repo (this set) is the canonical source.
- Generate a docs site (e.g., Docusaurus/MkDocs) from these files for v1.0.
- API docs auto-generated from code (rustdoc, OpenAPI, TypeDoc).
- Doc examples are compiled/tested in CI so they never rot.

## Maintenance Rules

- Update docs in the same PR as code (see [CONTRIBUTING](../../CONTRIBUTING.md)).
- Each release updates [CHANGELOG](../../CHANGELOG.md) and relevant guides.
- Diagrams kept in sync with the [Component Map](../01-architecture/COMPONENT_MAP.md).

## Estimated Development Time

Living effort; dedicated docs-site pass before v1.0 (~1–2 weeks). ([v1.0.0](../../versions/v1.0.0-production.md))
