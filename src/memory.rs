use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::{collections::HashMap, vec};

#[derive(Debug)]
pub struct Memory<'a> {
    pub stack: Vec<&'a str>,
    pub heap: HashMap<String, Vec<u8>>,
    pub lx: i32,
    pub rv: i32,
    pub ret: Vec<u8>
}


pub enum Type {
    Int(Int),
    Str(Str),
    Vec(Vectr)
}

pub struct Location {
    start: usize,
    end: usize,
}

// allocation on stack

pub struct Int(HashMap<String, Location>);

pub struct Str(HashMap<String, Location>);

pub struct Vectr {
    content: HashMap<String, Location>,
}

// implementations

impl Memory<'static> {
    pub fn new() -> Self {
        Self {
            stack: vec![],
            heap: HashMap::new(),
            lx: 0i32,
            rv: 0i32,
            ret: vec![],
        }
    }

    // stack operations

    pub fn pop_stack(&mut self, n: usize) {
        for _n in 0..n {
            self.stack.pop();
        }
    }

    pub fn reset_stack(&mut self) {
        self.stack = vec![];
    }

    pub fn load(&mut self, location: Location) -> Vec<&str> {
        let addrs: Vec<&str>;
        let refaddrs = &self.stack[location.start..location.end];
        addrs = refaddrs.to_vec();
        addrs
    }

    pub fn store(&mut self, addrs: Vec<&'static str>) {
        for addr in addrs {
            self.stack.push(addr);
        }
    }

    // heap operations

    pub fn malloc(&mut self, bytes: Vec<u8>) -> String {
        let heap_addr: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(15)
            .map(char::from)
            .collect();

        self.heap.insert(heap_addr.clone(), bytes);
        heap_addr
    }

    pub fn free(&mut self, addr: String) {
        self.heap.remove(&addr);    
    }

    pub fn heap_load(&mut self, addr: String) -> Vec<u8> {
        let bytes: Vec<u8>;
        match self.heap.get(&addr) {
            Some(data) => bytes = data.to_vec(),
            _ => panic!("Heap Overflow"),
        }
        bytes
    }
    
}
