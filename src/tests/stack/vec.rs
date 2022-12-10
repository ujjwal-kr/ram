use crate::funcs::stack;
use crate::{memory::Memory, types::Vars, CPU};

// vec x push y
// vec x > [1] = y
// vec x len
// vex y = x > [1]

// int push

#[test]
fn vec_int_push_1() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    vars.set_int_vec("x".to_string(), "[1, 2]", &mut memory).unwrap();
    let statement = "vec x push 1";
    let cmd: Vec<&str> = statement.split_whitespace().collect();
    stack::vec(&mut memory, &mut vars, &mut registers, cmd, statement).unwrap();

    let t = vars.get_type("x".to_string()).unwrap();
    assert_eq!(memory.yeild_int_vec(t.location), [1,2,1].to_vec())
}

#[test]
fn vec_int_push_var() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    vars.set_int_vec("x".to_string(), "[1, 2]", &mut memory).unwrap();
    vars.set_int("y".to_string(), "1", &mut memory).unwrap();
    let statement = "vec x push y";
    let cmd: Vec<&str> = statement.split_whitespace().collect();
    stack::vec(&mut memory, &mut vars, &mut registers, cmd, statement).unwrap();

    let t = vars.get_type("x".to_string()).unwrap();
    assert_eq!(memory.yeild_int_vec(t.location), [1,2,1].to_vec())
}

#[test]
fn vec_int_push_lx() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    vars.set_int_vec("x".to_string(), "[1, 2]", &mut memory).unwrap();
    registers.lx = 1;
    let statement = "vec x push lx";
    let cmd: Vec<&str> = statement.split_whitespace().collect();
    stack::vec(&mut memory, &mut vars, &mut registers, cmd, statement).unwrap();

    let t = vars.get_type("x".to_string()).unwrap();
    assert_eq!(memory.yeild_int_vec(t.location), [1,2,1].to_vec())
}

#[test]
fn vec_int_push_rv() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    vars.set_int_vec("x".to_string(), "[1, 2]", &mut memory).unwrap();
    registers.rv = 1;
    let statement = "vec x push rv";
    let cmd: Vec<&str> = statement.split_whitespace().collect();
    stack::vec(&mut memory, &mut vars, &mut registers, cmd, statement).unwrap();

    let t = vars.get_type("x".to_string()).unwrap();
    assert_eq!(memory.yeild_int_vec(t.location), [1,2,1].to_vec())
}

// str push


#[test]
fn vec_str_push_string() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    vars.set_str_vec("x".to_string(), "['a']", &mut memory);
    registers.string = "k".to_string();
    let statement = "vec x push string";
    let cmd: Vec<&str> = statement.split_whitespace().collect();
    stack::vec(&mut memory, &mut vars, &mut registers, cmd, statement).unwrap();

    let t = vars.get_type("x".to_string()).unwrap();
    assert_eq!(memory.yeild_str_vec(t.location), ["a".to_string(), "k".to_string()].to_vec())
}

#[test]
fn vec_str_push_lxstring() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    vars.set_str_vec("x".to_string(), "['a']", &mut memory);
    registers.lxstring = "k".to_string();
    let statement = "vec x push lxstring";
    let cmd: Vec<&str> = statement.split_whitespace().collect();
    stack::vec(&mut memory, &mut vars, &mut registers, cmd, statement).unwrap();

    let t = vars.get_type("x".to_string()).unwrap();
    assert_eq!(memory.yeild_str_vec(t.location), ["a".to_string(), "k".to_string()].to_vec())
}

#[test]
fn vec_str_push_var() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    vars.set_str_vec("x".to_string(), "['a']", &mut memory);
    vars.set_string("y".to_string(), "k", &mut memory);
    let statement = "vec x push y";
    let cmd: Vec<&str> = statement.split_whitespace().collect();
    stack::vec(&mut memory, &mut vars, &mut registers, cmd, statement).unwrap();

    let t = vars.get_type("x".to_string()).unwrap();
    assert_eq!(memory.yeild_str_vec(t.location), ["a".to_string(), "k".to_string()].to_vec())
}