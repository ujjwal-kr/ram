use crate::{
    funcs::errors::ErrorKind,
    memory::{Location, Memory},
};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Vars(HashMap<String, Type>);

#[derive(Clone, Debug)]
pub struct Type {
    pub name: TypeName,
    pub location: Location,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeName {
    I32,
    String,
    Vector(Vector),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Vector {
    String,
    Int,
    // Vector(Box<Vector>),
}

pub struct CastStack {
    pub dest_location: Location,
    pub src_data: Vec<u8>,
}

impl Vars {
    pub fn new() -> Self {
        Vars { 0: HashMap::new() }
    }

    pub fn get_type(&mut self, name: String) -> Result<Type, ErrorKind> {
        match self.0.get(&name) {
            Some(ty) => Ok(ty.clone()),
            _ => Err(ErrorKind::VarNotFound(name)),
        }
    }

    // Integers

    pub fn set_int(
        &mut self,
        name: String,
        value: &str,
        memory: &mut Memory,
    ) -> Result<(), ErrorKind> {
        let int_bytes = self.parse_i32(value)?.to_be_bytes();
        let location: Location = memory.store(&int_bytes);
        let new_int = Type {
            name: TypeName::I32,
            location,
        };
        self.0.insert(name, new_int);
        Ok(())
    }

    pub fn set_int_to_stack(&mut self, memory: &mut Memory, value: &str) -> Result<(), ErrorKind> {
        let num = self.parse_i32(value)?;
        let bytes = num.to_be_bytes();
        memory.store(&bytes);
        Ok(())
    }

    // Strings

    pub fn set_string(&mut self, name: String, value: &str, memory: &mut Memory) {
        let heap_addr = memory.malloc(value.as_bytes());
        let location = memory.store(&heap_addr.to_be_bytes());
        let new_string = Type {
            name: TypeName::String,
            location,
        };
        self.0.insert(name, new_string);
    }

    // vectors

    pub fn set_int_vec(
        &mut self,
        name: String,
        value: &str,
        memory: &mut Memory,
    ) -> Result<(), ErrorKind> {
        let items: &Vec<&str> = &value[1..value.len() - 1].split(',').collect::<Vec<&str>>();
        let mut final_bytes: Vec<u8> = vec![];
        for item in items {
            let int: i32 = self.parse_i32(item.trim())?;
            let byte: &[u8] = &int.to_be_bytes();
            for bit in byte {
                final_bytes.push(*bit);
            }
        }
        let heap_addr: u32 = memory.malloc(&final_bytes);
        let location: Location = memory.store(&heap_addr.to_be_bytes());
        let new_int_vec = Type {
            name: TypeName::Vector(Vector::Int),
            location,
        };

        self.0.insert(name, new_int_vec);
        Ok(())
    }

    pub fn set_str_vec(&mut self, name: String, value: &str, memory: &mut Memory) {
        let items: &Vec<&str> = &value[1..value.len() - 1].split(',').collect::<Vec<&str>>();
        let mut heap_addrs_bytes: Vec<u8> = vec![];
        for item in items {
            let item = item.trim();
            let current_heap_addr = memory.malloc(&item[1..item.len() - 1].as_bytes());
            let addr_bytes = current_heap_addr.to_be_bytes();
            for byte in addr_bytes {
                heap_addrs_bytes.push(byte)
            }
        }

        let heap_addr_addr = memory.malloc(&heap_addrs_bytes);
        let location: Location = memory.store(&heap_addr_addr.to_be_bytes());
        let new_str_vec: Type = Type {
            name: TypeName::Vector(Vector::String),
            location,
        };
        self.0.insert(name, new_str_vec);
    }

    pub fn set_raw_str_vec(&mut self, name: String, items: Vec<&str>, memory: &mut Memory) {
        let mut heap_addrs_bytes: Vec<u8> = vec![];
        for item in items {
            let current_heap_addr = memory.malloc(item.trim().as_bytes());
            let addr_bytes = current_heap_addr.to_be_bytes();
            for byte in addr_bytes {
                heap_addrs_bytes.push(byte)
            }
        }

        let heap_addr_addr = memory.malloc(&heap_addrs_bytes);
        let location: Location = memory.store(&heap_addr_addr.to_be_bytes());
        let new_str_vec: Type = Type {
            name: TypeName::Vector(Vector::String),
            location,
        };
        self.0.insert(name, new_str_vec);
    }

    // Casting stuff

    pub fn cast(
        &mut self,
        src: &str,
        dest: &str,
        memory: &mut Memory,
    ) -> Result<Option<CastStack>, ErrorKind> {
        // check if the type of both vars are same
        let source: Type;
        let destination: Type;
        match self.0.get(src) {
            Some(s) => source = s.clone(),
            _ => panic!("Var {} not found", src),
        }
        match self.0.get(dest) {
            Some(d) => destination = d.clone(),
            _ => panic!("Var {} not found", src),
        }
        if destination.name != source.name {
            return Err(ErrorKind::Casting {
                src: src.to_string(),
                dest: dest.to_string(),
            });
        }
        if destination.name == TypeName::String
            || destination.name == TypeName::Vector(Vector::String)
        {
            let src_addr: [u8; 4] = memory
                .load(source.location)
                .try_into()
                .expect("Error converting location to addr");
            let src_heap_addr = u32::from_be_bytes(src_addr);
            let src_heap_data = memory.heap_load(src_heap_addr);

            let dest_addr: [u8; 4] = memory
                .load(destination.location.clone())
                .try_into()
                .expect("Error converting location to addr");
            memory.heap_mod(u32::from_be_bytes(dest_addr), &src_heap_data);
            Ok(None)
        } else {
            let data: &[u8] = memory.load(source.location);
            Ok(Some(CastStack {
                dest_location: destination.location,
                src_data: data.to_vec(),
            }))
        }
    }
}

impl Vars {
    // parsers

    fn parse_i32(&mut self, value: &str) -> Result<i32, ErrorKind> {
        let num: i32;
        match value.parse::<i32>() {
            Ok(n) => num = n,
            _parse_int_error => return Err(ErrorKind::ParseInt),
        }
        Ok(num)
    }
}
