# Security Model

Security is mandatory and pervasive. This document defines the threat model, the sandbox, and infrastructure hardening.

## Threat Model

| Asset | Threat | Adversary |
|---|---|---|
| Host VPS | Sandbox escape, RCE | malicious user code |
| Other users' data | Cross-tenant access | malicious/curious user |
| Service availability | DoS, resource exhaustion | abusive user / bot |
| Secrets | Leakage | code exfiltration, logs |
| Supply chain | Malicious packages | registry attacker |

## Defense-in-Depth Overview

```
Cloudflare (WAF, DDoS, rate limit)
   ↓
Nginx (TLS, routing, limits)
   ↓
Auth + rate limiting (services)
   ↓
Docker Sandbox (the hard boundary)
   ↓
VM capability gate (VyroOS)
```

## 1. Execution Sandbox (the critical control)

Every code execution runs inside a Docker container with:

- **CPU limit** (cgroup quota).
- **RAM limit** (hard cap → OOM-kill inside container, never host).
- **Execution time limit** (wall-clock; killed on breach).
- **Filesystem isolation** — read-only base image + small writable scratch; workspace jailed; no host mounts.
- **Default-deny networking** — no egress unless `net.http` granted (then allowlisted + rate-limited).
- **Dropped capabilities** — `--cap-drop=ALL`, non-root user, `no-new-privileges`.
- **Seccomp profile** — restrict syscalls to the minimum needed.
- **PIDs limit** — prevent fork bombs.
- Ephemeral: container destroyed after each run.

The VM additionally enforces an instruction/time budget and routes all FS/Net through the [VyroOS capability gate](../05-vyroos/OS_LAYER.md), so even a VM bug has a second wall.

## 2. SSH & VPS Hardening (mandatory)

- **Key authentication only** — `PasswordAuthentication no`.
- **Disable root login** — `PermitRootLogin no`.
- **Disable password login** entirely.
- Non-standard SSH port (optional) + connection rate limiting.

## 3. Network Protection

- **Fail2Ban** — ban brute-force sources.
- **Firewall (ufw/nftables)** — allow only 80/443 (+ SSH from allowlist).
- **Cloudflare** — DDoS, WAF, bot management in front of origin.
- **Rate limiting** — per IP and per account, at edge and app.
- **Audit logs** — auth events, deploys, admin actions; centralized + tamper-evident.

## 4. Application Security

- Auth on every endpoint; short-lived tokens; CSRF protection.
- CSP, HSTS, secure cookies, security headers via Nginx.
- Input validation everywhere; parameterized SQL.
- Secrets in a secret store / env, never in source (`.env` git-ignored).
- Dependency scanning + `vpm audit` in CI.

## 5. Supply Chain

- Package tarball hash verification.
- Scoped, revocable publish tokens.
- Yank compromised versions; namespace ownership.

## Security Testing

| Test | Goal |
|---|---|
| Sandbox escape suite | escapes must fail |
| Fork-bomb / OOM / infinite-loop | limits hold; host unaffected |
| Network egress test | denied by default |
| Path traversal | jail holds |
| Pen-test checklist | before each public release |

## Incident Response

1. Detect (alerts on anomalies) → 2. Contain (kill jobs, revoke tokens) → 3. Eradicate (patch) → 4. Recover → 5. Post-mortem.

## Risk Analysis

| Risk | Severity | Mitigation |
|---|---|---|
| Container escape | Critical | seccomp, cap-drop, non-root, no-net, audits |
| Cross-tenant leak | High | per-workspace isolation, scoped tokens |
| DoS | High | edge + app rate limits, quotas, autoscale |
| Secret leakage | High | secret store, redaction, log scrubbing |
| Malicious package | High | hash verify, audit, yank |
