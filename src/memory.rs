use rand::Rng;
use std::collections::HashMap;

use crate::funcs::errors::ErrorKind;

#[derive(Debug)]
pub struct Memory {
    pub stack: Vec<u8>,
    pub heap: HashMap<u32, Vec<u8>>,
}

#[derive(Clone, Debug)]
pub struct Location {
    pub start: usize,
    pub size: usize,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            stack: vec![],
            heap: HashMap::from([(0, vec![0, 0, 0, 0])]),
        }
    }

    // stack operations

    pub fn pop_stack(&mut self) {
        self.stack.pop();
    }

    pub fn reset_stack(&mut self) {
        self.stack = vec![];
    }

    pub fn load(&mut self, location: Location) -> &[u8] {
        &self.stack[location.start..location.start + location.size] as _
    }

    pub fn store(&mut self, val: &[u8]) -> Location {
        let address = self.stack.len();
        for byte in val {
            self.stack.push(*byte);
        }

        Location {
            start: address,
            size: val.len(),
        }
    }

    pub fn stack_mod(&mut self, location: Location, data: &[u8]) {
        if data.len() != location.size {
            panic!("Illegal memory modification");
        }
        self.stack.splice(
            location.start..location.start + location.size,
            data.iter().cloned(),
        );
    }

    // heap operations

    pub fn malloc(&mut self, bytes: &[u8]) -> u32 {
        let addr_bytes = rand::thread_rng().gen::<[u8; 4]>();
        let addr: u32 = u32::from_be_bytes(addr_bytes);
        self.heap.insert(addr, bytes.to_vec());
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

    pub fn heap_mod(&mut self, addr: u32, data: &[u8]) {
        self.free(addr);
        self.heap.insert(addr, data.to_vec());
    }

    // stack yeilds

    // ints

    pub fn get_int_from_stack(&mut self) -> Result<i32, ErrorKind> {
        if self.stack.len() < 4 {
            return Err(ErrorKind::StackLen);
        }
        let location = Location {
            start: self.stack.len() - 4,
            size: 4,
        };
        let bytes = self.load(location);
        let num: i32 = i32::from_be_bytes(bytes.try_into().expect("invalid i32 len"));
        Ok(num)
    }

    pub fn set_int_to_stack(&mut self, value: i32) {
        let bytes = value.to_be_bytes();
        self.store(&bytes);
    }

    pub fn yeild_i32(&mut self, location: Location) -> i32 {
        let bytes = self.load(location);
        let num: i32 = i32::from_be_bytes(bytes.try_into().expect("invalid i32 len"));
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
        let heap_addr = u32::from_be_bytes(
            addr_bytes
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
        let addr_bytes = self.load(location);
        let heap_addr = u32::from_be_bytes(
            addr_bytes
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

// vec ops

impl Memory {
    // push

    pub fn vec_int_push(&mut self, heap_bytes: &[u8], value: [u8; 4]) {
        let heap_value = &mut *self
            .heap
            .get_mut(&u32::from_be_bytes(
                heap_bytes.try_into().expect("Illegal heap pointer"),
            ))
            .expect("Illegal heap pointer");

        for byte in value {
            heap_value.push(byte)
        }
    }

    pub fn vec_str_push(&mut self, heap_bytes: &[u8], new_str_addr: &[u8; 4]) {
        let heap_value = &mut *self
            .heap
            .get_mut(&u32::from_be_bytes(
                heap_bytes.try_into().expect("Illegal heap pointer"),
            ))
            .expect("Illegal heap pointer");

        for byte in new_str_addr {
            heap_value.push(*byte)
        }
    }

    // modify using index

    pub fn mod_vec_int(&mut self, heap_bytes: &[u8], idx: usize, value: &[u8]) -> Result<(), ()> {
        let heap_value = &mut *self
            .heap
            .get_mut(&u32::from_be_bytes(
                heap_bytes.try_into().expect("Illegal heap pointer"),
            ))
            .expect("Illegal heap pointer");

        if idx < heap_value.len() / 4 {
            heap_value.splice(idx * 4..idx * 4 + 4, value.to_vec());
        } else {
            return Err(());
        }
        Ok(())
    }

    pub fn mod_vec_str(
        &mut self,
        heap_bytes: &[u8],
        idx: usize,
        str_addr: &[u8],
    ) -> Result<(), ()> {
        let heap_value = &mut *self
            .heap
            .get_mut(&u32::from_be_bytes(
                heap_bytes.try_into().expect("Illegal heap pointer"),
            ))
            .expect("Illegal heap pointer");

        if idx < heap_value.len() / 4 {
            let str_addr_bytes = &heap_value[idx * 4..idx * 4 + 4];
            let old_str_addr =
                u32::from_be_bytes(str_addr_bytes.try_into().expect("invalid heap addr"));
            println!("{:?}", str_addr.to_vec());
            heap_value.splice(idx * 4..idx * 4 + 4, str_addr.to_vec());
            self.free(old_str_addr);
        } else {
            return Err(());
        }
        Ok(())
    }

    // pop

    pub fn pop_vec_int(&mut self, heap_bytes: &[u8]) {
        let heap_value = &mut *self
            .heap
            .get_mut(&u32::from_be_bytes(
                heap_bytes.try_into().expect("Illegal heap pointer"),
            ))
            .expect("Illegal heap pointer");

        let sub = heap_value.len().saturating_sub(4);
        heap_value.truncate(sub);
    }

    pub fn pop_vec_str(&mut self, heap_bytes: &[u8]) {
        let heap_value = &mut *self
            .heap
            .get_mut(&u32::from_be_bytes(
                heap_bytes.try_into().expect("Illegal heap pointer"),
            ))
            .expect("Illegal heap pointer");

        let str_addr_bytes = heap_value
            .drain(heap_value.len() - 4..heap_value.len())
            .collect::<Vec<u8>>();
        let str_addr = u32::from_be_bytes(str_addr_bytes.try_into().expect("invalid heap addr"));
        self.free(str_addr);
    }
}
