use crate::funcs::operations;
use crate::{memory::Memory, types::Vars, Registers};

// add
// add lx
// add rv
// add var
// add lx rv
// add var1 var2

#[test]
fn add() {
    let mut memory: Memory = Memory::new();
    let mut registers: Registers = Registers::new();
    let mut vars: Vars = Vars::new();

    memory.set_int_to_stack(1);
    memory.set_int_to_stack(2);

    let statement = "add";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    operations::add(
        &mut memory,
        &mut vars,
        &mut registers,
        cmd,
        statement,
        "main",
        1,
    );

    assert_eq!(memory.get_int_from_stack(), 3)
}

#[test]
fn add_lx() {
    let mut memory: Memory = Memory::new();
    let mut registers: Registers = Registers::new();
    let mut vars: Vars = Vars::new();

    memory.set_int_to_stack(1);
    registers.lx = 2;

    let statement = "add lx";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    operations::add(
        &mut memory,
        &mut vars,
        &mut registers,
        cmd,
        statement,
        "main",
        1,
    );

    assert_eq!(memory.get_int_from_stack(), 3)
}

#[test]
fn add_rv() {
    let mut memory: Memory = Memory::new();
    let mut registers: Registers = Registers::new();
    let mut vars: Vars = Vars::new();

    memory.set_int_to_stack(1);
    registers.rv = 2;

    let statement = "add rv";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    operations::add(
        &mut memory,
        &mut vars,
        &mut registers,
        cmd,
        statement,
        "main",
        1,
    );

    assert_eq!(memory.get_int_from_stack(), 3)
}

#[test]
fn add_var() {
    let mut memory: Memory = Memory::new();
    let mut registers: Registers = Registers::new();
    let mut vars: Vars = Vars::new();

    memory.set_int_to_stack(1);
    vars.set_int("x".to_string(), "2", &mut memory, "main", 1);

    let statement = "add x";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    operations::add(
        &mut memory,
        &mut vars,
        &mut registers,
        cmd,
        statement,
        "main",
        1,
    );

    assert_eq!(memory.get_int_from_stack(), 3)
}

#[test]
fn add_lx_rv() {
    let mut memory: Memory = Memory::new();
    let mut registers: Registers = Registers::new();
    let mut vars: Vars = Vars::new();

    registers.lx = 1;
    registers.rv = 2;

    let statement = "add lx rv";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    operations::add(
        &mut memory,
        &mut vars,
        &mut registers,
        cmd,
        statement,
        "main",
        1,
    );

    assert_eq!(memory.get_int_from_stack(), 3)
}

#[test]
fn add_var_var() {
    let mut memory: Memory = Memory::new();
    let mut registers: Registers = Registers::new();
    let mut vars: Vars = Vars::new();

    vars.set_int("x".to_string(), "1", &mut memory, "main", 1);
    vars.set_int("y".to_string(), "2", &mut memory, "main", 1);

    let statement = "add x y";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    operations::add(
        &mut memory,
        &mut vars,
        &mut registers,
        cmd,
        statement,
        "main",
        1,
    );

    assert_eq!(memory.get_int_from_stack(), 3)
}

#[test]
fn add_var_lx() {
    let mut memory: Memory = Memory::new();
    let mut registers: Registers = Registers::new();
    let mut vars: Vars = Vars::new();

    vars.set_int("x".to_string(), "1", &mut memory, "main", 1);
    registers.lx = 2;

    let statement = "add x lx";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    operations::add(
        &mut memory,
        &mut vars,
        &mut registers,
        cmd,
        statement,
        "main",
        1,
    );

    assert_eq!(memory.get_int_from_stack(), 3)
}

#[test]
fn add_var_rv() {
    let mut memory: Memory = Memory::new();
    let mut registers: Registers = Registers::new();
    let mut vars: Vars = Vars::new();

    vars.set_int("x".to_string(), "1", &mut memory, "main", 1);
    registers.rv = 2;

    let statement = "add x rv";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    operations::add(
        &mut memory,
        &mut vars,
        &mut registers,
        cmd,
        statement,
        "main",
        1,
    );

    assert_eq!(memory.get_int_from_stack(), 3)
}

#[test]
fn add_lx_var() {
    let mut memory: Memory = Memory::new();
    let mut registers: Registers = Registers::new();
    let mut vars: Vars = Vars::new();

    vars.set_int("x".to_string(), "1", &mut memory, "main", 1);
    registers.lx = 2;

    let statement = "add lx x";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    operations::add(
        &mut memory,
        &mut vars,
        &mut registers,
        cmd,
        statement,
        "main",
        1,
    );

    assert_eq!(memory.get_int_from_stack(), 3)
}

#[test]
fn add_rv_var() {
    let mut memory: Memory = Memory::new();
    let mut registers: Registers = Registers::new();
    let mut vars: Vars = Vars::new();

    vars.set_int("x".to_string(), "1", &mut memory, "main", 1);
    registers.rv = 2;

    let statement = "add rv x";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    operations::add(
        &mut memory,
        &mut vars,
        &mut registers,
        cmd,
        statement,
        "main",
        1,
    );

    assert_eq!(memory.get_int_from_stack(), 3)
}

#[test]
fn add_lx_lx() {
    let mut memory: Memory = Memory::new();
    let mut registers: Registers = Registers::new();
    let mut vars: Vars = Vars::new();

    registers.lx = 3;

    let statement = "add lx lx";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    operations::add(
        &mut memory,
        &mut vars,
        &mut registers,
        cmd,
        statement,
        "main",
        1,
    );

    assert_eq!(memory.get_int_from_stack(), 6)
}

#[test]
fn add_rv_rv() {
    let mut memory: Memory = Memory::new();
    let mut registers: Registers = Registers::new();
    let mut vars: Vars = Vars::new();

    registers.rv = 3;

    let statement = "add rv rv";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    operations::add(
        &mut memory,
        &mut vars,
        &mut registers,
        cmd,
        statement,
        "main",
        1,
    );

    assert_eq!(memory.get_int_from_stack(), 6)
}