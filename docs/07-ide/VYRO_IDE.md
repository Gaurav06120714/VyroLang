# VyroIDE — Browser IDE

## Goal

A fast, browser-based IDE where developers write, compile, run, debug, and deploy VyroLang — the VS Code of the ecosystem.

## Technology

- **Next.js + React + TypeScript** — app shell and routing.
- **Monaco Editor** — code editing (VS Code's editor core).
- **Tailwind CSS** — styling.
- **xterm.js** — integrated terminal.
- **WebSocket** — live run output and debugging.

## Features

| Feature | Notes |
|---|---|
| Syntax highlighting | Monaco TextMate grammar for `.vy` |
| Autocomplete | language server (LSP) backed by the compiler |
| Terminal | xterm.js wired to the Cloud runner |
| Debugger | breakpoints, step, variable inspection via VM debug protocol |
| File explorer | workspace tree, create/rename/delete |
| Git integration | clone, commit, push, diff |
| Theme support | light/dark + custom themes |

## Layout

```
┌───────────────────────────────────────────────┐
│ Top bar:  Run ▸  Debug  Deploy   ⚙   theme     │
├──────────┬────────────────────────┬───────────┤
│ File     │ Monaco editor          │ VyroAI    │
│ Explorer │ (tabs)                 │ panel     │
│          │                        │           │
├──────────┴────────────────────────┴───────────┤
│ Terminal / Output / Problems / Debug console   │
└───────────────────────────────────────────────┘
```

## Core Workflow

```
Write → Compile (Compiler API) → Run (Cloud sandbox) → Debug → Deploy (VyroDeploy)
```

1. Edit `.vy` files in Monaco.
2. **Compile**: POST source to Compiler API → diagnostics inline.
3. **Run**: stream stdout/stderr over WebSocket from the sandbox.
4. **Debug**: attach to VM debug protocol; set breakpoints.
5. **Deploy**: one click to VyroDeploy.

## Language Server (LSP)

A compiler-backed LSP provides: completions, hovers, go-to-definition, diagnostics, and rename — reusing the compiler's lexer/parser/sema.

## API Surface (IDE backend)

| Endpoint | Purpose |
|---|---|
| `POST /compile` | source → diagnostics + artifact id |
| `WS /run/:id` | stream execution I/O |
| `GET /workspace/:id/files` | file tree |
| `PUT /workspace/:id/file` | save file |
| `POST /git/*` | git operations |

## Security Considerations

- All code execution goes to the sandbox, never the IDE host.
- Workspace isolation per user; signed URLs for artifacts.
- Auth on every backend call; CSRF protection; CSP headers.

## Testing Strategy

- Component tests (editor, explorer, terminal).
- E2E (Playwright): write → run → see output → deploy.
- LSP conformance tests against the compiler.

## Estimated Development Time

Editor + run loop + explorer: ~3 weeks. Debugger + git + LSP: ~3–4 weeks. ([v0.6.0](../../versions/v0.6.0-ide.md))

## Future Improvements

- Real-time collaboration (CRDT), extensions/plugins, remote workspaces, mobile layout.

## Risk Analysis

| Risk | Mitigation |
|---|---|
| Editor perf on big files | virtualize; lazy-load |
| LSP latency | incremental compile; cache |
| WebSocket scaling | dedicated realtime tier (see Scalability) |
