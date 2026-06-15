# 365-Day Roadmap — Production · Community · Open Source

**Theme:** Reach a stable, documented, production-grade v1.0 and open the ecosystem.

**Target release:** [v1.0.0](../versions/v1.0.0-production.md)

## Quarter 3 — Stabilization
- [ ] Performance: VM JIT for hot paths; generational GC.
- [ ] Incremental compilation; faster LSP.
- [ ] Hardening + load testing the runner pool at scale.
- [ ] Observability: metrics, tracing, alerting across services.

## Quarter 4 — Production & Ecosystem
- [ ] Freeze stable language spec (1.0) + compatibility guarantees.
- [ ] Docs site (generated from `docs/`); full tutorials + API refs.
- [ ] Package registry GA; seed core libraries.
- [ ] Scale-out: split tiers, managed DB, object storage, multi-region path.
- [ ] Open-source the core; publish governance + roadmap.
- [ ] Launch: marketing site, examples gallery, community channels.

## Stretch
- [ ] Pattern matching, generics, traits in the language.
- [ ] Real-time IDE collaboration.
- [ ] Self-hosting toolchain components in VyroLang where practical.

## Exit Criteria — v1.0.0
- Stable spec + semver compatibility commitment.
- All components production-deployed, monitored, hardened.
- Complete user + developer docs.
- Public registry + open-source core.

## Risks
| Risk | Mitigation |
|---|---|
| Stability vs. features | freeze 1.0 spec; defer extras to 1.x |
| Scaling cost | quotas, hibernation, tiered limits |
| Community load (single maintainer) | clear governance; staged opening |
