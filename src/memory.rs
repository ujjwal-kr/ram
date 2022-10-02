use byteorder::{BigEndian, ReadBytesExt};
use rand::Rng;
use std::borrow::Cow;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Memory {
    pub stack: Vec<u8>,
    pub heap: HashMap<u32, Vec<u8>>,
    pub ret: Vec<u8>,
}

pub struct Types {
    pub int: HashMap<String, Location>,
    pub str: HashMap<String, Location>,
    pub vec: HashMap<String, Location>,
}

pub struct Location {
    start: usize,
    size: usize,
}

// implementations

impl Memory {
    pub fn new() -> Self {
        Self {
            stack: vec![],
            heap: HashMap::from([(0, vec![0, 0, 0, 0])]),
            ret: vec![],
        }
    }

    // stack operations

    pub fn pop_stack(&mut self, n: usize) {
        self.stack.pop();
    }

    pub fn reset_stack(&mut self) {
        self.stack = vec![];
    }

    pub fn load(&mut self, location: Location) -> &[u8] {
        let slice = &self.stack[location.start..location.start + location.size];
        slice
    }

    pub fn store(&mut self, val: Vec<u8>) -> Location {
        let address = self.stack.len();
        for byte in val {
            self.stack.push(byte);
        }

        Location {
            start: address,
            size: val.len(),
        }
    }

    // heap operations

    pub fn malloc(&mut self, bytes: Vec<u8>) -> u32 {
        let addr_bytes = rand::thread_rng().gen::<[u8; 4]>();
        let addr: u32 = u32::from_be_bytes(addr_bytes);
        self.heap.insert(addr, bytes);
        addr
    }

    pub fn free(&mut self, addr: u32) {
        self.heap.remove(&addr);
    }

    pub fn heap_load(&mut self, addr: u32) -> Vec<u8> {
        let bytes: Vec<u8>;
        match self.heap.get(&addr) {
            Some(data) => bytes = data.to_vec(),
            _ => panic!("Illegal heap pointer"),
        }
        bytes
    }

    // stack yeilds

    // ints

    pub fn get_int_from_stack(&mut self) -> i32 {
        let location = Location {
            start: self.stack.len() - 4,
            size: 4,
        };
        let bytes = self.load(location);
        let num: i32 = i32::from_be_bytes(bytes.try_into().expect("invalid i32 len"));
        num
    }

    pub fn yeild_i32(&mut self, location: Location) -> i32 {
        let bytes = self.load(location);
        let num: i32 = i32::from_be_bytes(bytes.try_into().expect("invalid i32 len"));
        num
    }

    pub fn yeild_i64(&mut self, location: Location) -> i64 {
        let bytes = self.load(location);
        let num: i64 = i64::from_be_bytes(bytes.try_into().expect("invalid i64 len"));
        num
    }

    pub fn yeild_i128(&mut self, location: Location) -> i128 {
        let bytes = self.load(location);
        let num: i128 = i128::from_be_bytes(bytes.try_into().expect("invalid i128 len"));
        num
    }

    pub fn yeild_u32(&mut self, location: Location) -> u32 {
        let bytes = self.load(location);
        let num: u32 = u32::from_be_bytes(bytes.try_into().expect("invalid u32 len"));
        num
    }

    pub fn yeild_u64(&mut self, location: Location) -> u64 {
        let bytes = self.load(location);
        let num: u64 = u64::from_be_bytes(bytes.try_into().expect("invalid u64 len"));
        num
    }

    pub fn yeild_u128(&mut self, location: Location) -> u128 {
        let bytes = self.load(location);
        let num: u128 = u128::from_be_bytes(bytes.try_into().expect("invalid u128 len"));
        num
    }

    // heap yeilds

    // strings
    pub fn yeild_string(&mut self, location: Location) -> String {
        let addr_bytes = self.load(location);
        let heap_addr = u32::from_be_bytes(addr_bytes.try_into().expect("invalid heap addr len"));
        let str_bytes = self.heap_load(heap_addr);
        let final_str = String::from_utf8_lossy(&str_bytes);
        final_str.into_owned()
    }

    pub fn yeild_int_vec_from_struct(&mut self, structure: String) -> Vec<i32> {
        if !self.get_struct_is_vec(structure.clone()) {
            panic!("Err in vec struct id");
        }
        let vec_type: &str = self.get_vec_type_from_struct(structure.clone());
        if vec_type != "int" {
            panic!("invalid invokation to yeld int vec")
        }
        let heap_addr: String = self.get_heap_addr_from_struct(structure);
        let bytes: Vec<u8> = self.heap_load(heap_addr);
        let mut final_vec: Vec<i32> = vec![];
        for mut byte in bytes.chunks(4) {
            let num: i32 = byte.read_i32::<BigEndian>().unwrap();
            final_vec.push(num);
        }

        final_vec
    }

