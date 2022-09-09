use std::collections::HashMap;
use std::io::prelude::*;
use std::{env, fs, io, path::Path, process};

mod memory;

mod funcs;
mod parser;
use memory::Memory;
// use funcs::{errors, jump, operations, print, stack, stdfn, var};
use funcs::stack;

fn main() -> std::io::Result<()> {
    let mut filename = String::new();
    if env::args().nth(1).is_none() {
        println!("Welcome to the Ram stack-based programming language.");
        println!("Please enter a filename: ");
        io::stdin().read_line(&mut filename)?;
        filename = filename.trim().to_string();
    } else {
        if env::args().nth(1).unwrap() == "test" {
            if Path::new("log.txt").exists() {
                fs::remove_file("log.txt").expect("");
                panic!("Tests failed");
            } else {
                process::exit(0);
            }
        } else {
            filename = env::args().nth(1).unwrap();
        }
    }

    let mut file = fs::File::open(filename.trim())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let p_lines: Vec<&str> = contents.split("\n").collect();
    let program: HashMap<String, Vec<String>> = parser::parse_lines(p_lines); // returns final file with imports

    let mut memory: Memory = Memory::new();

    match execute_block(
        program,
        "main:",
        &mut memory,
        // vars, &mut hash_vars
    ) {
        Ok(()) => (),
        _ => println!("Something went wrong"),
    }
    Ok(())
}

pub fn execute_block(
    program: HashMap<String, Vec<String>>,
    run_label: &str,
    mut memory: &mut Memory,
) -> std::io::Result<()> {
    let mut line = 0u32;

    let run_block: Vec<String>;
    match program.get(run_label) {
        Some(value) => run_block = value.to_vec(),
        _ => {
            println!("label '{}' not found", run_label);
            process::exit(1);
        }
    }

    for statement in run_block {
        let statement = statement.trim();
        line += 1;
        let cmd: Vec<&str> = statement.split_whitespace().collect();
        match cmd[0].trim() {
            // "print" => print::print(&mut stack, cmd, &mut local_vars, hash_vars, run_label, line),
            // "printc" => print::printc(cmd, statement, run_label, line),
            "ram" => stack::ram(&mut memory, cmd, statement, run_label, line),
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
