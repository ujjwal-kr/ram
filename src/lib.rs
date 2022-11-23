pub mod funcs;
pub mod memory;
pub mod parser;

#[cfg(test)]
pub mod tests;
pub mod types;

use funcs::*;
use funcs::errors::ErrorKind;
use memory::Memory;
use types::Vars;

use std::collections::HashMap;
use std::process;

#[derive(Debug, Clone)]
pub struct CPU {
    pub lx: i32,
    pub rv: i32,
    pub jmp: bool,
    pub string: String,
    pub lxstring: String,
    pub program_counter: u32,
    pub callstack: Vec<u32>
}

impl CPU {
    pub fn new() -> Self {
        Self {
            lx: 0,
            rv: 0,
            jmp: false,
            string: String::new(),
            lxstring: String::new(),
            program_counter: 0u32,
            callstack: vec![]
        }
    }

    pub fn execute(
        &mut self,
        instructions: Vec<String>,
        label_map: HashMap<String, usize>,
        memory: &mut Memory,
        vars: &mut Vars,
    ) {
        loop {
            let ret_val: Result<(), errors::ErrorKind>;
            let statement = instructions[self.program_counter as usize].trim();
            let cmd: Vec<&str> = statement.split_whitespace().collect();
            self.jmp = false;
            match cmd[0] {
                "dbg" => {
                    println!("Stack: ");
                    for i in memory.stack.chunks(8) {
                        println!("{:?}", i)
                    }
                    println!("{:#?} \n {:#?} \n HEAP: {:#?} \n", vars, self, memory.heap);
                    println!("{:?}", instructions)
                }
                "print" => ret_val = print::print(memory, vars, self, cmd),
                "ram" => ret_val = stack::ram(memory, vars, self, cmd, statement),
                "add" => ret_val = operations::add::add(memory, vars, self, cmd),
                "div" => ret_val = operations::div::div(memory, vars, self, cmd),
                "sub" => ret_val = operations::sub::sub(memory, vars, self, cmd),
                "mul" => ret_val = operations::mul::mul(memory, vars, self, cmd),
                "reset" => memory.reset_stack(),
                "pop" => memory.pop_stack(),
                "parse" => stdf::parse::parse(memory, vars, self, cmd, "", 1),
                "rand" => stdf::rand::rand(memory, self, cmd, statement, "", 1),
                "split" => stdf::string::split(memory, vars, self, cmd, statement, "", 1),
                "concat" => stdf::string::concat(memory, vars, self, cmd, "", 1),
                "copy" => ret_val = stack::copy(memory, vars, self, cmd, statement),

                "cmp" => ret_val = operations::cmp::cmp(memory, vars, self, cmd),
                "jmp" => ret_val = jump::jmp(self, cmd, label_map.clone()),
                "je" => ret_val = jump::je(self, cmd, label_map.clone(), memory),
                "jgr" => ret_val = jump::je(self, cmd, label_map.clone(), memory),
                "jsm" => ret_val = jump::je(self, cmd, label_map.clone(), memory),
                "jne" => ret_val = jump::je(self, cmd, label_map.clone(), memory),
                "ret" => ret_val = jump::ret(self),
                "halt" => process::exit(0),
                _ => {
                    println!("Cant recognize statement: {}", statement);
                    process::exit(1)
                }
            }

            match ret_val {
                Ok(()) => (),
                Err(t) => match t {
                    ErrorKind::ArgErr => (),
                    ErrorKind::ParseInt => (),
                    ErrorKind::StackLen => (),
                    ErrorKind::VarNotFound(var) => (),
                    ErrorKind::Casting{src, dest} => (),
                    ErrorKind::ExpectedInt(var) => (),
                    ErrorKind::ExpectedVec(var) => (),
                    ErrorKind::ExpectedStr(var) => ()
                },
            }
            if !self.jmp {
                self.program_counter += 1;
            }
        }
    }
}