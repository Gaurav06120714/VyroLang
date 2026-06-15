# Component Map

A directory of every component, its repository home, its language, and what it depends on.

## Components

Statuses reflect the real state after analyzing the existing repos — see [Existing Repo Analysis](../00-overview/EXISTING_REPO_ANALYSIS.md).

| Component | Repo | Language | Status | Depends on |
|---|---|---|---|---|
| VyroLang (spec) | VyroLang | — | Spec | — |
| VyroCompiler | (new) | Rust | **Greenfield** | VyroLang spec |
| VyroVM | (new) | Rust | **Greenfield** | Compiler (bytecode) |
| vpm | (new) | Rust | **Greenfield** | Compiler, Registry |
| VyroIDE | VyroCoding `apps/web` | TS/Next.js 15 | **Built — evolve** | Compiler API, AI |
| VyroAI | VyroCoding `apps/api` `/ai/*` | TS (SSE) | **Built — evolve** | Model provider |
| VyroCloud (exec) | VyroCoding `apps/api`+`apps/collab` | TS (BullMQ/Redis) | **Built — evolve** | Sandbox, DB |
| Realtime collab | VyroCoding `apps/collab` | TS (WebSocket) | **Built** | Redis Pub/Sub |
| Registry | (new) | TypeScript | Planned | PostgreSQL |
| VyroDeploy | (new) | TypeScript | Planned | Cloud, Nginx |
| Sandbox runner | (new) | Docker/Rust | Planned | VM, VyroOS |
| VyroOS (capability layer) | VyroOs | C/ASM (real OS) | **Built — referenced** | VM (contract) |

> Reality check: **VyroCoding** already implements the IDE/Cloud/AI/collab layers (Next.js 15 + Fastify + WebSocket + BullMQ; executes via Judge0) — evolve it rather than build new. **VyroOs** is a real, mature OS (from-scratch microkernel with net + TLS stack, plus Ubuntu remix and Linux-core paths, v7.3) — reference it; keep the capability layer as the thin sandbox contract. The genuinely new work is **VyroLang + Compiler + VM + vpm**.

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
