use std::collections::HashMap;
use std::io::prelude::*;
use std::{env, fs, io, process};

mod memory;
mod types;

mod funcs;
mod parser;
use memory::Memory;
use types::Vars;

// use funcs::{errors, jump, operations, print, stack, stdfn, var};
use funcs::{print, stack};

#[derive(Debug)]
pub struct Registers {
    pub lx: i32,
    pub rv: i32,
    pub string: String,
    pub lxstring: String,
}

fn main() -> std::io::Result<()> {
    let mut filename = String::new();
    if env::args().nth(1).is_none() {
        println!("Welcome to the Ram stack-based programming language.");
        println!("Please enter a filename: ");
        io::stdin().read_line(&mut filename)?;
        filename = filename.trim().to_string();
    } else {
        if env::args().nth(1).unwrap() == "test" {
        } else {
            filename = env::args().nth(1).unwrap();
        }
    }

    let mut file = fs::File::open(filename.trim())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let p_lines: Vec<&str> = contents.split("\n").collect();
    let program: HashMap<String, Vec<String>> = parser::parse_lines(p_lines); // returns final file with imports

    let registers = Registers {
        lx: 0,
        rv: 0,
        string: String::from(""),
        lxstring: String::from(""),
    };
    let mut memory: Memory = Memory::new();
    let mut vars: Vars = Vars::new();

    match execute_block(program, "main:", registers, &mut memory, &mut vars) {
        Ok(()) => (),
        _ => println!("Something went wrong"),
    }
    Ok(())
}

pub fn execute_block(
    program: HashMap<String, Vec<String>>,
    run_label: &str,
    registers: Registers,
    memory: &mut Memory,
    vars: &mut Vars,
) -> std::io::Result<()> {
    let mut line = 0i32;

    let run_block: Vec<String>;
    match program.get(run_label) {
        Some(value) => run_block = value.to_vec(),
        _ => {
            println!("label '{}' not found", run_label);
            process::exit(1);
        }
    }

    let mut local_registers = Registers {
        lx: registers.lx,
        rv: registers.rv,
        string: registers.string,
        lxstring: registers.lxstring,
    };

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
                )
            }
            "print" => print::print(
                memory,
                vars,
                &mut local_registers,
                cmd,
                statement,
                run_label,
                line,
            ),
            "ram" => stack::ram(
                memory,
                vars,
                &mut local_registers,
                cmd,
                statement,
                run_label,
                line,
            ),
            // "global_var" => var::global_var(
            //     &mut stack,
            //     cmd,
            //     statement,
            //     &mut local_vars,
            //     hash_vars,
            //     run_label,
            //     line,
            // ),
            // "var" => var::var(
            //     &mut stack,
            //     cmd,
            //     statement,
            //     &mut local_vars,
            //     hash_vars,
            //     run_label,
            //     line,
            // ),
            // "move" => var::movefn(cmd, &mut local_vars, hash_vars, run_label, line),
            // "str" => stack::strfn(&mut stack, &mut local_vars, cmd, run_label, line),
            // "stdin" => stdfn::stdin(&mut local_vars, cmd, run_label, line),
            // "stdfs" => stdfn::stdfs(&mut local_vars, cmd, statement, run_label, line),
            // "pop" => stack::pop(&mut stack, cmd, run_label, line),
            // // "popall" => stack = vec![],
            // "add" => operations::add(&mut stack, cmd, &mut local_vars, run_label, line),
            // "sub" => operations::sub(&mut stack, run_label, line),
            // "mul" => operations::mul(&mut stack, cmd, &mut local_vars, run_label, line),
            // "div" => operations::div(&mut stack, run_label, line),
            // "sqr" => operations::sqr(&mut stack, cmd, &mut local_vars, run_label, line),
            // "sqrt" => operations::sqrt(&mut stack, cmd, &mut local_vars, run_label, line),
            // "round" => operations::round(&mut stack, cmd, &mut local_vars, run_label, line),
            // "avg" => operations::avg(&mut stack, run_label, line),
            // "rand" => stdfn::random(&mut local_vars, cmd, &mut stack, statement, run_label, line),
            // "split" => operations::split(cmd, statement, &mut local_vars, run_label, line),
            // "parse" => stdfn::parse_int(&mut local_vars, cmd, run_label, line),
            // "vec" => {
            //     operations::vec_ops(&mut stack, cmd, statement, &mut local_vars, run_label, line)
            // }
            // "cmp" => operations::cmp(&mut stack, run_label, line),
            // "je" => jump::je(
            //     &mut stack,
            //     cmd,
            //     program.clone(),
            //     local_vars.clone(),
            //     hash_vars,
            //     run_label,
            //     line,
            // ),
            // "jne" => jump::jne(
            //     &mut stack,
            //     cmd,
            //     program.clone(),
            //     local_vars.clone(),
            //     hash_vars,
            //     run_label,
            //     line,
            // ),
            // "jgr" => jump::jgr(
            //     &mut stack,
            //     cmd,
            //     program.clone(),
            //     local_vars.clone(),
            //     hash_vars,
            //     run_label,
            //     line,
            // ),
            // "jsm" => jump::jsm(
            //     &mut stack,
            //     cmd,
            //     program.clone(),
            //     local_vars.clone(),
            //     hash_vars,
            //     run_label,
            //     line,
            // ),
            // "jmp" => jump::jmp(
            //     &mut stack,
            //     cmd,
            //     program.clone(),
            //     local_vars.clone(),
            //     hash_vars,
            //     run_label,
            //     line,
            // ),
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
    Ok(())
}
