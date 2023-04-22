use crate::{
    funcs::errors::ErrorKind,
    memory::{Location, Memory},
};
use rand::{distributions::Alphanumeric, Rng};
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

pub struct CastStack {
    pub dest_location: Location,
    pub src_data: Vec<u8>,
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
            if item == &"" {
                continue;
            }
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
            if item.len() == 0 {
                continue;
            }
            let current_heap_addr = memory.malloc(item[1..item.len() - 1].as_bytes());
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
                .load(destination.location)
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

    fn parse_type_str(&mut self, val: String, memory: &mut Memory) -> Result<Type, ErrorKind> {
        match val.parse::<i32>() {
            Ok(n) => {
                let name: String = rand::thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(10)
                    .map(char::from)
                    .collect();
                self.set_int(name.clone(), n.to_string().as_str(), memory)?;
                return Ok(self.get_type(name)?);
            }
            _ => {
                let quote: &str = &val[0..1];
                if quote == "\"" {
                    let extracted_str = &val[1..val.len() - 1];
                    let name: String = rand::thread_rng()
                        .sample_iter(&Alphanumeric)
                        .take(10)
                        .map(char::from)
                        .collect();
                    self.set_string(name.clone(), extracted_str, memory);
                    return Ok(self.get_type(name)?);
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
                let left = self.parse_type_str(key, memory)?;
                let right = self.parse_type_str(value, memory)?;
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
        let key_type = self.parse_type_str(key, memory)?;
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
            },
            _ => Err(ErrorKind::ExpectedMap(name))
        }
    }

    pub fn remove_from_map(
        &mut self,
        name: String,
        key: String,
        memory: &mut Memory,
    ) -> Result<(), ErrorKind> {
        let t: Type = self.get_type(name.clone())?;
        let key_type = self.parse_type_str(key, memory)?;
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
                for (i, item) in self.keys.iter().enumerate() {
                    if item.name == type_.name {
                        let key = memory.yeild_i32(item.location);
                        let i32_prop = memory.yeild_i32(type_.location);
                        if key == i32_prop {
                            return Ok(*self.values[i].clone());
                        }
                    }
                }
                return Err(ErrorKind::MapValueNotFound);
            }
            TypeName::String => {
                for (i, item) in self.keys.iter().enumerate() {
                    if item.name == type_.name {
                        let key = memory.yeild_string(item.location);
                        let prop = memory.yeild_string(type_.location);
                        if key == prop {
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
                for (i, item) in self.keys.iter_mut().enumerate() {
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
            TypeName::ButterFly(_) => unimplemented!(),
        }
    }
}
