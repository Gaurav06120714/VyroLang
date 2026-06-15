# VyroCompiler — Design

## Goal

Transform VyroLang source (`.vy`) into VyroVM bytecode (`.vbc`) with precise diagnostics, through a classic, testable pipeline.

## Pipeline

```
Source (.vy)
   ▼  Lexer        → tokens
   ▼  Parser       → AST
   ▼  Sema         → typed/validated AST + symbol table
   ▼  IR lowering  → intermediate representation
   ▼  Optimizer    → optimized IR
   ▼  Codegen      → bytecode (.vbc)
```

## Folder Structure (in VyroCoding)

```
compiler/
├── src/
│   ├── lexer/        # tokenizer
│   ├── parser/       # AST construction
│   ├── ast/          # node definitions
│   ├── sema/         # type & scope analysis
│   ├── ir/           # lowering + IR types
│   ├── opt/          # optimization passes
│   ├── codegen/      # bytecode emitter
│   ├── diagnostics/  # error reporting
│   └── main.rs       # CLI entry (vyroc)
├── tests/
│   ├── golden/       # snapshot tests of bytecode
│   └── fixtures/     # .vy inputs
```

Technology: **Rust** (safety, performance, strong tooling).

## 1. Lexer

Converts characters into tokens with source spans.

```
let x = 5      ⇒  LET  IDENT("x")  EQ  NUMBER(5)
```

- Tracks line/column for diagnostics.
- Handles comments, string escapes, numeric literals.

## 2. Parser

Recursive-descent + Pratt parsing for expressions (precedence climbing).

```
Program
 └─ VariableDeclaration(name="x")
     └─ NumberLiteral(5)
```

- Produces an AST defined in `ast/`.
- Error recovery: synchronize at statement boundaries to report multiple errors.

## 3. Semantic Analyzer

Validates:

- **Types** — inference + checking against annotations.
- **Variables** — declared before use; const not reassigned.
- **Functions** — arity and signature matching.
- **Scopes** — lexical scoping; shadowing rules.

Outputs a typed AST and a symbol table.

## 4. Optimization Passes

| Pass | Effect |
|---|---|
| Constant Folding | evaluate `2 + 3` → `5` at compile time |
| Dead Code Elimination | remove unreachable/unused code |
| Inline Expansion | inline small, hot functions |
| Loop Optimization | hoist invariants, simplify bounds |

Passes operate on IR and are individually toggleable for testing.

## 5. Code Generation

Emits stack-machine bytecode for the VM:

```
PUSH, POP, LOAD, STORE, ADD, SUB, MUL, DIV,
CMP, JMP, JMPF, CALL, RETURN, NEWOBJ, GETFIELD, SETFIELD, ...
```

Example — `let r = add(2, 3)`:

```
PUSH 2
PUSH 3
CALL add, 2
STORE r
```

See the full opcode set in [VM Design](../04-vm/VM_DESIGN.md).

## Diagnostics

Rust-style messages: error code, message, source span, and a hint.

```
error[E0102]: cannot add Int and String
  ┌─ main.vy:4:13
  │
4 │   let x = 5 + "a"
  │           ^^^^^^^ Int + String is not allowed
  = hint: convert with string(5)
```

## CLI (`vyroc`)

```
vyroc build main.vy -o main.vbc
vyroc check main.vy          # diagnostics only
vyroc emit-ast main.vy       # debug
vyroc emit-bytecode main.vy  # debug
```

## Testing Strategy

- Unit tests per stage (lexer/parser/sema/opt).
- Golden tests: `.vy` → expected `.vbc` / expected AST.
- Negative tests: malformed input must produce the right error code.
- Fuzzing the lexer/parser.

## Estimated Development Time

Frontend (lexer→sema): ~3–4 weeks. IR+optimizer+codegen: ~3–4 weeks. (Tracked in [30-Day Roadmap](../../planning/ROADMAP_30_DAY.md).)

## Future Improvements

- Incremental compilation, LSP server (for IDE), source maps for the debugger, and an optional native AOT backend (LLVM).

## Risk Analysis

| Risk | Mitigation |
|---|---|
| Type system complexity | start with inference + basic checks |
| Codegen/VM drift | shared bytecode spec + golden tests |
| Perf of naive passes | profile; optimize hot passes later |
