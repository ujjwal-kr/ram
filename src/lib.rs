pub mod funcs;
pub mod memory;
pub mod parser;

#[cfg(test)]
pub mod tests;
pub mod types;

use funcs::*;
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
                "print" => print::print(memory, vars, self, cmd, "", 1),
                "ram" => stack::ram(memory, vars, self, cmd, statement, "", 1),
                "add" => operations::add::add(memory, vars, self, cmd, "", 1),
                "div" => operations::div::div(memory, vars, self, cmd, "", 1),
                "sub" => operations::sub::sub(memory, vars, self, cmd, "", 1),
                "mul" => operations::mul::mul(memory, vars, self, cmd, "", 1),
                "reset" => memory.reset_stack(),
                "pop" => memory.pop_stack(),
                "parse" => stdf::parse::parse(memory, vars, self, cmd, "", 1),
                "rand" => stdf::rand::rand(memory, self, cmd, statement, "", 1),
                "split" => stdf::string::split(memory, vars, self, cmd, statement, "", 1),
                "concat" => stdf::string::concat(memory, vars, self, cmd, "", 1),
                "copy" => stack::copy(memory, vars, self, cmd, statement, "", 1),
                "cmp" => operations::cmp::cmp(memory, vars, self, cmd, "", 1),

                "jmp" => jump::jmp(self, cmd, label_map.clone()),
                "je" => jump::je(self, cmd, label_map.clone(), memory),
                "jgr" => jump::je(self, cmd, label_map.clone(), memory),
                "jsm" => jump::je(self, cmd, label_map.clone(), memory),
                "jne" => jump::je(self, cmd, label_map.clone(), memory),
                "halt" => process::exit(0),
                _ => {
                    println!("Cant recognize statement: {}", statement);
                    process::exit(1)
                }
            }
            if !self.jmp {
                self.program_counter += 1;
            }
        }
    }
}