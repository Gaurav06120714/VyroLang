# Running VyroLang in VyroCoding

VyroLang now executes **natively on the VyroVM** inside the VyroCoding platform — Judge0 is bypassed for `.vy` programs. This is the first end-to-end link between the greenfield language core and the existing platform (see [Existing Repo Analysis](../00-overview/EXISTING_REPO_ANALYSIS.md)).

## How it works

```
Browser (VyroIDE)
   │  code + languageId = Vyro (9001)
   ▼
/run · /run-all · /submit   (apps/api/src/routes/execute.routes.ts)
   │
   ▼
runner.service.ts  ──►  languageId == Vyro ?
   │                         │ yes                    │ no
   ▼                         ▼                        ▼
execution.queue (BullMQ) →  vyro.service.ts        judge0.service.ts
                              │  spawn `vyro run`     (unchanged)
                              ▼
                          VyroVM  →  stdout/stderr  →  compared to expected
```

The dispatcher [`runner.service.ts`](https://github.com/Gaurav06120714/VyroCoding/blob/main/apps/api/src/services/runner.service.ts) exposes the exact `submitAndWait` / `submitBatchAndWait` surface the worker and routes already used, so wiring VyroLang in was a one-line import swap in both the worker and the execute routes.

## Execution model

VyroCoding feeds each test case's `input` to the program's **stdin**; the program reads it with the `input()` native and prints results with `print()`. stdout is compared to `expectedOutput` using the platform's existing `outputsMatch` (exact, JSON, and float-tolerant comparison).

```vy
// A VyroLang solution in VyroCoding
let a = int(input())
let b = int(input())
print(a + b)
```

## Status mapping

| vyro CLI outcome | VyroCoding verdict |
|---|---|
| exit 0, output matches | `accepted` |
| exit 0, output differs | `wrong_answer` |
| `runtime error: ...` on stderr | `runtime_error` |
| parse/compile diagnostic (`line N: ...`) | `compile_error` |
| killed after `VYRO_TIMEOUT_MS` | `time_limit_exceeded` |

## Configuration

Set in VyroCoding's `.env` (see `.env.example`):

```bash
VYRO_BIN=/abs/path/to/VyroLang/impl/target/release/vyro   # or `vyro` on PATH
VYRO_TIMEOUT_MS=5000
```

Build the binary first:

```bash
cd VyroLang/impl && cargo build --release
```

## What changed in VyroCoding

- `packages/types`: added `Language.Vyro` (id `9001`, outside Judge0's range) + name/monaco maps.
- `apps/api/src/services/vyro.service.ts`: runs `.vy` via the `vyro` CLI (temp file + `spawn` + stdin) with a timeout and verdict classification.
- `apps/api/src/services/runner.service.ts`: language dispatcher (Vyro → VyroVM, else → Judge0).
- `apps/api/src/routes/execute.routes.ts` + `services/execution.queue.ts`: import the dispatcher.
- `apps/api/src/routes/languages.routes.ts`: offers **VyroLang** in the language picker with a starter template.
- `apps/api/src/config/env.ts`: `VYRO_BIN`, `VYRO_TIMEOUT_MS`.

## Verified

End-to-end against the built binary: correct output → `accepted`; wrong output → `wrong_answer`; `let x =` → `compile_error`; `print(1/0)` → `runtime_error`.

## Next steps (hardening)

This runs the VM as a local subprocess. Before exposing untrusted user code in production, run it inside the Docker sandbox described in the [Security Model](../11-security/SECURITY_MODEL.md) (CPU/RAM/PID limits, seccomp, no-net, non-root) rather than directly on the host.
