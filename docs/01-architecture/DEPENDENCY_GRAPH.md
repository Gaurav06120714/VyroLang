# Dependency Graph

## Build/Runtime Dependencies

```
VyroLang (spec)
    │
    ▼
VyroCompiler ──────► Bytecode format
    │                     │
    ▼                     ▼
  vpm                  VyroVM ──► VyroOS Layer
    │                     │            │
    ▼                     ▼            ▼
 Registry            Sandbox runner ───┘
    │                     │
    └────────┬────────────┘
             ▼
         VyroCloud ──► VyroDeploy
             │
             ├──► VyroIDE ──► VyroAI
             └──► PostgreSQL / Redis
```

## Phased Build Order

Each item depends only on those above it:

1. VyroLang spec
2. VyroCompiler (frontend: lexer, parser, sema)
3. Bytecode format
4. VyroVM (execution)
5. VyroCompiler (optimizer + codegen completeness)
6. Standard library
7. VyroOS capability layer
8. vpm + Registry
9. Sandbox runner
10. VyroCloud orchestrator
11. VyroIDE
12. VyroAI
13. VyroDeploy

## Reuse Mapping (existing repos → ecosystem)

Based on the [Existing Repo Analysis](../00-overview/EXISTING_REPO_ANALYSIS.md). VyroCoding is a built platform (not a compiler); VyroOs is a built OS.

| Existing asset | Reused as | Action |
|---|---|---|
| VyroCoding `apps/web` (Monaco, stores, theme, panels) | VyroIDE base | Adopt; add `.vy` grammar + LSP |
| VyroCoding `apps/api` execution queue (BullMQ/Redis) | VyroCloud exec | Adopt; swap Judge0 → VyroVM sandbox runner |
| VyroCoding `apps/collab` (WebSocket) + Redis Pub/Sub | Realtime/Scalability | Adopt directly |
| VyroCoding `/ai/*` SSE gateway | VyroAI base | Adopt; ground in compiler symbol table |
| VyroCoding JWT auth, rate-limit, Docker Compose, CI | Platform baseline | Adopt |
| VyroOs (microkernel, drivers, net/TLS stack, FAT32) | VyroOS — real OS | Reference; keep capability layer as sandbox contract |
| VyroOs Path A/B installable images | Native deploy target (future) | Optional: run platform on VyroOS Core |

> Rule: **reuse before rebuild.** Each reuse item gets a migration note in its PR (see [CONTRIBUTING](../../CONTRIBUTING.md)). The only true greenfield work is VyroLang + Compiler + VM + vpm.

## External Dependencies

| Dependency | Used by | Notes |
|---|---|---|
| Docker | Sandbox, deploy | isolation + packaging |
| PostgreSQL | Cloud, Registry | durable state |
| Redis | Cloud, services | cache/queue/sessions |
| Nginx | Edge | reverse proxy |
| Cloudflare | Edge | DNS/TLS/WAF |
| Monaco | IDE | editor |
| Model provider | VyroAI | completion/review |
