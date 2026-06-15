# VPS Deployment

## Goal

Run the Vyro Ecosystem on a hardened Ubuntu 24.04 VPS, with a clear path from single-host to scaled-out.

## Target Stack

| Component | Role |
|---|---|
| Ubuntu 24.04 LTS | base OS |
| Docker + Compose | run all services + sandboxes |
| Nginx | reverse proxy, TLS, routing, static |
| PostgreSQL | durable data |
| Redis | cache, queues, sessions |
| Cloudflare | DNS, TLS, WAF, DDoS |

## Topology

```
Internet
   ↓
Cloudflare (DNS/TLS/WAF/rate-limit)
   ↓
Nginx (reverse proxy, TLS)
   ↓
Vyro Services (IDE backend · Compiler API · Cloud orchestrator · AI gateway · Registry)
   ↓
Runner pool (Docker sandboxes)
   ↓
PostgreSQL + Redis
```

## Docker Compose (sketch)

```yaml
services:
  nginx:        { image: nginx, ports: ["80:80","443:443"] }
  ide-backend:  { build: ./services/ide }
  compiler-api: { build: ./services/compiler }
  orchestrator: { build: ./services/cloud }
  ai-gateway:   { build: ./services/ai }
  registry:     { build: ./services/registry }
  runner:       { build: ./services/runner, deploy: { replicas: 2 } }
  postgres:     { image: postgres:16, volumes: ["pgdata:/var/lib/postgresql/data"] }
  redis:        { image: redis:7 }
volumes: { pgdata: {} }
```

> Runners get extra Docker/seccomp constraints; see [Security Model](../11-security/SECURITY_MODEL.md).

## Provisioning Steps

1. Create VPS (Ubuntu 24.04); create non-root sudo user.
2. Harden SSH: key-only, no root, no password (see Security Model).
3. Configure firewall (allow 80/443; SSH from allowlist).
4. Install Docker + Compose; enable Fail2Ban.
5. Point DNS to Cloudflare; enable proxy + WAF.
6. Issue TLS (Cloudflare origin cert) and configure Nginx.
7. Set secrets in the environment / secret store.
8. `docker compose up -d`; run DB migrations.
9. Smoke test: compile → run (sandbox) → deploy.

## Nginx Responsibilities

- TLS termination (origin) + HSTS.
- Route `/api`, `/ws`, app subdomains.
- Rate limits + security headers (CSP).
- Serve IDE static assets.

## Backups & DR

- Nightly PostgreSQL dumps to object storage; tested restores.
- Redis is cache/queue (rebuildable); persist only if needed.
- Infrastructure as code so the host is reproducible.

## Observability

- Centralized logs, metrics (latency, queue depth, runner utilization), and alerts.

## Scaling Path

Single host → split runner pool + managed DB → autoscaling runners + object storage (see [Scalability](../01-architecture/SCALABILITY.md)).

## Estimated Development Time

Provisioning + Compose + Nginx + TLS: ~1 week. ([v0.9.0](../../versions/v0.9.0-deploy.md))

## Risk Analysis

| Risk | Mitigation |
|---|---|
| Single point of failure | backups, IaC, scale-out path |
| Misconfig exposure | hardening checklist, scans |
| Cert/TLS issues | Cloudflare + automated renewal |
