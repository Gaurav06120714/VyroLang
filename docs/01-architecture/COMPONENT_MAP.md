# Component Map

A directory of every component, its repository home, its language, and what it depends on.

## Components

| Component | Repo | Language | Status | Depends on |
|---|---|---|---|---|
| VyroLang (spec) | VyroLang | — | Spec | — |
| VyroCompiler | VyroCoding | Rust | Planned | VyroLang spec |
| VyroVM | VyroCoding | Rust | Planned | Compiler (bytecode) |
| VyroOS Layer | VyroOs | Rust | Research | VM |
| vpm | VyroCoding | Rust | Planned | Compiler, Registry |
| Registry | VyroCloud (svc) | TypeScript | Planned | PostgreSQL |
| VyroIDE | VyroCloud (web) | TS/Next.js | Planned | Compiler API, AI |
| VyroAI | VyroCloud (svc) | TS/Python | Planned | Model provider |
| VyroCloud | VyroCloud (svc) | TypeScript | Planned | Sandbox, DB |
| VyroDeploy | VyroCloud (svc) | TypeScript | Planned | Cloud, Nginx |
| Sandbox runner | VyroCloud (infra) | Docker/Rust | Planned | VM, VyroOS |

> Note: VyroCloud, VyroIDE, VyroDeploy, VyroAI, and Registry are services/web apps that will live together in a `VyroCloud` repo (to be created); the compiler/VM/vpm evolve inside `VyroCoding`; OS work in `VyroOs`.

## Logical Layers

```
┌──────────────────────────────────────────────┐
│ Experience:  VyroIDE · VyroAI                 │
├──────────────────────────────────────────────┤
│ Platform:    VyroCloud · VyroDeploy · vpm     │
├──────────────────────────────────────────────┤
│ Runtime:     VyroVM · VyroOS Layer            │
├──────────────────────────────────────────────┤
│ Toolchain:   VyroCompiler                     │
├──────────────────────────────────────────────┤
│ Language:    VyroLang                          │
├──────────────────────────────────────────────┤
│ Infra:       Docker · Nginx · Postgres · Redis│
└──────────────────────────────────────────────┘
```

## Ownership of Interfaces

| Interface | Producer | Consumer | Versioned by |
|---|---|---|---|
| Source grammar | VyroLang spec | Compiler, IDE | Language spec |
| Bytecode format | Compiler | VM | VM spec |
| VyroOS capability API | VyroOS | VM, stdlib | OS spec |
| Compiler API (HTTP) | Compiler service | IDE, Cloud | OpenAPI |
| Package manifest | vpm | Registry, Compiler | vpm spec |
