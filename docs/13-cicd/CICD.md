# CI/CD Pipeline

## Goal

Automate build, test, lint, security scan, and deployment using GitHub Actions, with safe, automatic delivery to the VPS.

## Pipeline Stages

```
Push / PR
   ↓
Build  →  Test  →  Lint  →  Security Scan  →  (main only) Deploy
```

## Flow to Production

```
GitHub  →  Actions  →  build & push image  →  VPS  →  Docker (compose up)
```

## Workflows

### CI (on PR + push)
- **Build** — compile Rust crates (compiler/VM/vpm) and TS services.
- **Test** — unit + integration + golden tests; upload coverage.
- **Lint** — `clippy` + `rustfmt --check`; `eslint` + `prettier --check`.
- **Security scan** — dependency audit (`cargo audit`, `npm audit`), secret scan, SAST.

### CD (on push to `main`, after CI passes)
- Build container images, tag with commit SHA + semver.
- Push to registry.
- SSH/deploy to VPS; `docker compose pull && up -d`; run migrations.
- Smoke test; auto-rollback on failure.

## Example (`.github/workflows/ci.yml` sketch)

```yaml
name: CI
on: [push, pull_request]
jobs:
  build-test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo build --workspace
      - run: cargo test --workspace
      - run: cargo clippy -- -D warnings
      - run: cargo fmt --check
      - run: cargo audit
```

## Branch Protection

- `main` protected: PR required, CI green required, no force-push.
- Conventional Commits enforced (see [CONTRIBUTING](../../CONTRIBUTING.md)).

## Secrets

- Stored in GitHub Actions secrets / environments.
- Deploy uses a scoped SSH key; never logged.

## Release Automation

- Tag `vX.Y.Z` → release workflow builds artifacts, updates [CHANGELOG](../../CHANGELOG.md), publishes GitHub Release.

## Testing the Pipeline

- Dry-run deploys to a staging compose stack.
- Rollback drills.

## Risk Analysis

| Risk | Mitigation |
|---|---|
| Bad deploy to prod | smoke test + auto-rollback; staging first |
| Leaked secrets in logs | masked secrets; scanning |
| Flaky tests blocking | quarantine + fix policy |
