use crate::memory::{Location, Memory};
use std::collections::HashMap;

pub struct Types {
    pub int: HashMap<String, Location>,
    pub str: HashMap<String, Location>,
    pub vec: HashMap<String, Location>,
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

    fn parse_i32(&mut self, value: &str, block: &str, line: i32) -> i32 {
        value
            .parse::<i32>()
            .expect(format!("Parse int error at {}:{}", block, line).trim())
    }

    fn parse_u32(&mut self, value: &str, block: &str, line: i32) -> u32 {
        value
            .parse::<u32>()
            .expect(format!("Parse int error at {}:{}", block, line).trim())
    }

    fn parse_i64(&mut self, value: &str, block: &str, line: i32) -> i64 {
        value
            .parse::<i64>()
            .expect(format!("Parse int error at {}:{}", block, line).trim())
    }

    fn parse_u64(&mut self, value: &str, block: &str, line: i32) -> u64 {
        value
            .parse::<u64>()
            .expect(format!("Parse int error at {}:{}", block, line).trim())
    }

    fn parse_i128(&mut self, value: &str, block: &str, line: i32) -> i128 {
        value
            .parse::<i128>()
            .expect(format!("Parse int error at {}:{}", block, line).trim())
    }

    fn parse_u128(&mut self, value: &str, block: &str, line: i32) -> u128 {
        value
            .parse::<u128>()
            .expect(format!("Parse int error at {}:{}", block, line).trim())
    }

    // Integers

    pub fn set_int(
        &mut self,
        name: String,
        value: &str,
        memory: &mut Memory,
        block: &str,
        line: i32,
    ) {
        let int_bytes = self.parse_i32(value, block, line).to_be_bytes();
        let location: Location = memory.store(&int_bytes);
        self.int.insert(name, location);
    }

    // TODO: make stuff for other types of ints

    pub fn get_int(&mut self, name: String, memory: &mut Memory, block: &str, line: i32) -> i32 {
        let location;
        match self.int.get(&name) {
            Some(loc) => location = loc,
            _ => panic!("Var int not found at {}:{}", block, line),
        }

        memory.yeild_i32(location.clone())
    }

    // Strings

    pub fn set_string(&mut self, name: String, value: &str, memory: &mut Memory) {
        let heap_addr = memory.malloc(value.as_bytes());
        let location = memory.store(&heap_addr.to_be_bytes());
        self.str.insert(name, location);
    }

    pub fn get_string(
        &mut self,
        name: String,
        memory: &mut Memory,
        block: &str,
        line: i32,
    ) -> String {
        let location: Location;
        match self.str.get(&name) {
            Some(loc) => location = loc.clone(),
            _ => panic!("Var str not found at {}:{}", block, line),
        }
        let heap_addr_bytes = memory.load(location);
        let heap_addr = u32::from_be_bytes(
            heap_addr_bytes
                .try_into()
                .expect("invalid heap addr len in get_string"),
        );

        let str_bytes = memory.heap_load(heap_addr);
        String::from_utf8_lossy(&str_bytes).to_string()
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
            let int: i32 = self.parse_i32(item, block, line);
            let byte: &[u8] = &int.to_be_bytes();
            for bit in byte {
                final_bytes.push(*bit);
            }
        }

        let location: Location = memory.store(&final_bytes);
        self.int.insert(name, location);
    }

    pub fn set_str_vec(&mut self, name: String, value: &str, memory: &mut Memory) {
        let items: &Vec<&str> = &value[1..value.len() - 2].split(',').collect::<Vec<&str>>();
        let mut heap_addrs_bytes: Vec<u8> = vec![];
        for item in items {
            let current_heap_addr = memory.malloc(item.as_bytes());
            let addr_bytes = current_heap_addr.to_be_bytes();
            for byte in addr_bytes {
                heap_addrs_bytes.push(byte)
            }
        }

        let heap_addr_addr = memory.malloc(&heap_addrs_bytes);
        let location: Location = memory.store(&heap_addr_addr.to_be_bytes());
        self.vec.insert(name, location);
    }
}
