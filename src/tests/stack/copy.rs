use crate::funcs::stack::copy;
use crate::{memory::Memory, types::Vars, CPU};

// copy lx = rv
// copy lx = var

// copy rv = lx
// copy rv = var

// copy string = lxstring
// copy string = var

// copy lxstring = string
// copy lxstring = var

// copy var = lx
// copy var = rv
// copy var = string
// copy var = lxstring
// copy var1 = var2

#[test]
fn copy_lx_rv() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    registers.rv = 5;
    let statement: &str = "copy lx = rv";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    copy(
        &mut memory,
        &mut vars,
        &mut registers,
        cmd,
        statement,
        "main:",
        1,
    );
    assert_eq!(registers.rv, registers.lx)
}

#[test]
fn copy_lx_var() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    registers.lx = 5;
    vars.set_int("x".to_string(), "1", &mut memory, "main", 1);

    let statement: &str = "copy lx = x";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    copy(
        &mut memory,
        &mut vars,
        &mut registers,
        cmd,
        statement,
        "main:",
        1,
    );
    assert_eq!(registers.lx, 1)
}

#[test]
fn copy_rv_lx() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    registers.lx = 5;
    let statement: &str = "copy rv = lx";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    copy(
        &mut memory,
        &mut vars,
        &mut registers,
        cmd,
        statement,
        "main:",
        1,
    );
    assert_eq!(registers.rv, registers.lx)
}

#[test]
fn copy_rv_var() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    registers.rv = 5;
    vars.set_int("x".to_string(), "1", &mut memory, "main", 1);

    let statement: &str = "copy rv = x";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    copy(
        &mut memory,
        &mut vars,
        &mut registers,
        cmd,
        statement,
        "main:",
        1,
    );
    assert_eq!(registers.rv, 1)
}

#[test]
fn copy_string_lxstring() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    registers.lxstring = "hello".to_string();

    let statement: &str = "copy string = lxstring";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    copy(
        &mut memory,
        &mut vars,
        &mut registers,
        cmd,
        statement,
        "main:",
        1,
    );
    assert_eq!(registers.string, registers.lxstring)
}

#[test]
fn copy_string_var() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    vars.set_string("x".to_string(), "hello", &mut memory);

    let statement: &str = "copy string = x";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    copy(
        &mut memory,
        &mut vars,
        &mut registers,
        cmd,
        statement,
        "main:",
        1,
    );
    assert_eq!(registers.string, "hello".to_string())
}

#[test]
fn copy_lxstring_string() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    registers.string = "hello".to_string();

    let statement: &str = "copy lxstring = string";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    copy(
        &mut memory,
        &mut vars,
        &mut registers,
        cmd,
        statement,
        "main:",
        1,
    );
    assert_eq!(registers.string, registers.lxstring)
}

#[test]
fn copy_lxstring_var() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    vars.set_string("x".to_string(), "hello", &mut memory);

    let statement: &str = "copy lxstring = x";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    copy(
        &mut memory,
        &mut vars,
        &mut registers,
        cmd,
        statement,
        "main:",
        1,
    );
    assert_eq!(registers.lxstring, "hello".to_string())
}

#[test]
fn copy_var_lx() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    vars.set_int("x".to_string(), "0", &mut memory, "main:", 1);
    registers.lx = 1;

    let statement: &str = "copy x = lx";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    copy(
        &mut memory,
        &mut vars,
        &mut registers,
        cmd,
        statement,
        "main:",
        1,
    );
    let t = vars.get_type("x".to_string(), "main", 1);

    assert_eq!(memory.yeild_i32(t.location), 1)
}

#[test]
fn copy_var_rv() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    vars.set_int("x".to_string(), "0", &mut memory, "main:", 1);
    registers.rv = 1;

    let statement: &str = "copy x = rv";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    copy(
        &mut memory,
        &mut vars,
        &mut registers,
        cmd,
        statement,
        "main:",
        1,
    );
    let t = vars.get_type("x".to_string(), "main", 1);

    assert_eq!(memory.yeild_i32(t.location), 1)
}

#[test]
fn copy_var_string() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    vars.set_string("x".to_string(), "", &mut memory);
    registers.string = "hello".to_string();

    let statement: &str = "copy x = string";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    copy(
        &mut memory,
        &mut vars,
        &mut registers,
        cmd,
        statement,
        "main:",
        1,
    );
    let t = vars.get_type("x".to_string(), "main", 1);

    assert_eq!(memory.yeild_string(t.location), "hello".to_string())
}

#[test]
fn copy_var_lxstring() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    vars.set_string("x".to_string(), "", &mut memory);
    registers.lxstring = "hello".to_string();

    let statement: &str = "copy x = lxstring";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    copy(
        &mut memory,
        &mut vars,
        &mut registers,
        cmd,
        statement,
        "main:",
        1,
    );
    let t = vars.get_type("x".to_string(), "main", 1);

    assert_eq!(memory.yeild_string(t.location), "hello".to_string())
}

#[test]
fn copy_var_var_int() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    vars.set_int("x".to_string(), "1", &mut memory, "main", 1);
    vars.set_int("y".to_string(), "0", &mut memory, "main", 1);

    let statement: &str = "copy y = x";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    copy(
        &mut memory,
        &mut vars,
        &mut registers,
        cmd,
        statement,
        "main:",
        1,
    );

    let t = vars.get_type("y".to_string(), "main", 1);
    assert_eq!(memory.yeild_i32(t.location), 1)
}

#[test]
fn copy_var_var_str() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    vars.set_string("x".to_string(), "hello", &mut memory);
    vars.set_string("y".to_string(), "", &mut memory);

    let statement: &str = "copy y = x";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    copy(
        &mut memory,
        &mut vars,
        &mut registers,
        cmd,
        statement,
        "main:",
        1,
    );

    let t = vars.get_type("y".to_string(), "main", 1);
    assert_eq!(memory.yeild_string(t.location), "hello".to_string())
}
