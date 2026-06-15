# Scalability Strategy

## Goal

Start on a single VPS, but design so that growth is a configuration change, not a rewrite.

## Scaling Dimensions

| Dimension | Bottleneck | Strategy |
|---|---|---|
| Compile/run throughput | CPU on the VPS | Stateless job runners behind a queue; add runner nodes horizontally |
| Concurrent users | WebSocket connections | Separate IDE/realtime tier; sticky sessions via Redis |
| Storage | Project files & artifacts | Object storage (S3-compatible) instead of local FS |
| Database | Postgres connections | Connection pooling (PgBouncer); read replicas |
| Cache/queue | Redis memory | Dedicated Redis; partition queues by job type |

## Execution as a Job Queue

```
IDE → Compiler API → enqueue(job) → Redis queue → Runner pool (N nodes)
                                                       │ Docker sandbox
                                                       ▼
                                              result → store → notify IDE
```

- Runners are **stateless and disposable**; scale by count.
- Each job has hard CPU/RAM/time limits (see [Security Model](../11-security/SECURITY_MODEL.md)).
- Backpressure: queue depth drives autoscaling and user-facing wait estimates.

## Stages of Growth

1. **Single VPS** — Docker Compose: all services + Postgres + Redis. Good to first users.
2. **Split tiers** — separate runner pool from web/services; managed Postgres + Redis.
3. **Horizontal runners** — autoscaling runner nodes; object storage for artifacts.
4. **Multi-region** — Cloudflare routing; regional runner pools; replicated read paths.

## Stateless-First

All services aside from the databases are stateless. Session and rate-limit state live in Redis; durable data in Postgres; large blobs in object storage. This makes any service horizontally scalable.

## Observability (enables scaling decisions)

- Metrics: request latency, queue depth, runner utilization, sandbox kill rate.
- Logs: structured, centralized.
- Tracing: compile→run spans across services.
- Alerts: queue saturation, error-rate spikes, sandbox limit breaches.

## Cost Controls

- Per-user execution quotas and rate limits.
- Idle workspace hibernation.
- Tiered limits for free vs. paid Cloud usage.
