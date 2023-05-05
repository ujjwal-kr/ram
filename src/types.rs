use crate::{
    funcs::errors::ErrorKind,
    memory::{Location, Memory},
};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Vars(HashMap<String, Type>);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Type {
    pub name: TypeName,
    pub location: Location,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeName {
    I32,
    String,
    Vector(Vector),
    ButterFly(ButterFly),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ButterFly {
    keys: Vec<Box<Type>>,   // left wing
    values: Vec<Box<Type>>, // right wing
    pub length: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Vector {
    String,
    Int,
}

impl Vars {
    pub fn new() -> Self {
        Vars(HashMap::new())
    }

    pub fn get_type(&mut self, name: String) -> Result<Type, ErrorKind> {
        match self.0.get(&name) {
            Some(ty) => Ok(ty.clone()),
            _ => Err(ErrorKind::VarNotFound(name)),
        }
    }

    fn parse_i32(&mut self, value: &str) -> Result<i32, ErrorKind> {
        let num: i32;
        match value.parse::<i32>() {
            Ok(n) => num = n,
            _parse_int_error => return Err(ErrorKind::ParseInt),
        }
        Ok(num)
    }

    // Integers

    pub fn set_int(
        &mut self,
        name: String,
        value: &str,
        memory: &mut Memory,
    ) -> Result<(), ErrorKind> {
        let int_bytes = self.parse_i32(value)?.to_be_bytes();
        match self.0.get(&name) {
            Some(old) => {
                memory.stack_mod(old.location, &int_bytes)
            }
            _ => {
                let location: Location = memory.store(&int_bytes);
                let new_int = Type {
                    name: TypeName::I32,
                    location,
                };
                self.0.insert(name, new_int);
            }
        }

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
        match self.0.get(&name) {
            Some(old) => {
                old.free_heap(memory);
                memory.stack_mod(old.location, &heap_addr.to_be_bytes());
            }
            _ => {
                let location = memory.store(&heap_addr.to_be_bytes());
                let new_string = Type {
                    name: TypeName::String,
                    location,
                };
                self.0.insert(name, new_string);
            }
        };
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
            if item == &"" {
                continue;
            }
            let int: i32 = self.parse_i32(item.trim())?;
            let byte: &[u8] = &int.to_be_bytes();
            for bit in byte {
                final_bytes.push(*bit);
            }
        }
        let heap_addr = memory.malloc(&final_bytes).to_be_bytes();
        match self.0.get(&name) {
            Some(old) => {
                old.free_heap(memory);
                memory.stack_mod(old.location, &heap_addr);
            }
            _ => {
                let location: Location = memory.store(&heap_addr);
                let new_int_vec = Type {
                    name: TypeName::Vector(Vector::Int),
                    location,
                };
                self.0.insert(name, new_int_vec);
            }
        }

        Ok(())
    }

    pub fn set_str_vec(&mut self, name: String, value: &str, memory: &mut Memory) {
        let items: &Vec<&str> = &value[1..value.len() - 1].split(',').collect::<Vec<&str>>();
        let mut heap_addrs_bytes: Vec<u8> = vec![];
        for item in items {
            let item = item.trim();
            if item.len() == 0 {
                continue;
            }
            let current_heap_addr = memory.malloc(item[1..item.len() - 1].as_bytes());
            let addr_bytes = current_heap_addr.to_be_bytes();
            for byte in addr_bytes {
                heap_addrs_bytes.push(byte)
            }
        }
        let heap_addr_addr = memory.malloc(&heap_addrs_bytes).to_be_bytes();
        match self.0.get(&name) {
            Some(old) => {
                old.free_str_vec(memory);
                memory.stack_mod(old.location, &heap_addr_addr);
            }
            _ => {
                let location: Location = memory.store(&heap_addr_addr);
                let new_str_vec: Type = Type {
                    name: TypeName::Vector(Vector::String),
                    location,
                };
                self.0.insert(name, new_str_vec);
            }
        }
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
        let heap_addr_addr = memory.malloc(&heap_addrs_bytes).to_be_bytes();
        match self.0.get(&name) {
            Some(old) => {
                old.free_str_vec(memory);
                memory.stack_mod(old.location, &heap_addr_addr);
            }
            _ => {
                let location: Location = memory.store(&heap_addr_addr);
                let new_str_vec: Type = Type {
                    name: TypeName::Vector(Vector::String),
                    location,
                };
                self.0.insert(name, new_str_vec);
            }
        }
    }

    pub fn set_owned_raw_str_vec(&mut self, name: String, items: Vec<String>, memory: &mut Memory) {
        let mut heap_addrs_bytes: Vec<u8> = vec![];
        for item in items {
            let current_heap_addr = memory.malloc(item.trim().as_bytes());
            let addr_bytes = current_heap_addr.to_be_bytes();
            for byte in addr_bytes {
                heap_addrs_bytes.push(byte)
            }
        }
        let heap_addr_addr = memory.malloc(&heap_addrs_bytes).to_be_bytes();
        match self.0.get(&name) {
            Some(old) => {
                old.free_str_vec(memory);
                memory.stack_mod(old.location, &heap_addr_addr);
            }
            _ => {
                let location: Location = memory.store(&heap_addr_addr);
                let new_str_vec: Type = Type {
                    name: TypeName::Vector(Vector::String),
                    location,
                };
                self.0.insert(name, new_str_vec);
            }
        }
    }

    // Casting stuff

    pub fn cast(&mut self, src: &str, dest: &str, memory: &mut Memory) -> Result<(), ErrorKind> {
        let source = self.get_type(src.to_string())?;
        let destination = self.get_type(dest.to_string())?;
        if destination.name != source.name {
            return Err(ErrorKind::Casting {
                src: src.to_string(),
                dest: dest.to_string(),
            });
        }
        let src_addr: [u8; 4] = memory.load(source.location).try_into().unwrap();
        if destination.name == TypeName::I32 {
            memory.stack_mod(destination.location, &src_addr);
            return Ok(());
        }
        let src_heap_addr = u32::from_be_bytes(src_addr);
        let src_heap_data = memory.heap_load(src_heap_addr);
        let dest_addr: [u8; 4] = memory.load(destination.location).try_into().unwrap();
        match destination.name {
            TypeName::String => memory.heap_mod(u32::from_be_bytes(dest_addr), &src_heap_data),
            TypeName::Vector(Vector::String) => {
                destination.free_str_vec(memory);
                memory.free(u32::from_be_bytes(dest_addr));
                let new_vec = memory.yeild_str_vec(source.location);
                self.set_owned_raw_str_vec(dest.to_string(), new_vec, memory);
            }
            TypeName::Vector(Vector::Int) => {
                memory.heap_mod(u32::from_be_bytes(dest_addr), &src_heap_data)
            }
            _ => panic!("Cast performed on illegal items"),
        }
        Ok(())
    }

    // drop

    pub fn drop(&mut self, name: String, memory: &mut Memory) -> Result<(), ErrorKind> {
        let t = self.get_type(name.clone())?;
        match t.name {
            TypeName::I32 => {}
            TypeName::String => t.free_heap(memory),
            TypeName::Vector(Vector::String) => t.free_str_vec(memory),
            TypeName::Vector(Vector::Int) => t.free_heap(memory),
            TypeName::ButterFly(b) => b.clone().free_butterfly(b.keys, b.values, memory),
        }
        self.0.remove(&name);
        Ok(())
    }
}

impl Type {
    pub fn free_str_vec(&self, memory: &mut Memory) {
        if self.name != TypeName::Vector(Vector::String) {
            panic!("free_str_vec called on illegal type")
        }
        let heap_addr = memory.load(self.location).to_owned();
        let data = memory.heap_load(u32::from_be_bytes(heap_addr.clone().try_into().unwrap()));
        if data.len() >= 4 {
            for bytes in data.chunks(4) {
                let str_addr = u32::from_be_bytes(bytes.try_into().unwrap());
                memory.free(str_addr)
            }
        }
        memory.free(u32::from_be_bytes(heap_addr.try_into().unwrap()))
    }

    pub fn free_heap(&self, memory: &mut Memory) {
        let str_addr = memory.load(self.location).to_owned();
        memory.free(u32::from_be_bytes(str_addr.try_into().unwrap()));
    }
}

impl Vars {
    // return by index

    pub fn vec_str_item(
        &mut self,
        location: Location,
        name: String,
        idx: usize,
        memory: &mut Memory,
    ) -> Result<String, ErrorKind> {
        let str_vec = memory.yeild_str_vec(location);
        if idx >= str_vec.len() {
            return Err(ErrorKind::VecLen(name));
        }
        Ok(str_vec[idx].clone())
    }

    pub fn vec_int_item(
        &mut self,
        location: Location,
        name: String,
        idx: usize,
        memory: &mut Memory,
    ) -> Result<i32, ErrorKind> {
        let int_vec = memory.yeild_int_vec(location);
        if idx >= int_vec.len() {
            return Err(ErrorKind::VecLen(name));
        }
        Ok(int_vec[idx])
    }

    // return length

    pub fn vec_int_len(&mut self, location: Location, memory: &mut Memory) -> i32 {
        let int_vec = memory.yeild_int_vec(location);
        int_vec.len() as i32
    }

    pub fn vec_str_len(&mut self, location: Location, memory: &mut Memory) -> i32 {
        let str_vec = memory.yeild_str_vec(location);
        str_vec.len() as i32
    }

    // mem mods

    pub fn get_vec_int_mod(&mut self, t: Type, value: i32, memory: &mut Memory) -> VecMod {
        let value_bytes = value.to_be_bytes();
        let heap_addr = memory.load(t.location).to_vec();
        VecMod {
            value_bytes,
            heap_addr,
        }
    }

    pub fn get_vec_str_mod(&mut self, t: Type, value: &str, memory: &mut Memory) -> VecMod {
        let heap_addr = memory.load(t.location).to_vec();
        let value_bytes = memory.malloc(value.as_bytes()).to_be_bytes();
        VecMod {
            heap_addr,
            value_bytes,
        }
    }
}

pub struct VecMod {
    pub value_bytes: [u8; 4],
    pub heap_addr: Vec<u8>,
}

// butterfly
impl Vars {
    pub fn add_map(&mut self, name: String) {
        let butterfly: ButterFly = ButterFly::new();
        let t: Type = Type {
            name: TypeName::ButterFly(butterfly),
            location: Location { start: 0, size: 0 },
        };
        self.0.insert(name, t);
    }

    fn parse_type_str(
        &mut self,
        name: String,
        val: String,
        key: bool,
        memory: &mut Memory,
    ) -> Result<Type, ErrorKind> {
        let val_name: String;
        match val.parse::<i32>() {
            Ok(n) => {
                if key {
                    val_name = format!("{}.n{}{}", name.clone().trim(), "k", val.trim());
                } else {
                    val_name = format!("{}.n{}{}", name.clone().trim(), "v", val.trim());
                }
                self.set_int(val_name.clone(), n.to_string().as_str(), memory)?;
                return Ok(self.get_type(val_name)?);
            }
            _ => {
                let quote: &str = &val[0..1];
                if quote == "\"" || quote == "'" {
                    if key {
                        val_name = format!("{}.{}{}", name.clone().trim(), "k", val.trim())
                    } else {
                        val_name = format!("{}.{}{}", name.clone().trim(), "v", val.trim())
                    }
                    let extracted_str = &val[1..val.len() - 1];
                    self.set_string(val_name.clone(), extracted_str, memory);
                    return Ok(self.get_type(val_name)?);
                } else {
                    return Ok(self.get_type(val)?);
                }
            }
        }
    }

    pub fn insert_to_map(
        &mut self,
        name: String,
        key: String,
        value: String,
        memory: &mut Memory,
    ) -> Result<(), ErrorKind> {
        let t: Type = self.get_type(name.clone())?;
        match t.name {
            TypeName::ButterFly(_) => {
                let left = self.parse_type_str(name.clone(), key, true, memory)?;
                let right = self.parse_type_str(name.clone(), value, false, memory)?;
                match self.0.get_mut(&name) {
                    Some(t) => match &mut t.name {
                        TypeName::ButterFly(b) => b.insert(left, right),
                        _ => return Err(ErrorKind::ExpectedMap(name)),
                    },
                    None => return Err(ErrorKind::ExpectedMap(name)),
                };
                Ok(())
            }
            _ => return Err(ErrorKind::ExpectedMap(name)),
        }
    }

    pub fn get_from_map(
        &mut self,
        name: String,
        key: String,
        memory: &mut Memory,
    ) -> Result<Type, ErrorKind> {
        let t: Type = self.get_type(name.clone())?;
        let key_type = self.parse_type_str(name.clone(), key, true, memory)?;
        match t.name {
            TypeName::ButterFly(mut butterfly) => butterfly.get(key_type, memory),
            _ => Err(ErrorKind::ExpectedMap(name)),
        }
    }

    pub fn replace_map(&mut self, name: String, assigned: Type) -> Result<(), ErrorKind> {
        match assigned.name {
            TypeName::ButterFly(_) => {
                self.0.remove(&name);
                self.0.insert(name, assigned);
                Ok(())
            }
            _ => Err(ErrorKind::ExpectedMap(name)),
        }
    }

    pub fn remove_from_map(
        &mut self,
        name: String,
        key: String,
        memory: &mut Memory,
    ) -> Result<(), ErrorKind> {
        let t: Type = self.get_type(name.clone())?;
        let key_type = self.parse_type_str(name.clone(), key, true, memory)?;
        match t.name {
            TypeName::ButterFly(_) => match self.0.get_mut(&name) {
                Some(t) => match &mut t.name {
                    TypeName::ButterFly(b) => b.delete(key_type, memory),
                    _ => return Err(ErrorKind::ExpectedMap(name)),
                },
                _ => return Err(ErrorKind::ExpectedMap(name)),
            },
            _ => Err(ErrorKind::ExpectedMap(name)),
        }
    }
}

impl ButterFly {
    pub fn new() -> Self {
        Self {
            keys: vec![],
            values: vec![],
            length: 0,
        }
    }

    pub fn insert(&mut self, left: Type, right: Type) {
        self.keys.push(Box::new(left));
        self.values.push(Box::new(right));
        self.length += 1;
    }

    pub fn get(&mut self, type_: Type, memory: &mut Memory) -> Result<Type, ErrorKind> {
        match type_.name {
            TypeName::I32 => {
                for (i, item) in self.keys.iter().enumerate().rev() {
                    if item.name == type_.name {
                        let key = memory.yeild_i32(item.location);
                        let i32_prop = memory.yeild_i32(type_.location);
                        if key == i32_prop {
                            let sub = memory.stack.len().saturating_sub(4);
                            memory.stack.truncate(sub);
                            return Ok(*self.values[i].clone());
                        }
                    }
                }
                return Err(ErrorKind::MapValueNotFound);
            }
            TypeName::String => {
                for (i, item) in self.keys.iter().enumerate().rev() {
                    if item.name == type_.name {
                        let key = memory.yeild_string(item.location);
                        let prop = memory.yeild_string(type_.location);
                        if key == prop {
                            type_.free_heap(memory);
                            return Ok(*self.values[i].clone());
                        }
                    }
                }
                return Err(ErrorKind::MapValueNotFound);
            }
            TypeName::Vector(_) => unimplemented!(),
            TypeName::ButterFly(_) => unimplemented!(),
        }
    }

    pub fn delete(&mut self, type_: Type, memory: &mut Memory) -> Result<(), ErrorKind> {
        match type_.name {
            TypeName::I32 => {
                for (i, item) in self.keys.iter().enumerate() {
                    if item.name == type_.name {
                        let key = memory.yeild_i32(item.location);
                        let i32_prop = memory.yeild_i32(type_.location);
                        if key == i32_prop {
                            self.keys.remove(i);
                            self.values.remove(i);
                            self.length -= 1;
                            return Ok(());
                        }
                    }
                }
                return Err(ErrorKind::MapValueNotFound);
            }
            TypeName::String => {
                for (i, item) in self.clone().keys.iter().enumerate() {
                    if item.name == type_.name {
                        let key = memory.yeild_string(item.location);
                        let prop = memory.yeild_string(type_.location);
                        if key == prop {
                            self.keys.remove(i);
                            self.values.remove(i);
                            self.length -= 1;
                            return Ok(());
                        }
                    }
                }
                return Err(ErrorKind::MapValueNotFound);
            }
            TypeName::Vector(_) => unimplemented!(),
            TypeName::ButterFly(butterfly_type) => {
                for (i, item) in self.keys.iter_mut().enumerate() {
                    match item.name.clone() {
                        TypeName::ButterFly(b) => {
                            if b == butterfly_type {
                                self.keys.remove(i);
                                self.values.remove(i);
                                self.length -= 1;
                                return Ok(());
                            }
                        }
                        _ => (),
                    }
                }
                return Err(ErrorKind::MapValueNotFound);
            }
        }
    }
}

impl ButterFly {
    pub fn free_butterfly(
        &mut self,
        left: Vec<Box<Type>>,
        right: Vec<Box<Type>>,
        memory: &mut Memory,
    ) {
        for item in right.iter() {
            match item.name.clone() {
                TypeName::ButterFly(b) => self.free_butterfly(b.keys, b.values, memory),
                TypeName::Vector(Vector::String) => item.free_str_vec(memory),
                TypeName::I32 => {}
                _ => item.free_heap(memory),
            }
        }

        for item in left.iter() {
            match item.name.clone() {
                TypeName::ButterFly(b) => self.free_butterfly(b.keys, b.values, memory),
                TypeName::Vector(Vector::String) => item.free_str_vec(memory),
                TypeName::I32 => {}
                _ => item.free_heap(memory),
            }
        }
    }
}
