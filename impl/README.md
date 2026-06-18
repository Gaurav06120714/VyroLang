# Vyro Toolchain (reference implementation)

The first working implementation of VyroLang: a **lexer → parser → bytecode compiler → stack VM**, in Rust. This is the genuinely greenfield core of the ecosystem (the IDE/Cloud/AI live in VyroCoding; the OS in VyroOs — see [Existing Repo Analysis](../docs/00-overview/EXISTING_REPO_ANALYSIS.md)).

## Build & Run

```bash
cd impl
cargo build --release

./target/release/vyro run ../examples/hello.vy
./target/release/vyro run ../examples/fib.vy
./target/release/vyro check ../examples/loops.vy
./target/release/vyro version
```

## Commands

| Command | Description |
|---|---|
| `vyro run <file.vy>` | Compile and execute |
| `vyro check <file.vy>` | Parse + compile only (diagnostics, no execution) |
| `vyro serve [port]` | Start the Compiler API + VyroIDE (default `8787`) |
| `vyro version` | Print version |

## Run the whole stack (your own pipeline)

This realizes the architecture diagram end-to-end in this one repo — no external platform:

```bash
cargo run --release -- serve 8787
# open http://localhost:8787  →  your VyroIDE in the browser
```

```
Browser ─▶ VyroIDE (web/index.html) ─▶ Compiler API (src/server.rs)
        ─▶ VyroCompiler ─▶ Bytecode ─▶ VyroVM (sandboxed: instruction + time limits)
```

Sandboxed in a container (the "Docker Sandbox / Linux VPS" layer):

```bash
./scripts/sandbox-run.sh        # builds the image, runs with CPU/RAM/PID limits, read-only fs, cap-drop
```

API endpoints: `GET /` (IDE), `GET /health`, `POST /api/run` `{code, stdin}`, `POST /api/compile` `{code}`.

## Pipeline (maps 1:1 to the design docs)

| Stage | File | Design doc |
|---|---|---|
| Lexer | [`src/lexer.rs`](src/lexer.rs) | [Compiler Design](../docs/03-compiler/COMPILER_DESIGN.md) |
| Tokens | [`src/token.rs`](src/token.rs) | — |
| Parser → AST | [`src/parser.rs`](src/parser.rs), [`src/ast.rs`](src/ast.rs) | Compiler Design |
| Bytecode compiler | [`src/compiler.rs`](src/compiler.rs) | Compiler Design |
| Opcodes / chunks | [`src/opcode.rs`](src/opcode.rs) | [VM Design](../docs/04-vm/VM_DESIGN.md) |
| Values | [`src/value.rs`](src/value.rs) | VM Design |
| Stack VM | [`src/vm.rs`](src/vm.rs) | VM Design |

The parser is recursive-descent with precedence-climbing for expressions. The compiler emits stack bytecode (clox-style locals + globals, call frames). The VM is a stack machine with call frames and a global table.

## Language supported

- `let` / `const` bindings (top-level → globals; in-scope → locals), assignment.
- Types: `Int`, `Float`, `Bool`, `String`, `Array`, `null`, plus user **classes**.
- Operators: `+ - * / %`, comparisons, `&& || !` (short-circuit), unary `-`.
- `+` does numeric add or string concatenation (`"x = " + 5`).
- Control flow: `if / else if / else`, `while`, `for i in a..b`.
- Functions with parameters, recursion, `return`; first-class function values.
- **Arrays**: literals `[1, 2, 3]`, indexing `a[i]`, index assignment `a[i] = x`.
- **Classes**: fields, `init`, methods, `self`, instantiation `User(...)`, property
  get/set, and method calls (including method-to-method via `self.m()`).
- **Standard library** (built-ins): `print`, `len`, `push`, `pop`, `str`, `int`,
  `float`, `abs`, `sqrt`, `floor`, `ceil`, `pow`, `min`, `max`, `upper`, `lower`, `type`,
  and `input()` (reads one line from stdin; `null` at EOF — used for competitive I/O
  and by the [VyroCoding integration](../docs/10-cloud/VYROCODING_INTEGRATION.md)).
- String indexing (`"hi"[0]`) and `len` on strings.
- Comments: `// line` and `/* block */`.
- Optional type annotations (`: Type`, `-> Type`) are parsed and currently ignored.

See [`examples/todo.vy`](../examples/todo.vy) for a program using all of the above.

## Tests

```bash
cargo test
```

12 end-to-end tests cover arithmetic/precedence, strings, conditionals, short-circuit logic, while/for loops, recursion, local isolation, FizzBuzz, and error reporting (`tests/run.rs`).

## Roadmap (next)

Per the [30-Day Roadmap](../planning/ROADMAP_30_DAY.md) and [Language Spec](../docs/02-vyrolang/LANGUAGE_SPEC.md): arrays/maps, classes, `import`/modules, optimizer passes (constant folding, DCE), a real GC, and the `async` runtime.
