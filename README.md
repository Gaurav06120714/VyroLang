# Vyro Ecosystem

> A fully independent software platform where developers **write, compile, execute, debug, package, and deploy** applications entirely within one ecosystem — secure, scalable, and production-grade.

[![Status](https://img.shields.io/badge/status-planning-blue)](./ROADMAP.md)
[![License](https://img.shields.io/badge/license-MIT-green)](./LICENSE)
[![Phase](https://img.shields.io/badge/phase-1%20architecture-orange)](./docs/01-architecture/SYSTEM_ARCHITECTURE.md)

---

## What is Vyro?

Vyro is a ground-up software ecosystem — its own language, compiler, virtual machine, OS layer, package manager, IDE, cloud runtime, deploy platform, and AI assistant — designed to interoperate as a single, coherent platform.

This repository (`VyroLang`) is the **planning and specification hub** for the entire ecosystem. It contains the architecture, language spec, component designs, version plans, roadmaps, and operating rules. Implementation lands in companion repos.

## The Components

| Component | Role | Inspired by |
|---|---|---|
| **VyroLang** | Modern, readable, statically-checked language | Python + Rust |
| **VyroCompiler** | Lexer → Parser → Semantic → Optimizer → Codegen | GCC + Rustc |
| **VyroVM** | Stack/heap VM, GC, scheduler, async runtime | JVM |
| **VyroPackageManager (vpm)** | Dependency resolution, registry, lockfiles | Cargo |
| **VyroOS** | System abstraction layer (FS/Process/Memory/Net) | Linux |
| **VyroIDE** | Browser IDE: editor, terminal, debugger, git | VS Code |
| **VyroCloud** | Run, store, and share projects in the cloud | Replit |
| **VyroDeploy** | Push-to-deploy application hosting | Vercel |
| **VyroAI** | Completion, review, bug detection, docs | GitHub Copilot |

## Architecture at a Glance

```
Browser
  ↓
VyroIDE  ──────────────►  VyroAI (assist)
  ↓
Compiler API
  ↓
VyroCompiler ──► Bytecode
  ↓
VyroVM
  ↓
VyroOS Layer
  ↓
Docker Sandbox  ──►  Linux VPS  ──►  PostgreSQL / Redis
```

Full diagrams: [System Architecture](./docs/01-architecture/SYSTEM_ARCHITECTURE.md).

## Repository Map

| Repo | Purpose |
|---|---|
| [VyroLang](https://github.com/Gaurav06120714/VyroLang) | Ecosystem planning, language spec, compiler/VM design (this repo) |
| [VyroCoding](https://github.com/Gaurav06120714/VyroCoding) | Language/compiler/runtime experimentation |
| [VyroOs](https://github.com/Gaurav06120714/VyroOs) | OS architecture, kernel research, system abstractions |

## Documentation Index

- **Overview** — [Ecosystem](./docs/00-overview/ECOSYSTEM.md) · [Vision](./docs/00-overview/VISION.md) · [Glossary](./docs/00-overview/GLOSSARY.md) · [Existing Repo Analysis](./docs/00-overview/EXISTING_REPO_ANALYSIS.md)
- **Architecture** — [Standalone Stack](./docs/01-architecture/STANDALONE_STACK.md) · [System](./docs/01-architecture/SYSTEM_ARCHITECTURE.md) · [Components](./docs/01-architecture/COMPONENT_MAP.md) · [Dependencies](./docs/01-architecture/DEPENDENCY_GRAPH.md) · [Scalability](./docs/01-architecture/SCALABILITY.md)
- **Language** — [VyroLang Spec](./docs/02-vyrolang/LANGUAGE_SPEC.md)
- **Compiler** — [Compiler Design](./docs/03-compiler/COMPILER_DESIGN.md)
- **VM** — [VM Design](./docs/04-vm/VM_DESIGN.md)
- **OS Layer** — [VyroOS](./docs/05-vyroos/OS_LAYER.md)
- **Package Manager** — [vpm](./docs/06-package-manager/VPM.md)
- **IDE** — [VyroIDE](./docs/07-ide/VYRO_IDE.md)
- **AI** — [VyroAI](./docs/08-ai/VYRO_AI.md)
- **Applications** — [Reference Apps](./docs/09-applications/REFERENCE_APPS.md)
- **Cloud** — [VyroCloud](./docs/10-cloud/VYRO_CLOUD.md) · [VyroCoding Integration](./docs/10-cloud/VYROCODING_INTEGRATION.md)
- **Security** — [Security Model](./docs/11-security/SECURITY_MODEL.md)
- **Deployment** — [VPS Deployment](./docs/12-deployment/VPS_DEPLOYMENT.md)
- **CI/CD** — [Pipeline](./docs/13-cicd/CICD.md)
- **Docs Plan** — [Documentation Plan](./docs/14-documentation/DOCS_PLAN.md)

## Roadmaps

[30-Day](./planning/ROADMAP_30_DAY.md) · [90-Day](./planning/ROADMAP_90_DAY.md) · [180-Day](./planning/ROADMAP_180_DAY.md) · [365-Day](./planning/ROADMAP_365_DAY.md) · [Master Roadmap](./ROADMAP.md)

## Versioning Plan

Release plans live in [`/versions`](./versions). See [CHANGELOG](./CHANGELOG.md) for the running history.

## Project Rules

Engineering rules and conventions: [CONTRIBUTING](./CONTRIBUTING.md) · [Security Policy](./SECURITY.md) · [Code of Conduct](./CODE_OF_CONDUCT.md).

## Status

**Phase 2 — Self-contained stack working.** The architecture diagram is realized end-to-end in [`impl/`](./impl): your own **VyroIDE → Compiler API → VyroCompiler → Bytecode → VyroVM**, with a sandboxed runtime and a Docker sandbox image. Run the whole thing locally:

```bash
cd impl && cargo run --release -- serve 8787   # open http://localhost:8787
```

See the [Standalone Stack](./docs/01-architecture/STANDALONE_STACK.md) doc (box → code mapping) and the [implementation README](./impl/README.md). Milestones tracked in [ROADMAP.md](./ROADMAP.md).

---

© Gaurav. Released under the [MIT License](./LICENSE).
