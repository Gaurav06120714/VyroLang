# VyroCloud — Cloud Execution

## Goal

Let users run programs, store projects, create workspaces, and deploy apps from the browser — every execution isolated in a Docker sandbox. The Replit of the ecosystem.

## Capabilities

- Run VyroLang programs in the cloud.
- Store and version projects.
- Create per-user workspaces.
- Hand off finished apps to [VyroDeploy](#vyrodeploy).

## Architecture

```
Browser
  ↓
VyroIDE
  ↓
Compiler Service ──► artifact (.vbc)
  ↓
Orchestrator ──► enqueue run job (Redis)
  ↓
Runner pool ──► Docker Sandbox (VM + VyroOS, limits)
  ↓
Execution Result ──► stream to IDE (WebSocket) + persist
```

## Execution Lifecycle

1. IDE submits source → Compiler Service returns artifact + diagnostics.
2. Orchestrator enqueues a run job with resource limits.
3. A stateless runner pulls the job, launches a Docker sandbox, runs the VM.
4. stdout/stderr stream back over WebSocket; stdin forwarded.
5. On exit (or limit breach), the sandbox is destroyed and results stored.

## Workspaces

- Each workspace = isolated project (files + metadata + history).
- Persistent storage in object storage; metadata in PostgreSQL.
- Hibernate idle workspaces to save cost (see [Scalability](../01-architecture/SCALABILITY.md)).

## Database Schema (cloud)

```sql
CREATE TABLE workspaces (
  id UUID PRIMARY KEY,
  owner_id INT NOT NULL,
  name TEXT NOT NULL,
  created_at TIMESTAMPTZ DEFAULT now(),
  last_active TIMESTAMPTZ
);

CREATE TABLE runs (
  id UUID PRIMARY KEY,
  workspace_id UUID REFERENCES workspaces(id),
  status TEXT,            -- queued|running|done|killed|error
  exit_code INT,
  started_at TIMESTAMPTZ,
  finished_at TIMESTAMPTZ
);
```

## API Surface

| Endpoint | Purpose |
|---|---|
| `POST /workspaces` | create workspace |
| `POST /runs` | start a run (returns run id) |
| `WS /runs/:id/io` | stream I/O |
| `GET /runs/:id` | run status/result |

## Sandbox (critical)

Every run executes inside Docker with: CPU limit, RAM limit, wall-clock time limit, filesystem isolation (read-only base + scratch), and **default-deny networking**. Details in [Security Model](../11-security/SECURITY_MODEL.md).

## <a id="vyrodeploy"></a>VyroDeploy

Push-to-deploy hosting for finished apps:

```
Project → Build → Container image → Deploy → Nginx route → Cloudflare → Public URL
```

- Per-app subdomain; TLS via Cloudflare.
- Rollbacks to previous deploys.
- Environment variables via secret store.

## Testing Strategy

- Sandbox escape tests (must fail).
- Load tests on the runner pool / queue.
- E2E: create workspace → run → deploy → fetch public URL.

## Estimated Development Time

Orchestrator + runner + sandbox: ~3–4 weeks. Deploy: ~2 weeks. ([v0.7.0](../../versions/v0.7.0-cloud.md), [v0.9.0](../../versions/v0.9.0-deploy.md))

## Future Improvements

- Persistent databases per app, cron jobs, custom domains, team workspaces, usage analytics.

## Risk Analysis

| Risk | Mitigation |
|---|---|
| Sandbox escape | defense-in-depth; audits; least privilege |
| Resource exhaustion | hard limits; quotas; autoscale |
| Cost blowup | hibernation; tiered quotas |
