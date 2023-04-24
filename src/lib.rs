pub mod funcs;
pub mod memory;
pub mod parser;

#[cfg(test)]
pub mod tests;
pub mod types;

use funcs::errors::ErrorKind;
use funcs::*;
use memory::Memory;
use types::Vars;

use std::collections::HashMap;
use std::process;

#[derive(Debug, Clone)]
pub struct CPU {
    pub lx: i32,
    pub rv: i32,
    pub string: String,
    pub lxstring: String,
    pub program_counter: i32,
    pub callstack: Vec<i32>,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            lx: 0,
            rv: 0,
            string: String::new(),
            lxstring: String::new(),
            program_counter: 0i32,
            callstack: vec![],
        }
    }

    pub fn execute(
        &mut self,
        instructions: Vec<String>,
        label_map: HashMap<String, isize>,
        memory: &mut Memory,
        vars: &mut Vars,
    ) {
        loop {
            let ret_val: Result<(), errors::ErrorKind>;
            let statement = instructions[self.program_counter as usize].trim();
            let cmd: Vec<&str> = statement.split_whitespace().collect();
            ret_val = match cmd[0] {
                "dbg" => {
                    println!("Stack: ");
                    for i in memory.stack.chunks(8) {
                        println!("{:?}", i)
                    }
                    println!("{:#?} \n {:#?} \n HEAP: {:#?} \n", vars, self, memory.heap);
                    println!("{:?}", instructions);
                    Ok(())
                }
                "print" => print::print(memory, vars, self, cmd),
                "ram" => stack::ram(memory, vars, self, cmd, statement),
                "add" => operations::add::add(memory, vars, self, cmd),
                "div" => operations::div::div(memory, vars, self, cmd),
                "sub" => operations::sub::sub(memory, vars, self, cmd),
                "mul" => operations::mul::mul(memory, vars, self, cmd),
                "reset" => {
                    memory.reset_stack();
                    Ok(())
                }
                "pop" => {
                    memory.pop_stack();
                    Ok(())
                }
                "parse" => stdf::parse::parse(memory, vars, self, cmd),
                "rand" => stdf::rand::rand(memory, self, cmd, statement),
                "split" => stdf::string::split(memory, vars, self, cmd, statement),
                "concat" => stdf::string::concat(memory, vars, self, cmd),
                "copy" => stack::copy(memory, vars, self, cmd, statement),
                "vec" => vec::vec(memory, vars, self, cmd, statement),
                "insert" => butterfly::map(memory, vars, cmd, statement),
                "delete" => butterfly::map(memory, vars, cmd, statement),
                "get" => butterfly::map(memory, vars, cmd, statement),
                "stdin" => stdf::stdin::stdin(memory, vars, self, cmd, statement),
                "stdfs" => stdf::stdfs::stdfs(memory, vars, self, cmd),

                "cmp" => operations::cmp::cmp(memory, vars, self, cmd),
                "jmp" => jump::jmp(self, cmd, label_map.clone()),
                "je" => jump::je(self, cmd, label_map.clone(), memory),
                "jgr" => jump::jgr(self, cmd, label_map.clone(), memory),
                "jsm" => jump::jsm(self, cmd, label_map.clone(), memory),
                "jne" => jump::jne(self, cmd, label_map.clone(), memory),
                "ret" => jump::ret(self),
                "halt" => process::exit(0),
                _ => {
                    println!("Cant recognize statement: {}", statement);
                    process::exit(1)
                }
            };

            match ret_val {
                Ok(()) => (),
                Err(e) => {
                    let label = errors::get_label(self.program_counter, label_map);
                    match e {
                        ErrorKind::ArgErr => {
                            println!("Invalid arguments at {} {}", label, statement)
                        }
                        ErrorKind::ParseInt => println!("Expected Int at {} {}", label, statement),
                        ErrorKind::StackLen => {
                            println!("Not enough items in stack at {} {}", label, statement)
                        }
                        ErrorKind::VarNotFound(var) => {
                            println!("Var '{}' not found at {} {}", var, label, statement)
                        }
                        ErrorKind::Casting { src, dest } => println!(
                            "Cannot cast '{}' to '{}' at {} {}",
                            src, dest, label, statement
                        ),
                        ErrorKind::ExpectedInt(var) => {
                            println!("Expected '{}' to be int at {} {}", var, label, statement)
                        }
                        ErrorKind::ExpectedVec(var) => {
                            println!("Expected '{}' to be int at {} {}", var, label, statement)
                        }
                        ErrorKind::ExpectedStr(var) => {
                            println!("Expected '{}' to be int at {} {}", var, label, statement)
                        }
                        ErrorKind::RangeNegative => println!(
                            "Range should be non-zero and positive at {} {}",
                            label, statement
                        ),
                        ErrorKind::EmptyCallstack => {
                            println!("Nowhere to return to at {} {}", label, statement)
                        }
                        ErrorKind::LabelNotFound(l) => {
                            println!("Label '{}' not found at {}: {}", l, label, statement)
                        }
                        ErrorKind::VecLen(name) => {
                            println!(
                                "'{}' dosent have enough length at {}: {}",
                                name, label, statement
                            )
                        }
                        ErrorKind::ExpectedMap(name) => {
                            println!(
                                "Expected '{}' to be a map at {}: {}",
                                name, label, statement
                            )
                        },
                        ErrorKind::MapValueNotFound => println!(
                            "Property Not found on map"
                        )
                    }
                    process::exit(1)
                }
            }
            self.program_counter += 1;
            if self.program_counter == instructions.len() as i32 {
                break;
            }
        }
    }
}
