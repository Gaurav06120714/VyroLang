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
| `vyro version` | Print version |

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

## Language supported in v0.1

- `let` / `const` bindings (top-level → globals; in-scope → locals), assignment.
- Types: `Int`, `Float`, `Bool`, `String`, `null`.
- Operators: `+ - * / %`, comparisons, `&& || !` (short-circuit), unary `-`.
- `+` does numeric add or string concatenation (`"x = " + 5`).
- Control flow: `if / else if / else`, `while`, `for i in a..b`.
- Functions with parameters, recursion, `return`; first-class function values.
- `print(...)` builtin (space-separated, newline).
- Comments: `// line` and `/* block */`.
- Optional type annotations (`: Type`, `-> Type`) are parsed and currently ignored.

## Tests

```bash
cargo test
```

12 end-to-end tests cover arithmetic/precedence, strings, conditionals, short-circuit logic, while/for loops, recursion, local isolation, FizzBuzz, and error reporting (`tests/run.rs`).

## Roadmap (next)

Per the [30-Day Roadmap](../planning/ROADMAP_30_DAY.md) and [Language Spec](../docs/02-vyrolang/LANGUAGE_SPEC.md): arrays/maps, classes, `import`/modules, optimizer passes (constant folding, DCE), a real GC, and the `async` runtime.
