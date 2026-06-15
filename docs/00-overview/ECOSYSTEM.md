# The Vyro Ecosystem

Vyro is an end-to-end software platform. Unlike a single tool, it is a vertically integrated stack: the language, the thing that compiles it, the thing that runs it, the OS surface it talks to, the place it's stored and executed, and the assistant that helps you write it — all designed together.

## The Promise

A developer opens a browser, writes VyroLang, hits Run, and watches it compile to bytecode, execute on the VyroVM inside a secure sandbox, and — when ready — deploy to a public URL. No local toolchain required.

## Components and Responsibilities

### VyroLang
A modern language with familiar, low-ceremony syntax (Python-like ergonomics) and predictable semantics with optional static typing (Rust-influenced rigor). Supports variables, rich data types, control flow, functions, classes, async, error handling, and modules.

### VyroCompiler
Transforms source into VM bytecode through a classic pipeline: **lexer → parser → semantic analyzer → optimizer → code generator**. Reports precise diagnostics.

### VyroVM
A stack-based virtual machine executing Vyro bytecode. Provides a heap with garbage collection, a scheduler, a memory manager, and concurrency primitives (threads, coroutines, async runtime).

### VyroOS (Layer)
A capability-based abstraction over the host: File System, Process, Memory, and Network APIs. Long-term: kernel research, drivers, shell, package loader, system services.

### VyroPackageManager (vpm)
Cargo-style dependency management: `install`, `publish`, `update`, `remove`, with dependency resolution, version locking, and a package registry.

### VyroIDE
A browser IDE built on Next.js + Monaco: syntax highlighting, autocomplete, integrated terminal, debugger, file explorer, git integration, themes.

### VyroAI
An AI assistant for completion, code review, bug detection, optimization suggestions, and documentation generation.

### VyroCloud
Cloud execution and project storage: run programs, store projects, create workspaces — each run isolated in a Docker sandbox.

### VyroDeploy
Push-to-deploy hosting for finished apps, fronted by Cloudflare and Nginx.

## How They Fit Together

See the data and control flows in [System Architecture](../01-architecture/SYSTEM_ARCHITECTURE.md) and the [Component Map](../01-architecture/COMPONENT_MAP.md).

## Relationship to Existing Repos

This ecosystem **evolves** the existing `VyroCoding` (language/compiler/runtime experiments) and `VyroOs` (OS research) repositories rather than replacing them. Existing code is the foundation; see migration guidance in [CONTRIBUTING](../../CONTRIBUTING.md) and the dependency mapping in [Dependency Graph](../01-architecture/DEPENDENCY_GRAPH.md).
