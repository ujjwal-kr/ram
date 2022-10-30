use std::result;

use crate::funcs::stack::copy;
use crate::{memory::Memory, types::Vars, Registers};

// copy lx = rv
// copy lx = var

// copy rv = lx
// copy rv = var

// copy string = lxstring
// copy string = var

// copy lxstring = string
// copy lxstring = var

#[test]
fn copy_lx_rv() {
    let mut memory: Memory = Memory::new();
    let mut registers: Registers = Registers::new();
    let mut vars: Vars = Vars::new();

    registers.lx = 5;
    let statement: &str = "copy lx = rv";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    copy(&mut memory, &mut vars, &mut registers, cmd, statement, "main:", 1);
    assert_eq!(registers.rv, registers.lx)
}

#[test]
fn copy_lx_var() {
    let mut memory: Memory = Memory::new();
    let mut registers: Registers = Registers::new();
    let mut vars: Vars = Vars::new();

    registers.lx = 5;
    vars.set_int("x".to_string(), "1", &mut memory, "main", 1);

    let statement: &str = "copy lx = x";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    copy(&mut memory, &mut vars, &mut registers, cmd, statement, "main:", 1);
    assert_eq!(registers.lx, 1)
}

#[test]
fn copy_rv_lx() {
    let mut memory: Memory = Memory::new();
    let mut registers: Registers = Registers::new();
    let mut vars: Vars = Vars::new();

    registers.lx = 5;
    let statement: &str = "copy rv = lx";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    copy(&mut memory, &mut vars, &mut registers, cmd, statement, "main:", 1);
    assert_eq!(registers.rv, registers.lx)
}

#[test]
fn copy_rv_var() {
    let mut memory: Memory = Memory::new();
    let mut registers: Registers = Registers::new();
    let mut vars: Vars = Vars::new();

    registers.rv = 5;
    vars.set_int("x".to_string(), "1", &mut memory, "main", 1);

    let statement: &str = "copy rv = x";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    copy(&mut memory, &mut vars, &mut registers, cmd, statement, "main:", 1);
    assert_eq!(registers.rv, 1)
}


#[test]
fn copy_string_lxstring() {
    let mut memory: Memory = Memory::new();
    let mut registers: Registers = Registers::new();
    let mut vars: Vars = Vars::new();

    registers.lxstring = "hello".to_string();

    let statement: &str = "copy string = lxstring";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    copy(&mut memory, &mut vars, &mut registers, cmd, statement, "main:", 1);
    assert_eq!(registers.string, registers.lxstring)
}

#[test]
fn copy_string_var() {
    let mut memory: Memory = Memory::new();
    let mut registers: Registers = Registers::new();
    let mut vars: Vars = Vars::new();

    vars.set_string("x".to_string(), "hello", &mut memory);

    let statement: &str = "copy string = x";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    copy(&mut memory, &mut vars, &mut registers, cmd, statement, "main:", 1);
    assert_eq!(registers.string, "hello".to_string())
}

#[test]
fn copy_lxstring_string() {
    let mut memory: Memory = Memory::new();
    let mut registers: Registers = Registers::new();
    let mut vars: Vars = Vars::new();

    registers.string = "hello".to_string();

    let statement: &str = "copy lxstring = string";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    copy(&mut memory, &mut vars, &mut registers, cmd, statement, "main:", 1);
    assert_eq!(registers.string, registers.lxstring)
}

#[test]
fn copy_lxstring_var() {
    let mut memory: Memory = Memory::new();
    let mut registers: Registers = Registers::new();
    let mut vars: Vars = Vars::new();

    vars.set_string("x".to_string(), "hello", &mut memory);

    let statement: &str = "copy lxstring = x";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    copy(&mut memory, &mut vars, &mut registers, cmd, statement, "main:", 1);
    assert_eq!(registers.lxstring, "hello".to_string())
}