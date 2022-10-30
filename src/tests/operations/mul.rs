use crate::funcs::operations;
use crate::{memory::Memory, types::Vars, Registers};

// mul
// mul lx
// mul rv
// mul var
// mul lx rv
// mul rv lx
// mul var1 var2
// mul var lx
// mul var rv
// mul rv var
// mul lx var

#[test]
fn mul() {
    let mut memory: Memory = Memory::new();
    let mut registers: Registers = Registers::new();
    let mut vars: Vars = Vars::new();

    memory.set_int_to_stack(2);
    memory.set_int_to_stack(2);

    let statement = "mul";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    operations::mul::mul(
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
fn mul_lx() {
    let mut memory: Memory = Memory::new();
    let mut registers: Registers = Registers::new();
    let mut vars: Vars = Vars::new();

    memory.set_int_to_stack(2);
    registers.lx = 2;

    let statement = "mul lx";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    operations::mul::mul(
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
fn mul_rv() {
    let mut memory: Memory = Memory::new();
    let mut registers: Registers = Registers::new();
    let mut vars: Vars = Vars::new();

    memory.set_int_to_stack(11);
    registers.rv = 2;

    let statement = "mul rv";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    operations::mul::mul(
        &mut memory,
        &mut vars,
        &mut registers,
        cmd,
        statement,
        "main",
        1,
    );

    assert_eq!(memory.get_int_from_stack("main", 1), 22)
}

#[test]
fn mul_var() {
    let mut memory: Memory = Memory::new();
    let mut registers: Registers = Registers::new();
    let mut vars: Vars = Vars::new();

    memory.set_int_to_stack(3);
    vars.set_int("x".to_string(), "2", &mut memory, "main", 1);

    let statement = "mul x";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    operations::mul::mul(
        &mut memory,
        &mut vars,
        &mut registers,
        cmd,
        statement,
        "main",
        1,
    );

    assert_eq!(memory.get_int_from_stack("main", 1), 6)
}

#[test]
fn mul_lx_rv() {
    let mut memory: Memory = Memory::new();
    let mut registers: Registers = Registers::new();
    let mut vars: Vars = Vars::new();

    registers.lx = 2;
    registers.rv = 2;

    let statement = "mul lx rv";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    operations::mul::mul(
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
fn mul_rv_lx() {
    let mut memory: Memory = Memory::new();
    let mut registers: Registers = Registers::new();
    let mut vars: Vars = Vars::new();

    registers.lx = 2;
    registers.rv = 2;

    let statement = "mul rv lx";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    operations::mul::mul(
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
fn mul_var_var() {
    let mut memory: Memory = Memory::new();
    let mut registers: Registers = Registers::new();
    let mut vars: Vars = Vars::new();

    vars.set_int("x".to_string(), "2", &mut memory, "main", 1);
    vars.set_int("y".to_string(), "2", &mut memory, "main", 1);

    let statement = "mul x y";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    operations::mul::mul(
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
fn mul_var_lx() {
    let mut memory: Memory = Memory::new();
    let mut registers: Registers = Registers::new();
    let mut vars: Vars = Vars::new();

    vars.set_int("x".to_string(), "2", &mut memory, "main", 1);
    registers.lx = 2;

    let statement = "mul x lx";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    operations::mul::mul(
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
fn mul_var_rv() {
    let mut memory: Memory = Memory::new();
    let mut registers: Registers = Registers::new();
    let mut vars: Vars = Vars::new();

    vars.set_int("x".to_string(), "2", &mut memory, "main", 1);
    registers.rv = 2;

    let statement = "mul x rv";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    operations::mul::mul(
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
fn mul_lx_var() {
    let mut memory: Memory = Memory::new();
    let mut registers: Registers = Registers::new();
    let mut vars: Vars = Vars::new();

    vars.set_int("x".to_string(), "2", &mut memory, "main", 1);
    registers.lx = 2;

    let statement = "mul lx x";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    operations::mul::mul(
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
fn mul_rv_var() {
    let mut memory: Memory = Memory::new();
    let mut registers: Registers = Registers::new();
    let mut vars: Vars = Vars::new();

    vars.set_int("x".to_string(), "1", &mut memory, "main", 1);
    registers.rv = 2;

    let statement = "mul rv x";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    operations::mul::mul(
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
fn mul_lx_lx() {
    let mut memory: Memory = Memory::new();
    let mut registers: Registers = Registers::new();
    let mut vars: Vars = Vars::new();

    registers.lx = 3;

    let statement = "mul lx lx";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    operations::mul::mul(
        &mut memory,
        &mut vars,
        &mut registers,
        cmd,
        statement,
        "main",
        1,
    );

    assert_eq!(memory.get_int_from_stack("main", 1), 9)
}

#[test]
fn mul_rv_rv() {
    let mut memory: Memory = Memory::new();
    let mut registers: Registers = Registers::new();
    let mut vars: Vars = Vars::new();

    registers.rv = 3;

    let statement = "mul rv rv";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    operations::mul::mul(
        &mut memory,
        &mut vars,
        &mut registers,
        cmd,
        statement,
        "main",
        1,
    );

    assert_eq!(memory.get_int_from_stack("main", 1), 9)
}
