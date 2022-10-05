use rand::Rng;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Memory {
    pub stack: Vec<u8>,
    pub heap: HashMap<u32, Vec<u8>>,
    pub ret: Vec<u8>,
}

pub struct Location {
    pub start: usize,
    pub size: usize,
}

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

    pub fn yeild_int_vec(&mut self, location: Location) -> Vec<i32> {
        let addr_bytes = self.load(location);
        let heap_addr_bytes = &addr_bytes[1..addr_bytes.len() - 1];
        let heap_addr = u32::from_be_bytes(
            heap_addr_bytes
                .try_into()
                .expect("invalid vec int heap addr len"),
        );

        let mut final_vec: Vec<i32> = vec![];
        let vec_bytes = self.heap_load(heap_addr);
        for num_bytes in vec_bytes.chunks(4) {
            let num = i32::from_be_bytes(num_bytes.try_into().expect("Invalid vec int chunk len"));
            final_vec.push(num);
        }
        final_vec
    }

    pub fn yeild_str_vec(&mut self, location: Location) -> Vec<String> {
        let stack_addr_bytes = self.load(location);
        let heap_addr_bytes = &stack_addr_bytes[1..stack_addr_bytes.len() - 1];
        let heap_addr = u32::from_be_bytes(
            heap_addr_bytes
                .try_into()
                .expect("invalid vec str heap addr len"),
        );
        let mut final_vec: Vec<String> = vec![];
        let vec_addr_bytes = self.heap_load(heap_addr);
        for addr_bytes in vec_addr_bytes.chunks(4) {
            let str_addr =
                u32::from_be_bytes(addr_bytes.try_into().expect("invalid vec str heap str len"));
            let str_bytes = self.heap_load(str_addr);
            let final_str = String::from_utf8_lossy(&str_bytes);
            final_vec.push(final_str.into_owned());
        }
        final_vec
    }
}
