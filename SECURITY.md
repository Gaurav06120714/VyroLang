# Security Policy

Security is a first-class requirement of the Vyro Ecosystem. The full threat model and controls live in [docs/11-security/SECURITY_MODEL.md](./docs/11-security/SECURITY_MODEL.md). This file is the summary and reporting policy.

## Supported Versions

During pre-release (0.x), only the latest tagged version receives security fixes.

## Reporting a Vulnerability

Report privately to the maintainer (Gaurav) via a GitHub Security Advisory on this repository. Do **not** open public issues for vulnerabilities. Expect an acknowledgement within 72 hours.

## Core Controls (summary)

- **Execution sandboxing**: all untrusted code runs in Docker with CPU, RAM, time, and filesystem limits, and default-deny networking.
- **VPS hardening**: SSH key-only auth, no root login, no password login, Fail2Ban, firewall, Cloudflare in front, rate limiting, audit logs.
- **Secrets**: never in source; stored in the deployment secret store.
- **Dependencies**: lockfiles + automated vulnerability scanning in CI.
- **Least privilege**: every service runs with the minimum permissions it needs.

See the [Security Model](./docs/11-security/SECURITY_MODEL.md) for details, attack scenarios, and mitigations.
