//! Bytecode instruction set and chunks for the VyroVM.

use crate::value::Value;

#[derive(Debug, Clone)]
pub enum Op {
    // Stack / constants
    Const(usize), // push constants[i]
    True,
    False,
    Null,
    Pop,
    Dup, // push a copy of the top of stack

    // Variables
    DefineGlobal(usize), // name index in constants
    GetGlobal(usize),
    SetGlobal(usize),
    GetLocal(usize),
    SetLocal(usize),

    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Neg,

    // Comparison / logic
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    Not,

    // Control flow (absolute instruction indices, patched by the compiler)
    Jump(usize),
    JumpIfFalse(usize),      // pops condition, jumps if falsey
    JumpIfFalsePeek(usize),  // jumps if top falsey, leaves it (for &&)
    JumpIfTruePeek(usize),   // jumps if top truthy, leaves it (for ||)

    // Collections
    NewArray(usize), // pop n elements, push an array
    NewMap(usize),   // pop n key/value pairs (2n values), push a map
    IndexGet,        // arr|map, idx|key -> value
    IndexSet,        // arr|map, idx|key, value -> value (value left on stack)

    // Objects / classes
    Class(usize),    // name const -> push empty class
    Method(usize),   // name const: pop func, attach to class on top of stack
    GetProp(usize),  // name const: instance -> field value
    SetProp(usize),  // name const: instance, value -> value (left on stack)
    Invoke(usize, usize), // method name const, argc: method call on receiver

    // Calls
    Call(usize),    // argc; callee is below the args on the stack
    Print(usize),   // native: argc values, prints space-separated + newline
    Native(usize, usize), // native id, argc
    Return,
}

#[derive(Clone, Default)]
pub struct Chunk {
    pub code: Vec<Op>,
    pub constants: Vec<Value>,
    pub lines: Vec<usize>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk { code: Vec::new(), constants: Vec::new(), lines: Vec::new() }
    }

    pub fn emit(&mut self, op: Op) -> usize {
        self.code.push(op);
        self.lines.push(0);
        self.code.len() - 1
    }

    pub fn add_const(&mut self, v: Value) -> usize {
        self.constants.push(v);
        self.constants.len() - 1
    }
}

// Manual Clone for Value is provided in value.rs; Chunk derives Clone which
// requires Value: Clone (it is) and Op: Clone (derived above).
