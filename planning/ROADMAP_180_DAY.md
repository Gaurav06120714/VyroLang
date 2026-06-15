# 180-Day Roadmap — Package Manager · Deployment · AI

**Theme:** Turn the toolchain into a platform: packages, push-to-deploy, and AI assistance.

**Target releases:** [v0.5.0](../versions/v0.5.0-vpm-os.md) · [v0.8.0](../versions/v0.8.0-ai.md) → [v0.9.0](../versions/v0.9.0-deploy.md)

## Months 4 — Package Manager + OS Layer
- [ ] `vpm` CLI: init/install/update/remove/publish/search.
- [ ] SemVer resolver + lockfile (`vyro.lock`).
- [ ] Registry service (Postgres + object storage) with auth.
- [ ] VyroOS capability layer: FS/Process/Memory/Net + gate.

## Month 5 — VyroAI
- [ ] AI gateway (provider-abstracted; default latest Claude model).
- [ ] Code completion (streaming) grounded in symbol table.
- [ ] Code review, explain, fix-diagnostic, docs generation.
- [ ] Guardrails: consent, redaction, quotas.

## Month 6 — VyroDeploy + Hardening
- [ ] Push-to-deploy: build → image → Nginx route → public URL.
- [ ] Rollbacks + env secret store.
- [ ] VPS hardening: SSH key-only, Fail2Ban, firewall, Cloudflare.
- [ ] CI/CD: build/test/lint/scan → auto-deploy with rollback.
- [ ] Full security pass + pen-test checklist.

## Exit Criteria
- Publish a package and depend on it from another project.
- Deploy a reference app to a live URL.
- VyroAI assists inside the IDE. Tag **v0.9.0**.

## Risks
| Risk | Mitigation |
|---|---|
| Supply-chain | hash verify, scoped tokens, audit, yank |
| AI cost/privacy | quotas, redaction, no-train policy |
| Deploy safety | staging + smoke test + auto-rollback |
