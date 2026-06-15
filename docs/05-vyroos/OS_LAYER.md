# VyroOS — System Abstraction Layer

## Goal

Provide a clean, **capability-based** API surface for Vyro programs to interact with the system — files, processes, memory, and network — without granting raw host access. Long-term, evolve toward kernel-level research (see [VyroOs](https://github.com/Gaurav06120714/VyroOs)).

## Positioning

VyroOS starts as a **layer**, not a bootable kernel. It sits between the VM and the host/sandbox, mediating every system interaction so the sandbox can enforce policy.

```
VyroLang stdlib  →  VyroOS API  →  policy gate  →  host (inside Docker sandbox)
```

## Capability Model

Every VyroOS call requires a capability granted to the workspace. No capability → call denied. This is how the sandbox stays safe even as programs use real-looking APIs.

| Capability | Grants |
|---|---|
| `fs.read` | read files within the workspace |
| `fs.write` | write files within the workspace |
| `net.http` | outbound HTTP (rate-limited, allowlist) |
| `proc.spawn` | spawn child processes (restricted) |
| `mem.alloc` | heap allocation up to the workspace cap |

## File System API

```vy
let content = file.read("notes.txt")
file.write("out.txt", "hello")
let exists = file.exists("config.json")
let names = dir.list(".")
```

- Paths are jailed to the workspace root (no `..` escapes).
- Quotas on total size and file count.

## Process API

```vy
let p = process.start("vyrovm", ["job.vbc"])
let code = process.wait(p)
```

- Restricted to allowlisted binaries inside the sandbox.

## Memory API

```vy
let region = memory.allocate(1024)   // bytes, within cap
memory.free(region)
let stats = memory.stats()           // used/limit
```

## Network API

```vy
let res = http.get("https://api.example.com/data")
let post = http.post(url, body)
```

- Default-deny; enabled only with `net.http`, behind rate limits and an allowlist.

## Folder Structure (in VyroOs)

```
vyroos/
├── src/
│   ├── caps/        # capability definitions + gate
│   ├── fs/          # filesystem abstraction (jailed)
│   ├── proc/        # process control
│   ├── mem/         # memory accounting
│   ├── net/         # network (allowlist + limits)
│   └── lib.rs       # VyroOS API surface
└── tests/
```

Technology: **Rust**, exposed to the VM via a stable internal ABI.

## Future Roadmap

- **Kernel research** — minimal microkernel experiments in VyroOs.
- **Drivers** — abstracted device interfaces.
- **Shell** — a VyroLang-scriptable shell.
- **Package Loader** — load system services as Vyro packages.
- **System Services** — scheduler, logging, init.

## Testing Strategy

- Capability gate tests (deny without grant; allow with grant).
- Path-jail tests (escape attempts fail).
- Quota enforcement tests.
- Network allowlist tests.

## Estimated Development Time

Capability layer + FS/Mem: ~2 weeks. Net/Process + quotas: ~2 weeks. ([v0.5.0](../../versions/v0.5.0-vpm-os.md))

## Risk Analysis

| Risk | Mitigation |
|---|---|
| Path traversal escape | canonicalize + jail + tests |
| Capability bypass | single gate; deny-by-default; audits |
| Scope toward full kernel | keep as layer; kernel work isolated in VyroOs |
