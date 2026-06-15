# Vyro Ecosystem — Master Roadmap

This is the top-level roadmap. Detailed time-boxed plans live in [`/planning`](./planning) and per-release plans in [`/versions`](./versions).

## Strategy

Build bottom-up: a working **language → compiler → VM** core first, then the developer surface (IDE, package manager), then the cloud/deploy/AI layers. Every layer is shippable and tested before the next begins.

## Phase Overview

| Phase | Focus | Output | Doc |
|---|---|---|---|
| 1 | Architecture & Spec | This repo | [System Architecture](./docs/01-architecture/SYSTEM_ARCHITECTURE.md) |
| 2 | VyroLang | Language spec + grammar | [Language Spec](./docs/02-vyrolang/LANGUAGE_SPEC.md) |
| 3 | Compiler | Lexer→Parser→Sema→Opt→Codegen | [Compiler Design](./docs/03-compiler/COMPILER_DESIGN.md) |
| 4 | VyroVM | Stack VM, GC, scheduler | [VM Design](./docs/04-vm/VM_DESIGN.md) |
| 5 | VyroOS | FS/Process/Memory/Net APIs | [OS Layer](./docs/05-vyroos/OS_LAYER.md) |
| 6 | vpm | Package manager + registry | [vpm](./docs/06-package-manager/VPM.md) |
| 7 | VyroIDE | Browser IDE | [VyroIDE](./docs/07-ide/VYRO_IDE.md) |
| 8 | VyroAI | AI assistant | [VyroAI](./docs/08-ai/VYRO_AI.md) |
| 9 | Reference Apps | Todo, Calculator, Tic-Tac-Toe | [Reference Apps](./docs/09-applications/REFERENCE_APPS.md) |
| 10 | VyroCloud | Cloud execution | [VyroCloud](./docs/10-cloud/VYRO_CLOUD.md) |
| 11 | Security | Sandbox + hardening | [Security Model](./docs/11-security/SECURITY_MODEL.md) |
| 12 | Deployment | Ubuntu VPS stack | [VPS Deployment](./docs/12-deployment/VPS_DEPLOYMENT.md) |
| 13 | CI/CD | GitHub Actions → VPS | [CI/CD](./docs/13-cicd/CICD.md) |
| 14 | Documentation | User + dev docs | [Docs Plan](./docs/14-documentation/DOCS_PLAN.md) |
| 15 | Roadmap | Time-boxed plans | [/planning](./planning) |

## Time-Boxed Plans

| Horizon | Theme | Plan |
|---|---|---|
| 30 days | Language, Compiler, VM | [ROADMAP_30_DAY](./planning/ROADMAP_30_DAY.md) |
| 90 days | IDE, Apps, Cloud Runtime | [ROADMAP_90_DAY](./planning/ROADMAP_90_DAY.md) |
| 180 days | Package Manager, Deploy, AI | [ROADMAP_180_DAY](./planning/ROADMAP_180_DAY.md) |
| 365 days | Production, Community, OSS | [ROADMAP_365_DAY](./planning/ROADMAP_365_DAY.md) |

## Release Milestones

| Version | Theme | Plan |
|---|---|---|
| v0.1.0 | Foundation (spec + skeleton) | [v0.1.0](./versions/v0.1.0-foundation.md) |
| v0.2.0 | Compiler frontend | [v0.2.0](./versions/v0.2.0-compiler-frontend.md) |
| v0.3.0 | VM + bytecode execution | [v0.3.0](./versions/v0.3.0-vm-runtime.md) |
| v0.4.0 | Optimizer + stdlib | [v0.4.0](./versions/v0.4.0-optimizer-stdlib.md) |
| v0.5.0 | vpm + OS layer | [v0.5.0](./versions/v0.5.0-vpm-os.md) |
| v0.6.0 | VyroIDE alpha | [v0.6.0](./versions/v0.6.0-ide.md) |
| v0.7.0 | VyroCloud + sandbox | [v0.7.0](./versions/v0.7.0-cloud.md) |
| v0.8.0 | VyroAI assistant | [v0.8.0](./versions/v0.8.0-ai.md) |
| v0.9.0 | VyroDeploy + hardening | [v0.9.0](./versions/v0.9.0-deploy.md) |
| v1.0.0 | Production release | [v1.0.0](./versions/v1.0.0-production.md) |
