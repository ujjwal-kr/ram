use crate::funcs::stack;
use crate::{memory::Memory, types::Vars, CPU};

// vec x push y
// vec x > [1] = y
// vec x len
// vex y = x > [1]

// push

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

// return item by index

#[test]
fn vec_int_idx_to_lx() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    vars.set_int_vec("x".to_string(), "[1, 2]", &mut memory).unwrap();
    let statement = "vec x > [1] = lx";
    let cmd: Vec<&str> = statement.split_whitespace().collect();
    stack::vec(&mut memory, &mut vars, &mut registers, cmd, statement).unwrap();
    assert_eq!(registers.lx, 2);
}

#[test]
fn vec_int_idx_to_rv() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    vars.set_int_vec("x".to_string(), "[1, 2]", &mut memory).unwrap();
    let statement = "vec x > [1] = rv";
    let cmd: Vec<&str> = statement.split_whitespace().collect();
    stack::vec(&mut memory, &mut vars, &mut registers, cmd, statement).unwrap();
    assert_eq!(registers.rv, 2);
}

#[test]
fn vec_int_idx_to_var() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    vars.set_int_vec("x".to_string(), "[1, 2]", &mut memory).unwrap();
    vars.set_int("y".to_string(), "0", &mut memory).unwrap();
    let statement = "vec x > [1] = y";
    let cmd: Vec<&str> = statement.split_whitespace().collect();
    stack::vec(&mut memory, &mut vars, &mut registers, cmd, statement).unwrap();
    let t = vars.get_type("y".to_string()).unwrap();
    assert_eq!(memory.yeild_i32(t.location), 2);
}


#[test]
fn vec_int_idx_get_lx_to_rv() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    vars.set_int_vec("x".to_string(), "[1, 2]", &mut memory).unwrap();
    registers.lx = 1;
    let statement = "vec x > [lx] = rv";
    let cmd: Vec<&str> = statement.split_whitespace().collect();
    stack::vec(&mut memory, &mut vars, &mut registers, cmd, statement).unwrap();
    assert_eq!(registers.rv, 2);
}

#[test]
fn vec_int_idx_get_rv_to_lx() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    vars.set_int_vec("x".to_string(), "[1, 2]", &mut memory).unwrap();
    registers.rv = 1;
    let statement = "vec x > [rv] = lx";
    let cmd: Vec<&str> = statement.split_whitespace().collect();
    stack::vec(&mut memory, &mut vars, &mut registers, cmd, statement).unwrap();
    assert_eq!(registers.lx, 2);
}

#[test]
fn vec_int_idx_lx_to_var() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    vars.set_int_vec("x".to_string(), "[1, 2]", &mut memory).unwrap();
    vars.set_int("y".to_string(), "0", &mut memory).unwrap();
    let statement = "vec x > [lx] = y";
    let cmd: Vec<&str> = statement.split_whitespace().collect();
    stack::vec(&mut memory, &mut vars, &mut registers, cmd, statement).unwrap();
    let t = vars.get_type("y".to_string()).unwrap();
    assert_eq!(memory.yeild_i32(t.location), 1);
}

#[test]
fn vec_int_idx_var_to_var() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    vars.set_int_vec("x".to_string(), "[1, 2]", &mut memory).unwrap();
    vars.set_int("y".to_string(), "0", &mut memory).unwrap();
    vars.set_int("z".to_string(), "1", &mut memory).unwrap();
    registers.lx = 1;
    let statement = "vec x > [z] = y";
    let cmd: Vec<&str> = statement.split_whitespace().collect();
    stack::vec(&mut memory, &mut vars, &mut registers, cmd, statement).unwrap();
    let t = vars.get_type("y".to_string()).unwrap();
    assert_eq!(memory.yeild_i32(t.location), 2);
}

#[test]
fn vec_int_idx_var_to_lx() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    vars.set_int_vec("x".to_string(), "[1, 2]", &mut memory).unwrap();
    vars.set_int("z".to_string(), "1", &mut memory).unwrap();
    let statement = "vec x > [z] = lx";
    let cmd: Vec<&str> = statement.split_whitespace().collect();
    stack::vec(&mut memory, &mut vars, &mut registers, cmd, statement).unwrap();
    assert_eq!(registers.lx, 2);
}


#[test]
fn vec_int_idx_var_to_rv() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    vars.set_int_vec("x".to_string(), "[1, 2]", &mut memory).unwrap();
    vars.set_int("z".to_string(), "1", &mut memory).unwrap();
    let statement = "vec x > [z] = rv";
    let cmd: Vec<&str> = statement.split_whitespace().collect();
    stack::vec(&mut memory, &mut vars, &mut registers, cmd, statement).unwrap();
    assert_eq!(registers.rv, 2);
}



#[test]
fn vec_str_idx_to_lxstring() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    vars.set_str_vec("x".to_string(), "['ok', 'omk']", &mut memory);
    let statement = "vec x > [1] = lxstring";
    let cmd: Vec<&str> = statement.split_whitespace().collect();
    stack::vec(&mut memory, &mut vars, &mut registers, cmd, statement).unwrap();
    assert_eq!(registers.lxstring, "omk".to_string());
}

#[test]
fn vec_str_idx_lx_to_lxstring() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    registers.lx = 1;
    vars.set_str_vec("x".to_string(), "['ok', 'omk']", &mut memory);
    let statement = "vec x > [lx] = lxstring";
    let cmd: Vec<&str> = statement.split_whitespace().collect();
    stack::vec(&mut memory, &mut vars, &mut registers, cmd, statement).unwrap();
    assert_eq!(registers.lxstring, "omk".to_string());
}


#[test]
fn vec_str_idx_lx_to_string() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    registers.lx = 1;
    vars.set_str_vec("x".to_string(), "['ok', 'omk']", &mut memory);
    let statement = "vec x > [lx] = string";
    let cmd: Vec<&str> = statement.split_whitespace().collect();
    stack::vec(&mut memory, &mut vars, &mut registers, cmd, statement).unwrap();
    assert_eq!(registers.string, "omk".to_string());
}

#[test]
fn vec_str_idx_to_string() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    vars.set_str_vec("x".to_string(), "['ok', 'omk']", &mut memory);
    let statement = "vec x > [1] = string";
    let cmd: Vec<&str> = statement.split_whitespace().collect();
    stack::vec(&mut memory, &mut vars, &mut registers, cmd, statement).unwrap();
    assert_eq!(registers.string, "omk".to_string());
}

#[test]
fn vec_str_idx_to_var() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();
    vars.set_string("y".to_string(), "''", &mut memory);
    vars.set_str_vec("x".to_string(), "['ok', 'omk']", &mut memory);
    let statement = "vec x > [1] = y";
    let cmd: Vec<&str> = statement.split_whitespace().collect();
    stack::vec(&mut memory, &mut vars, &mut registers, cmd, statement).unwrap();
    let t = vars.get_type("y".to_string()).unwrap();
    assert_eq!(memory.yeild_string(t.location), "omk".to_string());
}

// get length

#[test]
fn vec_str_len() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    vars.set_str_vec("x".to_string(), "['ok', 'omk']", &mut memory);
    let statement = "vec x len";
    let cmd: Vec<&str> = statement.split_whitespace().collect();
    stack::vec(&mut memory, &mut vars, &mut registers, cmd, statement).unwrap();
    assert_eq!(memory.get_int_from_stack().unwrap(), 2);
}

// vec mod