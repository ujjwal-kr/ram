use std::collections::HashMap;
use std::io::prelude::*;
use std::{env, f64, fs, io, path::Path, process};

mod funcs;
mod tests;
use funcs::{errors, operations, print, stack, stdfn, var};

#[derive(Clone)]
pub struct Vars {
    pub lx: f64,
    pub rv: f64,
    pub string: String,
    pub lxstring: String,
    pub str_vec: Vec<String>,
    pub num_vec: Vec<f64>,
}

#[derive(Clone)]
pub struct HashVars {
    pub hash_str: HashMap<String, String>,
    pub hash_int: HashMap<String, f64>,
    pub hash_str_vec: HashMap<String, Vec<String>>,
    pub hash_int_vec: HashMap<String, Vec<f64>>,
}

fn main() -> std::io::Result<()> {
    let mut filename = String::new();
    if env::args().nth(1).is_none() == true {
        println!("Welcome to the Ram stack-based programming language.");
        println!("Please enter a filename: ");
        io::stdin().read_line(&mut filename)?;
        filename = filename.trim().to_string();
    } else {
        if env::args().nth(1).unwrap() == "test" {
            tests::test();
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

    let mut _program: Vec<&str> = contents.split("\r\n\r\n").collect();
    let mut blocks: Vec<Vec<&str>> = vec![];
    for block in &_program {
        let block_vec: Vec<&str> = block.split("\n").collect();
        blocks.push(block_vec);
    }
    let vars = Vars {
        lx: 0.0,
        rv: 0.0,
        string: "".to_string(),
        lxstring: "".to_string(),
        num_vec: vec![],
        str_vec: vec![],
    };

    let hash_vars = HashVars {
        hash_str: HashMap::new(),
        hash_int: HashMap::new(),
        hash_int_vec: HashMap::new(),
        hash_str_vec: HashMap::new(),
    };
    match run_statement(&blocks, &blocks[0], 0, vars, hash_vars) {
        Ok(()) => (),
        _ => println!("Something went wrong"),
    }
    Ok(())
}

fn run_statement(
    blocks: &Vec<Vec<&str>>,
    run_block: &Vec<&str>,
    block_number: usize,
    vars: Vars,
    hash_vars: HashVars,
) -> std::io::Result<()> {
    let mut line = 0u32;
    let mut local_vars = Vars {
        lx: vars.lx,
        rv: vars.rv,
        string: vars.string,
        lxstring: vars.lxstring,
        num_vec: vars.num_vec,
        str_vec: vars.str_vec,
    };

    let mut local_hash_vars = HashVars {
        hash_str: hash_vars.hash_str,
        hash_int: hash_vars.hash_int,
        hash_int_vec: hash_vars.hash_int_vec,
        hash_str_vec: hash_vars.hash_str_vec,
    };

    let mut stack: Vec<f64> = vec![];
    for statement in run_block {
        line = line + 1;
        let mut cmd: Vec<&str> = statement.split(" ").collect();
        // comments
        let lits: Vec<&str> = cmd[0].trim().split("").collect();
        if lits[1] == "/" && lits[2] == "/" {
            cmd[0] = "//";
        }
        match cmd[0].trim() {
            "//" => (),
            "print" => print::print(
                &mut stack,
                cmd,
                &mut local_vars,
                &mut local_hash_vars,
                block_number,
                line,
            ),
            "printc" => print::printc(cmd, statement, block_number, line),
            "ram" => stack::ram(
                &mut stack,
                cmd,
                statement,
                &mut local_vars,
                &mut local_hash_vars,
                block_number,
                line,
            ),
            "var" => var::var(
                &mut stack,
                cmd,
                statement,
                &mut local_vars,
                &mut local_hash_vars,
                block_number,
                line,
            ),
            "move" => var::movefn(
                cmd,
                &mut local_vars,
                &mut local_hash_vars,
                block_number,
                line,
            ),
            "str" => stack::strfn(&mut stack, &mut local_vars, cmd, block_number, line),
            "stdin" => stdfn::stdin(&mut local_vars, cmd, block_number, line),
            "stdfs" => stdfn::stdfs(&mut local_vars, cmd, statement, block_number, line),
            "pop" => stack::pop(&mut stack, block_number, line),
            "popall" => stack = vec![],
            "add" => operations::add(&mut stack, cmd, &mut local_vars, block_number, line),
            "sub" => operations::sub(&mut stack, block_number, line),
            "mul" => operations::mul(&mut stack, cmd, &mut local_vars, block_number, line),
            "div" => operations::div(&mut stack, block_number, line),
            "sqr" => operations::sqr(&mut stack, cmd, &mut local_vars, block_number, line),
            "sqrt" => operations::sqrt(&mut stack, cmd, &mut local_vars, block_number, line),
            "round" => operations::round(&mut stack, cmd, &mut local_vars, block_number, line),
            "avg" => operations::avg(&mut stack, block_number, line),
            "rand" => stdfn::random(
                &mut local_vars,
                cmd,
                &mut stack,
                statement,
                block_number,
                line,
            ),
            "split" => operations::split(cmd, statement, &mut local_vars, block_number, line),
            "parse" => stdfn::parse_int(&mut local_vars, cmd, block_number, line),
            "vec" => operations::vec_ops(
                &mut stack,
                cmd,
                statement,
                &mut local_vars,
                block_number,
                line,
            ),
            "cmp" => operations::cmp(&mut stack, block_number, line),

            "je" => {
                if cmd.len() != 2 {
                    errors::args_error(block_number, line);
                    break;
                }
                if stack[stack.len() - 1] == 0.0 {
                    let index: usize = errors::parse_usize(cmd[1].trim(), block_number, line);
                    if blocks.len() <= index {
                        errors::invalid_jmp(block_number, line, index);
                        break;
                    }
                    match run_statement(
                        blocks,
                        &blocks[index],
                        index,
                        local_vars.clone(),
                        local_hash_vars.clone(),
                    ) {
                        Ok(()) => (),
                        _ => println!("Something went wrong"),
                    }
                    stack.pop();
                }
                stack.pop();
            }

            "jne" => {
                if cmd.len() != 2 {
                    errors::args_error(block_number, line);
                    break;
                }
                if stack[stack.len() - 1] != 0.0 {
                    let index: usize = errors::parse_usize(cmd[1].trim(), block_number, line);
                    if blocks.len() <= index {
                        errors::invalid_jmp(block_number, line, index);
                        break;
                    }
                    match run_statement(
                        blocks,
                        &blocks[index],
                        index,
                        local_vars.clone(),
                        local_hash_vars.clone(),
                    ) {
                        Ok(()) => (),
                        _ => println!("Something went wrong"),
                    }
                    stack.pop();
                }
                stack.pop();
            }

            "jgr" => {
                if cmd.len() != 2 {
                    errors::args_error(block_number, line);
                    break;
                }
                if stack[stack.len() - 1] == 1.0 {
                    let index: usize = errors::parse_usize(cmd[1].trim(), block_number, line);
                    if blocks.len() <= index {
                        errors::invalid_jmp(block_number, line, index);
                        break;
                    }
                    match run_statement(
                        blocks,
                        &blocks[index],
                        index,
                        local_vars.clone(),
                        local_hash_vars.clone(),
                    ) {
                        Ok(()) => (),
                        _ => println!("Something went wrong"),
                    }
                    stack.pop();
                }
                stack.pop();
            }

            "jsm" => {
                if cmd.len() != 2 {
                    errors::args_error(block_number, line);
                    break;
                }
                if stack[stack.len() - 1] == -1.0 {
                    let index: usize = errors::parse_usize(cmd[1].trim(), block_number, line);
                    if blocks.len() <= index {
                        errors::invalid_jmp(block_number, line, index)
                    }
                    match run_statement(
                        blocks,
                        &blocks[index],
                        index,
                        local_vars.clone(),
                        local_hash_vars.clone(),
                    ) {
                        Ok(()) => (),
                        _ => println!("Something went wrong"),
                    }
                    stack.pop();
                }
                stack.pop();
            }

            "jmp" => {
                if cmd.len() != 2 {
                    errors::args_error(block_number, line);
                    break;
                }
                let index: usize = errors::parse_usize(cmd[1].trim(), block_number, line);
                if blocks.len() <= index {
                    errors::invalid_jmp(block_number, line, index);
                    break;
                }
                match run_statement(
                    blocks,
                    &blocks[index],
                    index,
                    local_vars.clone(),
                    local_hash_vars.clone(),
                ) {
                    Ok(()) => (),
                    _ => println!("Something went wrong"),
                }
            }
            "halt" => {
                process::exit(0);
            }
            _ => {
                println!(
                    "Cant recognize command '{}' at b{}:l{}",
                    cmd[0],
                    block_number.to_string(),
                    line.to_string()
                );
                break;
            }
        }
    }
    Ok(())
}
