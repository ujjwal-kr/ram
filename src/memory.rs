use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::{collections::HashMap, vec};

#[derive(Debug)]
pub struct Memory {
    pub stack: Vec<String>,
    pub heap: HashMap<String, Vec<u8>>,
    pub lx: i32,
    pub rv: i32,
    pub ret: Vec<u8>,
}

pub struct Types {
    pub int: HashMap<String, Location>,
    pub str: HashMap<String, Location>,
    pub vec: HashMap<String, Location>,
}

pub struct Location {
    start: usize,
    end: usize,
}

pub enum StructID {
    Int(String), // 0xffff
    Str(String), // 0x0000
    Vec(String), // 0xaaaa
}

// implementations

impl Memory {
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

    pub fn load(&mut self, location: Location) -> Vec<String> {
        let addrs: Vec<String>;
        let refaddrs = &self.stack[location.start..location.end];
        addrs = refaddrs.to_vec();
        addrs
    }

    pub fn store(&mut self, addrs: Vec<String>) -> usize {
        for addr in addrs {
            self.stack.push(addr);
        }
        return self.stack.len();
    }

    // heap operations

    pub fn malloc(&mut self, bytes: Vec<u8>) -> String {
        let heap_addr: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(12)
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
            _ => panic!("Illegal heap pointer"),
        }
        bytes
    }

    // Structure stuff

    pub fn get_struct_id(value: String) -> StructID {
        todo!()
    }

    pub fn get_stack_struct_id() -> StructID {
        todo!()
    }

    pub fn yeild_value_from_struct_id<T>(id: StructID) -> T {
        todo!()
    }

}

impl Types {
    pub fn new() -> Self {
        Self {
            int: HashMap::new(),
            str: HashMap::new(),
            vec: HashMap::new(),
        }
    }
 
    // Integers

    pub fn set_int(value: &str, memory: &mut Memory) -> usize {
        memory.store(vec![value.to_owned()])
    }

    pub fn set_var_int(name: String, value: &str, memory: &mut Memory) {
        todo!()
    }

    pub fn get_int(name: String, memory: &mut Memory) -> i64 {
        todo!()
    }

    // Strings

    pub fn set_string(name: String, value: &str, memory: &mut Memory) {
        
    }

    pub fn get_string(name: String, memory: &mut Memory) -> String {
        todo!()
    }
}
