use crate::funcs::operations;
use crate::{memory::Memory, types::Vars, Registers};

// div
// div lx
// div rv
// div var
// div lx rv
// div rv lx
// div var1 var2
// div var lx
// div var rv
// div rv var
// div lx var

#[test]
fn div() {
    let mut memory: Memory = Memory::new();
    let mut registers: Registers = Registers::new();
    let mut vars: Vars = Vars::new();

    memory.set_int_to_stack(10);
    memory.set_int_to_stack(2);

    let statement = "div";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    operations::div::div(
        &mut memory,
        &mut vars,
        &mut registers,
        cmd,
        statement,
        "main",
        1,
    );

    assert_eq!(memory.get_int_from_stack("main", 1), 5)
}

#[test]
fn div_lx() {
    let mut memory: Memory = Memory::new();
    let mut registers: Registers = Registers::new();
    let mut vars: Vars = Vars::new();

    memory.set_int_to_stack(10);
    registers.lx = 2;

    let statement = "div lx";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    operations::div::div(
        &mut memory,
        &mut vars,
        &mut registers,
        cmd,
        statement,
        "main",
        1,
    );

    assert_eq!(memory.get_int_from_stack("main", 1), 5)
}

#[test]
fn div_rv() {
    let mut memory: Memory = Memory::new();
    let mut registers: Registers = Registers::new();
    let mut vars: Vars = Vars::new();

    memory.set_int_to_stack(20);
    registers.rv = 5;

    let statement = "div rv";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    operations::div::div(
        &mut memory,
        &mut vars,
        &mut registers,
        cmd,
        statement,
        "main",
        1,
    );

    assert_eq!(memory.get_int_from_stack("main", 1), 4)
}

#[test]
fn div_var() {
    let mut memory: Memory = Memory::new();
    let mut registers: Registers = Registers::new();
    let mut vars: Vars = Vars::new();

    vars.set_int("x".to_string(), "2", &mut memory, "main", 1);
    memory.set_int_to_stack(6);

    let statement = "div x";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    operations::div::div(
        &mut memory,
        &mut vars,
        &mut registers,
        cmd,
        statement,
        "main",
        1,
    );

    assert_eq!(memory.get_int_from_stack("main", 1), 3)
}

#[test]
fn div_lx_rv() {
    let mut memory: Memory = Memory::new();
    let mut registers: Registers = Registers::new();
    let mut vars: Vars = Vars::new();

    registers.lx = 2;
    registers.rv = 2;

    let statement = "div lx rv";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    operations::div::div(
        &mut memory,
        &mut vars,
        &mut registers,
        cmd,
        statement,
        "main",
        1,
    );

    assert_eq!(memory.get_int_from_stack("main", 1), 1)
}

#[test]
fn div_rv_lx() {
    let mut memory: Memory = Memory::new();
    let mut registers: Registers = Registers::new();
    let mut vars: Vars = Vars::new();

    registers.lx = 2;
    registers.rv = 2;

    let statement = "div rv lx";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    operations::div::div(
        &mut memory,
        &mut vars,
        &mut registers,
        cmd,
        statement,
        "main",
        1,
    );

    assert_eq!(memory.get_int_from_stack("main", 1), 1)
}

#[test]
fn div_var_var() {
    let mut memory: Memory = Memory::new();
    let mut registers: Registers = Registers::new();
    let mut vars: Vars = Vars::new();

    vars.set_int("x".to_string(), "2", &mut memory, "main", 1);
    vars.set_int("y".to_string(), "2", &mut memory, "main", 1);

    let statement = "div x y";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    operations::div::div(
        &mut memory,
        &mut vars,
        &mut registers,
        cmd,
        statement,
        "main",
        1,
    );

    assert_eq!(memory.get_int_from_stack("main", 1), 1)
}

#[test]
fn div_var_lx() {
    let mut memory: Memory = Memory::new();
    let mut registers: Registers = Registers::new();
    let mut vars: Vars = Vars::new();

    vars.set_int("x".to_string(), "2", &mut memory, "main", 1);
    registers.lx = 2;

    let statement = "div x lx";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    operations::div::div(
        &mut memory,
        &mut vars,
        &mut registers,
        cmd,
        statement,
        "main",
        1,
    );

    assert_eq!(memory.get_int_from_stack("main", 1), 1)
}

#[test]
fn div_var_rv() {
    let mut memory: Memory = Memory::new();
    let mut registers: Registers = Registers::new();
    let mut vars: Vars = Vars::new();

    vars.set_int("x".to_string(), "2", &mut memory, "main", 1);
    registers.rv = 2;

    let statement = "div x rv";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    operations::div::div(
        &mut memory,
        &mut vars,
        &mut registers,
        cmd,
        statement,
        "main",
        1,
    );

    assert_eq!(memory.get_int_from_stack("main", 1), 1)
}

#[test]
fn div_lx_var() {
    let mut memory: Memory = Memory::new();
    let mut registers: Registers = Registers::new();
    let mut vars: Vars = Vars::new();

    vars.set_int("x".to_string(), "2", &mut memory, "main", 1);
    registers.lx = 10;

    let statement = "div lx x";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    operations::div::div(
        &mut memory,
        &mut vars,
        &mut registers,
        cmd,
        statement,
        "main",
        1,
    );

    assert_eq!(memory.get_int_from_stack("main", 1), 5)
}

#[test]
fn div_rv_var() {
    let mut memory: Memory = Memory::new();
    let mut registers: Registers = Registers::new();
    let mut vars: Vars = Vars::new();

    vars.set_int("x".to_string(), "4", &mut memory, "main", 1);
    registers.rv = 8;

    let statement = "div rv x";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    operations::div::div(
        &mut memory,
        &mut vars,
        &mut registers,
        cmd,
        statement,
        "main",
        1,
    );

    assert_eq!(memory.get_int_from_stack("main", 1), 2)
}

#[test]
fn div_lx_lx() {
    let mut memory: Memory = Memory::new();
    let mut registers: Registers = Registers::new();
    let mut vars: Vars = Vars::new();

    registers.lx = 3;

    let statement = "div lx lx";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    operations::div::div(
        &mut memory,
        &mut vars,
        &mut registers,
        cmd,
        statement,
        "main",
        1,
    );

    assert_eq!(memory.get_int_from_stack("main", 1), 1)
}

#[test]
fn div_rv_rv() {
    let mut memory: Memory = Memory::new();
    let mut registers: Registers = Registers::new();
    let mut vars: Vars = Vars::new();

    registers.rv = 3;

    let statement = "div rv rv";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    operations::div::div(
        &mut memory,
        &mut vars,
        &mut registers,
        cmd,
        statement,
        "main",
        1,
    );

    assert_eq!(memory.get_int_from_stack("main", 1), 1)
}
