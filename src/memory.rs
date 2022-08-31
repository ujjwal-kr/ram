use std::{collections::HashMap, vec};

pub struct Memory {
    pub stack: Vec<u8>,
    pub heap: HashMap<String, Vec<u8>>,
    pub lx: i32,
    pub rv: i32,
    pub ret: HashMap<Type, Location>,
}

pub enum Type {
    Int(Int),
    Str(Str),
}

pub struct Location {
    start: usize,
    end: usize,
}

// allocation on stack
pub struct Int(HashMap<String, Location>);
pub struct Str(HashMap<String, Location>);

// allocation on heap
pub struct Vectr {
    _type: Option<Type>,
    content: HashMap<String, String>,
}

// implementations

impl Memory {
    pub fn new() -> Self {
        let mut stack: Vec<u8> = vec![];
        for _ in 0..1024 {
            stack.push(0u8)
        }
        Self {
            stack,
            heap: HashMap::new(),
            lx: 0i32,
            rv: 0i32,
            ret: HashMap::new(),
        }
    }

    // stack operations

    pub fn push(&mut self, bytes: u8) {
        self.stack.push(bytes);
    }

    pub fn pop(&mut self, n: usize) {
        for _n in 0..n {
            self.stack.pop();
        }
    }

    pub fn reset_stack(&mut self) {
        self.stack = vec![];
    }

    pub fn load(&mut self, location: Location) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![];
        let refbytes = &self.stack[location.start..location.end].to_vec();
        for b in refbytes {
            bytes.push(*b)
        }
        bytes
    }

    pub fn store(&mut self, bytes: Vec<u8>, location: Location) {
        self.stack.splice(location.start..location.end, bytes);
    }
}
