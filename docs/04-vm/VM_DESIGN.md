# VyroVM — Design

## Goal

A portable, stack-based virtual machine that executes Vyro bytecode (`.vbc`) with managed memory and concurrency — the JVM of the Vyro ecosystem.

## Architecture

```
┌───────────────────────────────────────────────┐
│ VyroVM                                          │
│  ┌──────────┐  ┌────────┐  ┌────────────────┐  │
│  │ Operand  │  │ Call   │  │ Constant Pool  │  │
│  │ Stack    │  │ Stack  │  └────────────────┘  │
│  └──────────┘  └────────┘                       │
│  ┌──────────────────────┐  ┌────────────────┐  │
│  │ Heap (objects)       │  │ Garbage        │  │
│  │                      │◄─┤ Collector      │  │
│  └──────────────────────┘  └────────────────┘  │
│  ┌──────────┐  ┌──────────┐  ┌──────────────┐  │
│  │ Scheduler│  │ Memory   │  │ Async Runtime│  │
│  │          │  │ Manager  │  │ (coroutines) │  │
│  └──────────┘  └──────────┘  └──────────────┘  │
└───────────────────────────────────────────────┘
```

Technology: **Rust**.

## Execution Model

- **Stack-based**: operands pushed/popped from the operand stack.
- **Frames**: each call pushes a frame (locals, return address) onto the call stack.
- **Constant pool**: literals and symbol references per module.

## Bytecode / Opcodes

| Category | Opcodes |
|---|---|
| Stack | `PUSH`, `POP`, `DUP` |
| Locals | `LOAD`, `STORE` |
| Arithmetic | `ADD`, `SUB`, `MUL`, `DIV`, `MOD`, `POW`, `NEG` |
| Comparison/logic | `CMP`, `EQ`, `LT`, `GT`, `AND`, `OR`, `NOT` |
| Control flow | `JMP`, `JMPF`, `JMPT` |
| Functions | `CALL`, `RETURN` |
| Objects | `NEWOBJ`, `GETFIELD`, `SETFIELD` |
| Collections | `NEWARR`, `IDXGET`, `IDXSET`, `LEN` |
| Async | `SPAWN`, `AWAIT`, `YIELD` |

### Example

`func add(a,b){ return a+b }` → `add(2,3)`:

```
PUSH 2
PUSH 3
CALL add, 2     ; new frame; a=2, b=3
  LOAD a
  LOAD b
  ADD
  RETURN
STORE result
```

## Memory Management

- **Heap** holds objects, arrays, strings, closures.
- **Memory Manager** allocates/tracks; exposes stats to the OS layer.
- Allocation respects sandbox RAM limits (hard cap → controlled OOM, not host impact).

## Garbage Collection

- Start: **tracing mark-and-sweep**.
- Evolve: **generational** collector (young/old) to reduce pause times.
- Roots: stack frames, globals, constant pool references.

## Concurrency

| Primitive | Mechanism |
|---|---|
| Threads | OS threads mapped by the scheduler (bounded pool) |
| Coroutines | lightweight, cooperatively scheduled |
| Async runtime | event loop + `SPAWN`/`AWAIT`/`YIELD` opcodes |

The scheduler balances coroutines across worker threads; async I/O integrates with the [VyroOS](../05-vyroos/OS_LAYER.md) network/file capabilities.

## VM CLI (`vyrovm`)

```
vyrovm run main.vbc
vyrovm run main.vbc --max-mem 256m --max-time 5s
vyrovm trace main.vbc       # opcode-level trace (debug)
```

## Safety & Sandboxing Hooks

The VM enforces resource limits passed by the runner: instruction-count/time budget, heap cap, and a capability gate so programs can only touch FS/Net through approved VyroOS calls. See [Security Model](../11-security/SECURITY_MODEL.md).

## Testing Strategy

- Opcode-level unit tests (each instruction's stack effect).
- Conformance suite: bytecode programs with expected results.
- GC stress tests (allocation churn, cycle reclamation).
- Concurrency tests (determinism where promised; race checks).

## Estimated Development Time

Core execution + heap: ~3 weeks. GC + scheduler/async: ~3–4 weeks. ([v0.3.0](../../versions/v0.3.0-vm-runtime.md))

## Future Improvements

- JIT compilation of hot paths, generational/concurrent GC, snapshotting workspaces, and a debug protocol for the IDE debugger.

## Risk Analysis

| Risk | Mitigation |
|---|---|
| GC pauses | start simple; move to generational; measure |
| Async complexity | build sync core first; layer runtime |
| Sandbox bypass via VM | capability gate + runner limits + audits |
