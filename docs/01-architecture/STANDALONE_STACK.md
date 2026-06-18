# The Standalone Vyro Stack

The architecture diagram, realized **entirely inside this repo** — your own language, compiler, VM, Compiler API, IDE, and sandbox. No dependency on VyroCoding or any other platform.

```
Browser
   │
   ▼
VyroIDE            impl/web/index.html        (your browser editor + console)
   │  POST /api/run {code, stdin}
   ▼
Compiler API       impl/src/server.rs         (zero-dep HTTP server)
   │
   ▼
VyroCompiler       impl/src/{lexer,parser,compiler}.rs
   │
   ▼
Bytecode           impl/src/opcode.rs
   │
   ▼
VyroVM             impl/src/vm.rs             (stack VM, GC-free heap, call frames)
   │  sandbox layer: instruction budget + wall-clock deadline + captured I/O
   ▼
Docker Sandbox     impl/Dockerfile + scripts/sandbox-run.sh
   │  CPU / RAM / PID limits · read-only fs · cap-drop · non-root
   ▼
Linux host / VPS
```

## Box → code mapping

| Diagram box | Implemented by | Status |
|---|---|---|
| Browser → **VyroIDE** | `impl/web/index.html` (editor, examples, stdin, console; Ctrl+Enter to run) | Built |
| **Compiler API** | `impl/src/server.rs` (`vyro serve`) — `/`, `/health`, `/api/run`, `/api/compile` | Built |
| **VyroCompiler** | `impl/src/lexer.rs`, `parser.rs`, `compiler.rs` | Built |
| **Bytecode** | `impl/src/opcode.rs` | Built |
| **VyroVM** | `impl/src/vm.rs` | Built |
| **VyroOS layer** (resource/capability guard) | VM instruction budget + wall-clock deadline + captured stdin/stdout (no host stdio in server mode) | Built (in-process); capability FS/net gate is future |
| **Docker Sandbox** | `impl/Dockerfile`, `impl/scripts/sandbox-run.sh` | Built |
| **Linux VPS / PostgreSQL / Redis** | deploy target | Planned ([VPS Deployment](../12-deployment/VPS_DEPLOYMENT.md)) |
| **VyroAI (assist)** | optional, separate | Planned ([VyroAI](../08-ai/VYRO_AI.md)) |

## Run it

```bash
cd impl
cargo run --release -- serve 8787      # then open http://localhost:8787
```

Sandboxed:

```bash
cd impl && ./scripts/sandbox-run.sh    # Docker, with CPU/RAM/PID limits
```

## Safety model (today)

The Compiler API runs untrusted source, so the VM enforces, per request:

- **Instruction budget** (50M ops) — stops infinite loops / runaway recursion.
- **Wall-clock deadline** (5s) — stops slow programs.
- **Captured I/O** — `print` is buffered (never touches the server's stdout); `input()` reads only the request's `stdin`, never the host's.
- No filesystem or network access exists in the language yet, so programs can only compute.

The Docker layer adds CPU/RAM/PID caps, a read-only filesystem, dropped capabilities, and a non-root user. The remaining hardening (per-request container isolation, seccomp, network-deny) is tracked in the [Security Model](../11-security/SECURITY_MODEL.md).
