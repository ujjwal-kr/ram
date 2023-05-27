use crate::funcs::stdf::parse::*;
use crate::{memory::Memory, types::Vars, CPU};

// parse lx :str
// parse rv :str

// parse string :int
// parse lxstring :int

// parse var :int
// parse var :str

#[test]
fn parse_lx() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    registers.lx = 5;
    let statement: &str = "parse lx :str";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    parse(&mut memory, &mut vars, &mut registers, cmd).unwrap();
    assert_eq!(registers.string, "5".to_string())
}

#[test]
fn parse_rv() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    registers.rv = 5;
    let statement: &str = "parse rv :str";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    parse(&mut memory, &mut vars, &mut registers, cmd).unwrap();
    assert_eq!(registers.string, "5".to_string())
}

#[test]
fn parse_string() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    registers.string = "10".to_string();
    let statement: &str = "parse string :int";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    parse(&mut memory, &mut vars, &mut registers, cmd).unwrap();
    assert_eq!(registers.lx, 10)
}

#[test]
fn parse_lxstring() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    registers.lxstring = "10".to_string();
    let statement: &str = "parse lxstring :int";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    parse(&mut memory, &mut vars, &mut registers, cmd).unwrap();
    assert_eq!(registers.lx, 10)
}

#[test]
fn parse_var_int() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    vars.set_string("x".to_string(), "10", &mut memory);
    let statement: &str = "parse x :int";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    parse(&mut memory, &mut vars, &mut registers, cmd).unwrap();
    assert_eq!(registers.lx, 10)
}

#[test]
#[should_panic]
fn parse_int_err() {
     let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    vars.set_string("x".to_string(), "ww", &mut memory);
    let statement: &str = "parse x :int";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    parse(&mut memory, &mut vars, &mut registers, cmd).unwrap();
}

#[test]
fn parse_var_str() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    vars.set_int("x".to_string(), "10", &mut memory).unwrap();
    let statement: &str = "parse x :str";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    parse(&mut memory, &mut vars, &mut registers, cmd).unwrap();
    assert_eq!(registers.string, "10".to_string())
}
