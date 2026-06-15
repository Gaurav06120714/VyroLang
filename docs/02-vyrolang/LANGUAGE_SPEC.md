# VyroLang — Language Specification (Draft v0.1)

## Goal

A modern, readable language with Python-like ergonomics and Rust-influenced predictability. Easy to start, safe to scale.

## Design Choices

- **Familiar syntax**, braces for blocks, no required semicolons.
- **`let` bindings**, optional type annotations, type inference by default.
- **Expression-oriented** where natural; statements where clearer.
- **First-class functions**, classes, modules, async, and structured error handling.
- File extension: `.vy`. Entry point: `main()`.

## Lexical Structure

- **Comments**: `// line` and `/* block */`.
- **Identifiers**: `[A-Za-z_][A-Za-z0-9_]*`.
- **Keywords**: `let`, `const`, `func`, `class`, `if`, `else`, `for`, `while`, `in`, `return`, `break`, `continue`, `async`, `await`, `try`, `catch`, `throw`, `import`, `export`, `true`, `false`, `null`.
- **Literals**: integers, floats, strings (`"..."`), booleans, `null`.

## Variables

```vy
let name = "Gaurav"     // inferred String
let age: Int = 20       // explicit type
const PI = 3.14159      // immutable
```

## Data Types

| Type | Example | Notes |
|---|---|---|
| `String` | `"hello"` | UTF-8 |
| `Int` | `42` | 64-bit signed |
| `Float` | `3.14` | 64-bit IEEE-754 |
| `Bool` | `true` | |
| `Array<T>` | `[1, 2, 3]` | ordered, growable |
| `Object` | `{ name: "G", age: 20 }` | struct-like |
| `Map<K,V>` | `Map{ "a": 1 }` | keyed |
| `Set<T>` | `Set{1, 2, 3}` | unique |
| `null` | `null` | absence |

## Operators

- Arithmetic: `+ - * / % **`
- Comparison: `== != < <= > >=`
- Logical: `&& || !`
- Assignment: `= += -= *= /=`
- Range: `1..10` (exclusive), `1..=10` (inclusive)
- Member/index: `obj.field`, `arr[i]`

## Conditions

```vy
if age > 18 {
    print("Adult")
} else if age > 12 {
    print("Teen")
} else {
    print("Child")
}
```

## Loops

```vy
for i in 1..10 {
    print(i)
}

for item in items {
    print(item)
}

while running {
    tick()
}
```

`break` and `continue` are supported.

## Functions

```vy
func add(a: Int, b: Int) -> Int {
    return a + b
}

func greet(name) {           // inferred types
    print("Hello, " + name)
}
```

- Default parameters and variadics planned for v0.4.
- Functions are first-class values.

## Classes

```vy
class User {
    name: String
    age: Int

    func init(name, age) {
        self.name = name
        self.age = age
    }

    func describe() -> String {
        return self.name + " (" + self.age + ")"
    }
}

let u = User("Gaurav", 20)
print(u.describe())
```

- Single inheritance via `class Admin : User { ... }` (planned v0.4).

## Async

```vy
async func fetchData(url) -> String {
    let res = await http.get(url)
    return res.body
}
```

Backed by the VM's async runtime and scheduler (coroutines). See [VM Design](../04-vm/VM_DESIGN.md).

## Error Handling

```vy
try {
    let data = parse(input)
} catch err {
    print("Failed: " + err.message)
}

throw Error("invalid input")
```

## Modules

```vy
import math
import { readFile } from "fs"

export func square(x) -> Int { return x * x }
```

Modules map to files/packages; resolution handled by the compiler and [vpm](../06-package-manager/VPM.md).

## Standard Library (initial surface)

- `print`, `len`, `range`
- `math` (sqrt, sin, cos, pow, abs)
- `string` (split, join, trim, replace)
- `array` (map, filter, reduce, push, pop)
- `fs` (read, write) — via [VyroOS](../05-vyroos/OS_LAYER.md)
- `http` (get, post) — sandbox-gated
- `json` (parse, stringify)

## Grammar (EBNF excerpt)

```
program     = { statement } ;
statement   = letDecl | funcDecl | classDecl | ifStmt | forStmt
            | whileStmt | tryStmt | return | exprStmt | import | export ;
letDecl     = ("let" | "const") IDENT [ ":" type ] "=" expr ;
funcDecl    = ["async"] "func" IDENT "(" [ params ] ")" [ "->" type ] block ;
block       = "{" { statement } "}" ;
expr        = assignment ;
```

Full grammar maintained alongside the parser; see [Compiler Design](../03-compiler/COMPILER_DESIGN.md).

## Future Improvements

- Generics with bounds, pattern matching (`match`), enums/sum types, traits/interfaces, named/default args, decorators, and a formatter (`vyro fmt`).

## Risk Analysis

| Risk | Mitigation |
|---|---|
| Syntax churn breaking early code | Freeze core grammar at v0.2; gate changes behind versioned spec |
| Type-system overreach | Ship inference + optional annotations first; add power later |
| Stdlib scope creep | Minimal surface in v0.4; grow via packages |
