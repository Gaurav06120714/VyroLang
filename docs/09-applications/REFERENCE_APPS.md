# Reference Applications (built in VyroLang)

These apps prove the ecosystem end-to-end and double as tutorials. Each is written entirely in VyroLang, compiled by VyroCompiler, run on VyroVM, and runnable in VyroCloud.

---

## 1. Todo Application

### Goal
A task manager demonstrating storage, state, and UI logic in VyroLang.

### Features
Task creation, editing, deletion, deadlines, categories, priority levels, search, dark mode, notifications.

### Storage
SQLite (primary) or JSON (simple mode), via the [VyroOS](../05-vyroos/OS_LAYER.md) FS capability.

### Data Model
```sql
CREATE TABLE tasks (
  id INTEGER PRIMARY KEY,
  title TEXT NOT NULL,
  notes TEXT,
  category TEXT,
  priority INTEGER DEFAULT 1,   -- 1 low … 3 high
  due_date TEXT,
  done INTEGER DEFAULT 0,
  created_at TEXT
);
```

### Example (VyroLang)
```vy
class Task {
    id: Int
    title: String
    priority: Int
    done: Bool
}

func addTask(title, priority) -> Task {
    let t = Task()
    t.title = title
    t.priority = priority
    t.done = false
    db.insert("tasks", t)
    return t
}
```

### Testing
Unit tests for CRUD + search; integration test against SQLite.

---

## 2. Calculator

### Goal
Show the language's expression handling with a real expression parser.

### Features
Basic ops, scientific mode, memory, history, expression parser.

### Examples
```
(5 + 3) * 4   → 32
sin(90)       → 1
sqrt(16)      → 4
```

### Design
A small Pratt parser inside the app evaluates expressions — mirroring the compiler's expression parsing on a small scale.

### Example (VyroLang)
```vy
func evaluate(expr: String) -> Float {
    let tokens = tokenize(expr)
    let ast = parseExpr(tokens)
    return eval(ast)
}
```

### Testing
Property tests on arithmetic; golden tests for scientific functions.

---

## 3. Tic-Tac-Toe

### Goal
Demonstrate game state, multiplayer, and an AI opponent.

### Features
Single player, multiplayer, AI opponent, leaderboard.

### AI
**Minimax** (with optional alpha-beta pruning) for an unbeatable opponent.

### Example (VyroLang)
```vy
func minimax(board, isMax) -> Int {
    let result = checkWinner(board)
    if result != null { return score(result) }

    var best = isMax ? -1000 : 1000
    for move in emptyCells(board) {
        board[move] = isMax ? "X" : "O"
        let val = minimax(board, !isMax)
        board[move] = null
        best = isMax ? max(best, val) : min(best, val)
    }
    return best
}
```

### Testing
Minimax correctness (AI never loses); leaderboard persistence tests.

---

## Build Order

Built during Phase 9 to validate the toolchain before Cloud GA. See [90-Day Roadmap](../../planning/ROADMAP_90_DAY.md) and [v0.6.0](../../versions/v0.6.0-ide.md).
