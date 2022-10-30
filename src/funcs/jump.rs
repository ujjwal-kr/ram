use crate::{execute_block, Registers};
use crate::{memory::Memory, types::Vars};

use super::errors;
use std::collections::HashMap;

pub fn jmp(
    memory: &mut Memory,
    vars: &mut Vars,
    registers: Registers,
    program: HashMap<String, Vec<String>>,
    cmd: Vec<&str>,
    b: &str,
    l: i32,
) {
    if cmd.len() != 2 {
        errors::args_error(b, l);
    }
    let label = cmd[1].trim();
    execute_block(program, label, registers, memory, vars);
}

pub fn je(
    memory: &mut Memory,
    vars: &mut Vars,
    registers: Registers,
    program: HashMap<String, Vec<String>>,
    cmd: Vec<&str>,
    b: &str,
    l: i32,
) {
    if cmd.len() < 2 {
        errors::args_error(b, l)
    }
    let num = memory.get_int_from_stack(b, l);
    if num == 0 {
        let label = cmd[1].trim();
        for _ in 0..4 {
            memory.pop_stack()
        }
        execute_block(program, label, registers, memory, vars)
    }
}

pub fn jne(
    memory: &mut Memory,
    vars: &mut Vars,
    registers: Registers,
    program: HashMap<String, Vec<String>>,
    cmd: Vec<&str>,
    b: &str,
    l: i32,
) {
    if cmd.len() < 2 {
        errors::args_error(b, l)
    }
    let num = memory.get_int_from_stack(b, l);
    if num != 0 {
        let label = cmd[1].trim();
        for _ in 0..4 {
            memory.pop_stack()
        }
        execute_block(program, label, registers, memory, vars)
    }
}

pub fn jgr(
    memory: &mut Memory,
    vars: &mut Vars,
    registers: Registers,
    program: HashMap<String, Vec<String>>,
    cmd: Vec<&str>,
    b: &str,
    l: i32,
) {
    if cmd.len() < 2 {
        errors::args_error(b, l)
    }
    let num = memory.get_int_from_stack(b, l);
    if num == 1 {
        let label = cmd[1].trim();
        for _ in 0..4 {
            memory.pop_stack()
        }
        execute_block(program, label, registers, memory, vars)
    }
}

pub fn jsm(
    memory: &mut Memory,
    vars: &mut Vars,
    registers: Registers,
    program: HashMap<String, Vec<String>>,
    cmd: Vec<&str>,
    b: &str,
    l: i32,
) {
    if cmd.len() < 2 {
        errors::args_error(b, l)
    }
    let num = memory.get_int_from_stack(b, l);
    if num == -1 {
        let label = cmd[1].trim();
        for _ in 0..4 {
            memory.pop_stack()
        }
        execute_block(program, label, registers, memory, vars)
    }
}
