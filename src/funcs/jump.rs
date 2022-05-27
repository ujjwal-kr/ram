use super::super::{run_statement, HashVars, Vars};
use super::errors;
use std::collections::HashMap;

pub fn jmp(
    cmd: Vec<&str>,
    program: HashMap<String, Vec<String>>,
    local_vars: Vars,
    hash_vars: &mut HashVars,
    block_number: usize,
    line: u32,
) {
    if cmd.len() != 2 {
        errors::args_error(block_number, line);
    }
    let label = cmd[1].trim();
    match run_statement(program, label, local_vars.clone(), hash_vars) {
        Ok(()) => (),
        _ => println!("Something went wrong"),
    }
}

pub fn je(
    stack: &mut Vec<f64>,
    cmd: Vec<&str>,
    program: HashMap<String, Vec<String>>,
    local_vars: Vars,
    hash_vars: &mut HashVars,
    block_number: usize,
    line: u32,
) {
    if cmd.len() != 2 {
        errors::args_error(block_number, line);
    }
    if stack[stack.len() - 1] == 0.0 {
        let label = cmd[1].trim();
        match run_statement(program, label, local_vars.clone(), hash_vars) {
            Ok(()) => (),
            _ => println!("Something went wrong"),
        }
        stack.pop();
    }
    stack.pop();
}

pub fn jne(
    stack: &mut Vec<f64>,
    cmd: Vec<&str>,
    program: HashMap<String, Vec<String>>,
    local_vars: Vars,
    hash_vars: &mut HashVars,
    block_number: usize,
    line: u32,
) {
    if cmd.len() != 2 {
        errors::args_error(block_number, line);
    }
    if stack[stack.len() - 1] != 0.0 {
        let label = cmd[1].trim();
        match run_statement(program, label, local_vars.clone(), hash_vars) {
            Ok(()) => (),
            _ => println!("Something went wrong"),
        }
        stack.pop();
    }
    stack.pop();
}

pub fn jgr(
    stack: &mut Vec<f64>,
    cmd: Vec<&str>,
    program: HashMap<String, Vec<String>>,
    local_vars: Vars,
    hash_vars: &mut HashVars,
    block_number: usize,
    line: u32,
) {
    if cmd.len() != 2 {
        errors::args_error(block_number, line);
    }
    if stack[stack.len() - 1] == 1.0 {
        let label = cmd[1].trim();
        match run_statement(program, label, local_vars.clone(), hash_vars) {
            Ok(()) => (),
            _ => println!("Something went wrong"),
        }
        stack.pop();
    }
    stack.pop();
}

pub fn jsm(
    stack: &mut Vec<f64>,
    cmd: Vec<&str>,
    program: HashMap<String, Vec<String>>,
    local_vars: Vars,
    hash_vars: &mut HashVars,
    block_number: usize,
    line: u32,
) {
    if cmd.len() != 2 {
        errors::args_error(block_number, line);
    }
    if stack[stack.len() - 1] == -1.0 {
        let label = cmd[1].trim();
        match run_statement(program, label, local_vars.clone(), hash_vars) {
            Ok(()) => (),
            _ => println!("Something went wrong"),
        }
        stack.pop();
    }
    stack.pop();
}
