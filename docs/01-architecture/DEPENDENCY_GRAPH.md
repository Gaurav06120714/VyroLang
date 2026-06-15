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

| Existing asset | Reused as | Action |
|---|---|---|
| VyroCoding: language experiments | Basis for VyroLang grammar | Consolidate into spec |
| VyroCoding: compiler prototypes | Compiler frontend foundation | Refactor into staged pipeline |
| VyroCoding: runtime code | VyroVM seed | Extract opcodes/heap |
| VyroOs: kernel/system research | VyroOS capability layer | Wrap as host-abstraction API |

> Rule: **reuse before rebuild.** Each reuse item gets a migration note in its PR (see [CONTRIBUTING](../../CONTRIBUTING.md)).

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
