use std::{collections::HashMap};

pub struct Memory {
    stack: Vec<u8>,
    heap: HashMap<String, Vec<u8>>,
    lx: i32,
    rv: i32,
    ret: HashMap<Type, Addr>,
}

pub enum Type {
    Int(Int),
    Str(Str),
    Vectr(Vectr)
}

pub struct Addr(i32);

// allocation on stack
pub struct Int(HashMap<String, Addr>);
pub struct Str(HashMap<String, Addr>);

// allocation on heap
pub struct Vectr(HashMap<String, String>);