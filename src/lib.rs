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
    pub string: String,
    pub lxstring: String,
    pub program_counter: u32,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            lx: 0,
            rv: 0,
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
            self.program_counter += 1;
            let statement = instructions[self.program_counter as usize].trim();
            let cmd: Vec<&str> = statement.split_whitespace().collect();
            change(self);
            println!("{}", self.lx);
            match cmd[1] {
                _ => {
                    println!("Cant recognize statement: {}", statement);
                    process::exit(1)
                }
            }
        }
    }
}

fn change(cpu: &mut CPU) {
    cpu.lx = 10;
}

pub fn execute_block(
    instructions: HashMap<String, Vec<String>>,
    run_label: &str,
    registers: CPU,
    memory: &mut Memory,
    vars: &mut Vars,
) {
    let mut line = 0i32;

    let run_block: Vec<String>;
    match instructions.get(run_label) {
        Some(value) => run_block = value.to_vec(),
        _ => {
            println!("label '{}' not found", run_label);
            process::exit(1);
        }
    }

    let mut local_registers = CPU {
        lx: registers.lx,
        rv: registers.rv,
        string: registers.string,
        lxstring: registers.lxstring,
        program_counter: registers.program_counter,
    };

    let block_ref = run_block.clone();

    for statement in run_block {
        let statement = statement.trim();
        line += 1;
        let cmd: Vec<&str> = statement.split_whitespace().collect();
        match cmd[0] {
            "dbg" => {
                println!("Stack: ");
                for i in memory.stack.chunks(8) {
                    println!("{:?}", i)
                }
                println!(
                    "{:#?} \n {:#?} \n HEAP: {:#?}",
                    vars, local_registers, memory.heap
                );

                println!("{:?}", block_ref);
            }
            "print" => print::print(memory, vars, &mut local_registers, cmd, run_label, line),
            "ram" => stack::ram(
                memory,
                vars,
                &mut local_registers,
                cmd,
                statement,
                run_label,
                line,
            ),
            "add" => operations::add::add(memory, vars, &mut local_registers, cmd, run_label, line),
            "div" => operations::div::div(memory, vars, &mut local_registers, cmd, run_label, line),
            "sub" => operations::sub::sub(memory, vars, &mut local_registers, cmd, run_label, line),
            "mul" => operations::mul::mul(memory, vars, &mut local_registers, cmd, run_label, line),
            "reset" => memory.reset_stack(),
            "pop" => memory.pop_stack(),
            "parse" => stdf::parse::parse(memory, vars, &mut local_registers, cmd, run_label, line),
            "rand" => stdf::rand::rand(
                memory,
                &mut local_registers,
                cmd,
                statement,
                run_label,
                line,
            ),
            "split" => stdf::string::split(
                memory,
                vars,
                &mut local_registers,
                cmd,
                statement,
                run_label,
                line,
            ),
            "concat" => {
                stdf::string::concat(memory, vars, &mut local_registers, cmd, run_label, line)
            }
            // "stdin" => stdfn::stdin(&mut local_vars, cmd, run_label, line),
            // "stdfs" => stdfn::stdfs(&mut local_vars, cmd, statement, run_label, line),
            // "sqr" => operations::sqr(&mut stack, cmd, &mut local_vars, run_label, line),
            // "sqrt" => operations::sqrt(&mut stack, cmd, &mut local_vars, run_label, line),
            // "round" => operations::round(&mut stack, cmd, &mut local_vars, run_label, line),
            // "split" => operations::split(cmd, statement, &mut local_vars, run_label, line),
            // "vec" => {
            //     operations::vec_ops(&mut stack, cmd, statement, &mut local_vars, run_label, line)
            // }
            "copy" => stack::copy(
                memory,
                vars,
                &mut local_registers,
                cmd,
                statement,
                run_label,
                line,
            ),
            "cmp" => operations::cmp::cmp(memory, vars, &mut local_registers, cmd, run_label, line),
            "je" => jump::je(
                memory,
                vars,
                local_registers.clone(),
                instructions.clone(),
                cmd,
                run_label,
                line,
            ),
            "jne" => jump::jne(
                memory,
                vars,
                local_registers.clone(),
                instructions.clone(),
                cmd,
                run_label,
                line,
            ),
            "jgr" => jump::jgr(
                memory,
                vars,
                local_registers.clone(),
                instructions.clone(),
                cmd,
                run_label,
                line,
            ),
            "jsm" => jump::jsm(
                memory,
                vars,
                local_registers.clone(),
                instructions.clone(),
                cmd,
                run_label,
                line,
            ),
            "jmp" => jump::jmp(
                memory,
                vars,
                local_registers.clone(),
                instructions.clone(),
                cmd,
                run_label,
                line,
            ),
            "halt" => process::exit(0),

            _ => {
                println!(
                    "Cant recognize command '{}' at '{}' line: {}",
                    cmd[0], run_label, line
                );
                process::exit(1)
            }
        }
    }
}