    pub fn yeild_str_vec_from_struct(&mut self, structure: String) -> Vec<String> {
        if !self.get_struct_is_vec(structure.clone()) {
            panic!("Err in vec struct id");
        }
        let vec_type: &str = self.get_vec_type_from_struct(structure.clone());
        if vec_type != "str" {
            panic!("invalid invokation to yeld str vec")
        }
        let mut final_vec: Vec<String> = vec![];
        let heap_addr: String = self.get_heap_addr_from_struct(structure);
        let addr_bytes: Vec<u8> = self.heap_load(heap_addr);
        for addr_byte in addr_bytes.chunks(6) {
            let addr_str: String = String::from_utf8_lossy(addr_byte).to_string();
            let bytes: &[u8] = &self.heap_load(addr_str);
            let final_str: String = String::from_utf8_lossy(bytes).to_string();
            final_vec.push(final_str);
        }

        final_vec
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

    // parsers

    fn parse_int(&mut self, value: &str, block: &str, line: i32) -> i32 {
        value
            .parse::<i32>()
            .expect(format!("Parse int error at {}:{}", block, line).trim())
    }

    // Integers

    pub fn set_int(&mut self, value: &str, memory: &mut Memory, block: &str, line: i32) -> usize {
        let parsed_val = self.parse_int(value, block, line).to_string();
        let final_value: String = format!("0xffff{}", parsed_val);
        memory.store(final_value)
    }

    pub fn set_var_int(
        &mut self,
        name: String,
        value: &str,
        memory: &mut Memory,
        block: &str,
        line: i32,
    ) {
        let offset: usize = self.set_int(value, memory, block, line);
        self.int.insert(name, offset);
    }

    pub fn get_int(&mut self, name: String, memory: &mut Memory) -> i64 {
        let final_int: i64;
        match self.int.get(&name) {
            Some(&location) => {
                let structure: String = memory.load(location);
                final_int = memory.yeild_int_from_struct(structure);
            }
            _ => todo!("Need to implement errs"),
        }
        final_int
    }

    // Strings

    pub fn set_string(&mut self, name: String, value: &str, memory: &mut Memory) {
        let bytes: Vec<u8> = value.to_string().as_bytes().to_vec();
        let heap_addr: String = memory.malloc(bytes);
        let hex_heap_addr: String = format!("0x0000{}", heap_addr);
        let location: usize = memory.store(hex_heap_addr);
        self.str.insert(name, location);
    }

    pub fn get_string(&mut self, name: String, memory: &mut Memory) -> String {
        let final_str: String;
        match self.str.get(&name) {
            Some(&location) => {
                let structure: String = memory.load(location);
                final_str = memory.yeild_string_from_struct(structure);
            }
            _ => todo!("Need to implement err"),
        }
        final_str
    }

    // vectors

    pub fn set_int_vec(
        &mut self,
        name: String,
        value: &str,
        memory: &mut Memory,
        block: &str,
        line: i32,
    ) {
        let items: &Vec<&str> = &value[1..value.len() - 2].split(',').collect::<Vec<&str>>();
        let mut final_bytes: Vec<u8> = vec![];
        for item in items {
            let int: i32 = self.parse_int(item, block, line);
            let byte: &[u8] = &int.to_be_bytes();
            for bit in byte {
                final_bytes.push(*bit);
            }
        }

        let addr_prefix: &str = "0xaaaaffff";
        let heap_addr: String = memory.malloc(final_bytes);
        let final_addr: String = format!("{}{}", addr_prefix, heap_addr);

        let location: usize = memory.store(final_addr);
        self.vec.insert(name, location);
    }

    pub fn set_str_vec(&mut self, name: String, value: &str, memory: &mut Memory) {
        let items: &Vec<&str> = &value[1..value.len() - 2].split(',').collect::<Vec<&str>>();
        let mut heap_addrs_bytes: Vec<u8> = vec![];
        for item in items {
            let byte: Vec<u8> = item.as_bytes().to_vec();
            let heap_addr_current: String = memory.malloc(byte);
            let current_addr_bytes: Vec<u8> = heap_addr_current.as_bytes().to_vec();
            for byte in current_addr_bytes {
                heap_addrs_bytes.push(byte);
            }
        }

        let addr_prefix: &str = "0xaaaa0000";
        let heap_addr: String = memory.malloc(heap_addrs_bytes);
        let final_heap_addr: String = format!("{}{}", addr_prefix, heap_addr);
        let location: usize = memory.store(final_heap_addr);
        self.vec.insert(name, location);
    }
}
