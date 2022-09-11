use byteorder::{BigEndian, ReadBytesExt};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::borrow::Cow;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Memory {
    pub stack: Vec<String>,
    pub heap: HashMap<String, Vec<u8>>,
    pub ret: Vec<u8>,
}

pub struct Types {
    pub int: HashMap<String, usize>,
    pub str: HashMap<String, usize>,
    pub vec: HashMap<String, usize>,
}

// implementations

impl Memory {
    pub fn new() -> Self {
        Self {
            stack: vec![],
            heap: HashMap::new(),
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

    pub fn load(&mut self, location: usize) -> String {
        self.stack[location].clone()
    }

    pub fn store(&mut self, val: String) -> usize {
        self.stack.push(val);
        return self.stack.len();
    }

    // heap operations

    pub fn malloc(&mut self, bytes: Vec<u8>) -> String {
        let heap_addr: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(6)
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

    fn get_struct_id(&mut self, structure: String) -> String {
        let type_slice: &str = &structure[0..6];
        type_slice.to_owned()
    }

    fn get_struct_is_string(&mut self, structure: String) -> bool {
        let struct_type: String = self.get_struct_id(structure);
        if struct_type == "0x0000".to_string() {
            true
        } else {
            false
        }
    }

    fn get_struct_is_int(&mut self, structure: String) -> bool {
        let struct_type: String = self.get_struct_id(structure);
        if struct_type == "0xffff".to_string() {
            true
        } else {
            false
        }
    }

    fn get_struct_is_vec(&mut self, structure: String) -> bool {
        let struct_type: String = self.get_struct_id(structure);
        if struct_type == "0xaaaa".to_string() {
            true
        } else {
            false
        }
    }

    fn get_vec_type_from_struct(&mut self, structure: String) -> &str {
        let slice = &structure[6..10];
        match slice {
            "0000" => "string",
            "ffff" => "int",
            "aaaa" => "vec",
            _ => panic!("Illegal struct type"),
        }
    }

    fn get_heap_addr_from_struct(&mut self, structure: String) -> String {
        let addr: &str = &structure[structure.len() - 8..structure.len() - 1];
        addr.to_string()
    }

    // stack yeilds

    pub fn yeild_int_from_stack(&mut self) -> i64 {
        let structure: String = self.stack[self.stack.len() - 1].clone();
        if self.get_struct_is_int(structure.clone()) {
            self.yeild_int_from_struct(structure)
        } else {
            panic!("Illegal use of yeild int from stack")
        }
    }

    pub fn yeild_string_from_stack(&mut self) -> String {
        let structure: String = self.stack[self.stack.len() - 1].clone();
        if self.get_struct_is_string(structure.clone()) {
            self.yeild_string_from_struct(structure)
        } else {
            panic!("Illegal use of yeild str from stack")
        }
    }

    pub fn yeild_int_from_struct(&mut self, structure: String) -> i64 {
        if !self.get_struct_is_int(structure.clone()) {
            panic!("Err in int struct id")
        }
        let slice: &str = &structure[6..structure.len()];
        slice.parse::<i64>().expect("int struct parse err")
    }

    // heap yeilds

    pub fn yeild_string_from_struct(&mut self, structure: String) -> String {
        if !self.get_struct_is_string(structure.clone()) {
            panic!("Err in str struct id")
        }
        let slice: &str = &structure[6..structure.len()];
        let bytes: Vec<u8> = self.heap_load(slice.to_string());
        let value: Cow<str> = String::from_utf8_lossy(&bytes);
        value.to_string()
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
        value.parse::<i32>().expect(
            format!("Parse int error at {}:{}", block, line).trim()
        )
    }

    fn parse_int_vec(value: &str, block: &str, line: i32) -> Vec<i32> {
        todo!()
    }

    fn parse_str_vec(value: &str, block: &str, line: i32) -> Vec<String> {
        todo!()
    }

    // Integers

    pub fn set_int(&mut self, value: &str, memory: &mut Memory) -> usize {
        let final_value: String = format!("0xffff{}", value);
        memory.store(final_value)
    }

    pub fn set_var_int(&mut self, name: String, value: &str, memory: &mut Memory) {
        let offset: usize = self.set_int(value, memory);
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

    pub fn set_int_vec(&mut self, name: String, value: &str, memory: &mut Memory, block: &str, line: i32) {
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
        let heap_addr:String = memory.malloc(final_bytes);
        let final_addr: String = format!("{}{}", addr_prefix, heap_addr);

        let location: usize = memory.store(final_addr);
        self.vec.insert(name, location);
    }

    pub fn set_str_vec(&mut self, name: String, value: &str, memory: &mut Memory) {
        let items: &Vec<&str> = &value[1..value.len() -2].split(',').collect::<Vec<&str>>();
        let mut heap_addrs_bytes: Vec<u8> = vec![];
        for item in items {
            let byte: Vec<u8> = item.as_bytes().to_vec();
            let heap_addr_current: String = memory.malloc(byte);
            let current_addr_bytes = heap_addr_current.as_bytes().to_vec();
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
