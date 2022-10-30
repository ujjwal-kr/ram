use crate::funcs::stack;
use crate::{memory::Memory, types::Vars, Registers};

#[test]
fn ram_10() {
    let mut memory: Memory = Memory::new();
    let mut registers: Registers = Registers::new();
    let mut vars: Vars = Vars::new();

    let statement = "ram 10";
    let cmd: Vec<&str> = statement.split_whitespace().collect();
    stack::ram(
        &mut memory,
        &mut vars,
        &mut registers,
        cmd,
        statement,
        "main",
        1,
    );
    assert_eq!(memory.get_int_from_stack("main", 1), 10)
}

#[test]
fn ram_lx_10() {
    let mut memory: Memory = Memory::new();
    let mut registers: Registers = Registers::new();
    let mut vars: Vars = Vars::new();

    let statement = "ram lx 10";
    let cmd: Vec<&str> = statement.split_whitespace().collect();
    stack::ram(
        &mut memory,
        &mut vars,
        &mut registers,
        cmd,
        statement,
        "main",
        1,
    );

    assert_eq!(registers.lx, 10)
}

#[test]
fn ram_rv_10() {
    let mut memory: Memory = Memory::new();
    let mut registers: Registers = Registers::new();
    let mut vars: Vars = Vars::new();

    let statement = "ram rv 10";
    let cmd: Vec<&str> = statement.split_whitespace().collect();
    stack::ram(
        &mut memory,
        &mut vars,
        &mut registers,
        cmd,
        statement,
        "main",
        1,
    );

    assert_eq!(registers.rv, 10)
}

#[test]
fn ram_lx_prev() {
    let mut memory: Memory = Memory::new();
    let mut registers: Registers = Registers::new();
    let mut vars: Vars = Vars::new();

    vars.set_int_to_stack(&mut memory, "10", "main", 1);
    let statement = "ram lx prev";
    let cmd: Vec<&str> = statement.split_whitespace().collect();
    stack::ram(
        &mut memory,
        &mut vars,
        &mut registers,
        cmd,
        statement,
        "main",
        1,
    );

    assert_eq!(registers.lx, 10)
}

#[test]
fn ram_rv_prev() {
    let mut memory: Memory = Memory::new();
    let mut registers: Registers = Registers::new();
    let mut vars: Vars = Vars::new();

    vars.set_int_to_stack(&mut memory, "10", "main", 1);
    let statement = "ram rv prev";
    let cmd: Vec<&str> = statement.split_whitespace().collect();
    stack::ram(
        &mut memory,
        &mut vars,
        &mut registers,
        cmd,
        statement,
        "main",
        1,
    );

    assert_eq!(registers.rv, 10)
}

#[test]
fn ram_lx() {
    let mut memory: Memory = Memory::new();
    let mut registers: Registers = Registers::new();
    let mut vars: Vars = Vars::new();

    registers.lx = 10;
    let statement = "ram lx";
    let cmd: Vec<&str> = statement.split_whitespace().collect();
    stack::ram(
        &mut memory,
        &mut vars,
        &mut registers,
        cmd,
        statement,
        "main",
        1,
    );

    assert_eq!(memory.get_int_from_stack("main", 1), registers.lx)
}

#[test]
fn ram_rv() {
    let mut memory: Memory = Memory::new();
    let mut registers: Registers = Registers::new();
    let mut vars: Vars = Vars::new();

    registers.rv = 10;
    let statement = "ram rv";
    let cmd: Vec<&str> = statement.split_whitespace().collect();
    stack::ram(
        &mut memory,
        &mut vars,
        &mut registers,
        cmd,
        statement,
        "main",
        1,
    );

    assert_eq!(memory.get_int_from_stack("main", 1), registers.rv)
}

#[test]
fn ram_string_eq_hellow() {
    let mut memory: Memory = Memory::new();
    let mut registers: Registers = Registers::new();
    let mut vars: Vars = Vars::new();

    registers.rv = 10;
    let statement = "ram string = hello";
    let cmd: Vec<&str> = statement.split_whitespace().collect();
    stack::ram(
        &mut memory,
        &mut vars,
        &mut registers,
        cmd,
        statement,
        "main",
        1,
    );

    assert_eq!(registers.string, "hello".to_string())
}

#[test]
fn ram_lxstring_eq_hellow() {
    let mut memory: Memory = Memory::new();
    let mut registers: Registers = Registers::new();
    let mut vars: Vars = Vars::new();

    registers.rv = 10;
    let statement = "ram lxstring = hello";
    let cmd: Vec<&str> = statement.split_whitespace().collect();
    stack::ram(
        &mut memory,
        &mut vars,
        &mut registers,
        cmd,
        statement,
        "main",
        1,
    );

    assert_eq!(registers.lxstring, "hello".to_string())
}

#[test]
fn ram_var_int() {
    let mut memory: Memory = Memory::new();
    let mut registers: Registers = Registers::new();
    let mut vars: Vars = Vars::new();

    let statement = "ram x :int = 5";
    let cmd: Vec<&str> = statement.split_whitespace().collect();
    stack::ram(
        &mut memory,
        &mut vars,
        &mut registers,
        cmd,
        statement,
        "main",
        1,
    );

    let t = vars.get_type("x".to_string(), "main", 1);
    assert_eq!(memory.yeild_i32(t.location), 5)
}

#[test]
fn ram_var_str() {
    let mut memory: Memory = Memory::new();
    let mut registers: Registers = Registers::new();
    let mut vars: Vars = Vars::new();

    let statement = "ram x :str = meow";
    let cmd: Vec<&str> = statement.split_whitespace().collect();
    stack::ram(
        &mut memory,
        &mut vars,
        &mut registers,
        cmd,
        statement,
        "main",
        1,
    );

    let t = vars.get_type("x".to_string(), "main", 1);
    assert_eq!(memory.yeild_string(t.location), "meow".to_string())
}

#[test]
fn ram_vec_int() {
    let mut memory: Memory = Memory::new();
    let mut registers: Registers = Registers::new();
    let mut vars: Vars = Vars::new();

    let statement = "ram x :vec :int = [1, 2]";
    let cmd: Vec<&str> = statement.split_whitespace().collect();
    stack::ram(
        &mut memory,
        &mut vars,
        &mut registers,
        cmd,
        statement,
        "main",
        1,
    );

    let t = vars.get_type("x".to_string(), "main", 1);
    assert_eq!(memory.yeild_int_vec(t.location), [1, 2].to_vec())
}

#[test]
fn ram_vec_str() {
    let mut memory: Memory = Memory::new();
    let mut registers: Registers = Registers::new();
    let mut vars: Vars = Vars::new();

    let statement = "ram x :vec :str = [ok, meow]";
    let cmd: Vec<&str> = statement.split_whitespace().collect();
    stack::ram(
        &mut memory,
        &mut vars,
        &mut registers,
        cmd,
        statement,
        "main",
        1,
    );

    let t = vars.get_type("x".to_string(), "main", 1);
    assert_eq!(
        memory.yeild_str_vec(t.location),
        ["ok".to_string(), "meow".to_string()].to_vec()
    )
}
