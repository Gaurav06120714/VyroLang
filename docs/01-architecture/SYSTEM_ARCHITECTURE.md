# System Architecture

## Goal

Define the end-to-end structure of the Vyro Ecosystem: how a developer's keystroke becomes a running, deployed program — and where the trust boundaries sit.

## High-Level Data Flow

```
 Developer (Browser)
        │
        ▼
   ┌──────────┐        ┌─────────┐
   │ VyroIDE  │◄──────►│ VyroAI  │   (completion, review, docs)
   └────┬─────┘        └─────────┘
        │ HTTPS (REST + WebSocket)
        ▼
   ┌──────────────┐
   │ Compiler API │  (gateway: auth, rate-limit, queue)
   └──────┬───────┘
          ▼
   ┌──────────────┐   emits   ┌──────────┐
   │ VyroCompiler │ ────────► │ Bytecode │ (.vbc)
   └──────┬───────┘           └────┬─────┘
          │                        ▼
          │                  ┌──────────┐
          └─────────────────►│ VyroVM   │
                             └────┬─────┘
                                  ▼
                          ┌───────────────┐
                          │ VyroOS Layer  │ (FS/Process/Mem/Net caps)
                          └──────┬────────┘
                                 ▼
                        ┌──────────────────┐
                        │  Docker Sandbox  │  (CPU/RAM/time/FS limits)
                        └────────┬─────────┘
                                 ▼
                        ┌──────────────────┐
                        │   Linux VPS      │
                        └────────┬─────────┘
                                 ▼
                     ┌────────────────────────┐
                     │ PostgreSQL  │  Redis    │
                     └────────────────────────┘
```

## Request Path (edge to core)

```
Internet → Cloudflare → Nginx → Vyro Services → Database
```

- **Cloudflare**: DNS, TLS, DDoS protection, WAF, rate limiting.
- **Nginx**: reverse proxy, TLS termination (origin), static assets, routing.
- **Vyro Services**: IDE backend, Compiler API, Cloud orchestrator, Deploy controller, AI gateway.
- **Database**: PostgreSQL (durable state), Redis (cache, queues, sessions, rate counters).

## Trust Boundaries

1. **Browser ↔ Services** — authenticated HTTPS; no execution trust.
2. **Services ↔ Sandbox** — the critical boundary. All user code crosses into a locked-down Docker container. Default-deny networking, strict resource caps.
3. **Sandbox ↔ Host** — enforced by container isolation, seccomp, dropped capabilities, read-only mounts. See [Security Model](../11-security/SECURITY_MODEL.md).

## Component Communication

| From | To | Protocol | Notes |
|---|---|---|---|
| IDE | Compiler API | HTTPS REST | submit source, fetch diagnostics/artifacts |
| IDE | Cloud (run) | WebSocket | stream stdout/stderr, send stdin |
| IDE | VyroAI | HTTPS REST/SSE | completions and reviews |
| Compiler API | Sandbox | internal RPC | dispatch compile/run jobs |
| Services | PostgreSQL | TCP (TLS) | durable data |
| Services | Redis | TCP | cache/queue/sessions |
| vpm | Registry | HTTPS | publish/fetch packages |

## Deployment Topology

A single Ubuntu 24.04 VPS hosts the stack via Docker Compose initially; the design scales out (see [Scalability](./SCALABILITY.md)) to a job-runner pool and managed databases.

## Technology Choices (summary)

| Layer | Choice | Why |
|---|---|---|
| Compiler/VM | Rust | safety, performance, great tooling |
| Services/IDE | TypeScript + Next.js | unified web stack, SSR, fast DX |
| Editor | Monaco | proven, VS Code-grade editing |
| Sandbox | Docker + seccomp | mature isolation, resource cgroups |
| DB | PostgreSQL + Redis | reliable durable + fast ephemeral |
| Edge | Cloudflare + Nginx | security + routing |
| CI/CD | GitHub Actions | native to the repos |

## Risk Analysis

| Risk | Impact | Mitigation |
|---|---|---|
| Sandbox escape | Critical | defense-in-depth: seccomp, caps drop, no-net default, time/mem caps, audits |
| Compiler complexity | Schedule | incremental phases; golden tests; reuse VyroCoding |
| Single-VPS bottleneck | Availability | early metrics; design for runner pool; managed DB path |
| Scope creep | Schedule | strict phase gates; versioned milestones |
